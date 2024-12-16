pub mod instructions;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use crate::device::{DeviceResult, IOMappedDevice};
use bitflags::bitflags;
use proc_bitfield::bitfield;

use super::device::AddressMappedDevice;

type AddressMappedDevices = Vec<Arc<Mutex<dyn AddressMappedDevice>>>;
type IOMappedDevices = Vec<Arc<Mutex<dyn IOMappedDevice>>>;

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct Flag: u8 {
        const Z = 0b0000_0001; // Zero
        const N = 0b0000_0010; // Negative
        const C = 0b0000_0100; // Carry
        const O = 0b0000_1000; // Overflow
        const L = 0b0001_0000; // Less Than
        const G = 0b0010_0000; // Greater Than
        const I = 0b0100_0000; // Interrupt Disable

        const data = !0;
    }
}

bitfield! {
    #[derive(Debug, Default, Clone, Copy)]
    struct CPUMetadata(pub u16) {
        pub data: u16 @ ..,
        pub lo_byte: u8 @ 0..8,

        pub reg0: u8 @ 0..3,
        pub reg1: u8 @ 3..6,
        pub ext_addr: u8 @ 6..8,
        pub offset: u8 @ 8..16,
    }
}

#[derive(Debug)]
enum AddressingMode {
    Immediate,
    Register,
    Absolute,
}

#[derive(Debug, Default)]
enum CPUState {
    #[default]
    Reset,
    Fetch,
    Execute,
    Halt,
}

#[derive(Debug, Default)]
struct Instruction {
    pub opcode: u8,
    pub metadata: CPUMetadata,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StackAddress {
    value: u16,
}

#[allow(dead_code)]
impl StackAddress {
    pub fn new(address: u16) -> Self {
        Self {
            value: address & 0x3FF,
        }
    }

    fn set(&mut self, address: u16) {
        self.value = address & 0x3FF
    }

    fn increment(&mut self) {
        self.value += 1;
        self.value &= 0x3FF;
    }

    fn decrement(&mut self) {
        (self.value, _) = self.value.overflowing_sub(1);
        self.value &= 0x3FF;
    }

    fn offset(&mut self, offset: i8) {
        if offset < 0 {
            self.value -= offset.unsigned_abs() as u16;
        } else {
            self.value += offset as u16;
        }

        self.value &= 0x3FF;
    }

    fn get_offset(&mut self, offset: i8) -> StackAddress {
        let mut value = *self;

        if offset < 0 {
            value.value -= offset.unsigned_abs() as u16;
        } else {
            value.value += offset as u16;
        }

        value
    }
}

impl From<StackAddress> for u16 {
    fn from(val: StackAddress) -> Self {
        val.value & 0x3FF
    }
}

impl From<StackAddress> for ExtendedAddress {
    fn from(value: StackAddress) -> Self {
        ExtendedAddress::new_16bit_address(0xFC00 | value.value)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ExtendedAddress {
    value: u32,
}

#[allow(dead_code)]
impl ExtendedAddress {
    pub fn new_16bit_address(address: u16) -> Self {
        Self {
            value: address as u32,
        }
    }

    pub fn new_split_18bit_address(address: u16, extended: u8) -> Self {
        Self {
            value: (address as u32) | ((extended & 0x3) as u32) << 16,
        }
    }

    pub fn new_18bit_address(address: u32) -> Self {
        Self {
            value: address & 0x3FFFF,
        }
    }

    fn set_16bit_value(&mut self, address: u16) {
        self.value = address as u32
    }

    fn get_low_byte(&self) -> u8 {
        self.value as u8
    }

    fn set_low_byte(&mut self, byte: u8) {
        self.value &= 0x3FF00;
        self.value |= byte as u32
    }

    fn get_hi_byte(&self) -> u8 {
        ((self.value & 0xFF00) >> 8) as u8
    }

    fn set_hi_byte(&mut self, byte: u8) {
        self.value &= 0x300FF;
        self.value |= (byte as u32) << 8
    }

    fn set_18bit_value(&mut self, address: u32) {
        self.value = address & 0x3FFFF
    }

    fn get_extended_value(&self) -> u8 {
        ((self.value & 0x30000) >> 16) as u8
    }

    fn set_extended_value(&mut self, extended: u8) {
        self.value &= 0xFFFF;
        self.value |= ((extended & 0x3) as u32) << 16
    }

    fn increment(&mut self) {
        self.value += 1;
        self.value &= 0x3FFFF;
    }

    fn offset(&mut self, offset: i8) {
        if offset < 0 {
            self.value -= offset.unsigned_abs() as u32;
        } else {
            self.value += offset as u32;
        }
    }
}

impl From<ExtendedAddress> for u16 {
    fn from(val: ExtendedAddress) -> Self {
        (val.value & 0xFFFF) as u16
    }
}

impl From<ExtendedAddress> for u32 {
    fn from(val: ExtendedAddress) -> Self {
        val.value & 0x3FFFF
    }
}

impl Display for ExtendedAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#07x}", self.value)
    }
}

impl PartialEq<u32> for ExtendedAddress {
    fn eq(&self, other: &u32) -> bool {
        self.value == (other & 0x3FFFF)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ReadWrite {
    #[default]
    Read,
    Write,
}

#[derive(Debug, Default)]
pub struct Pins {
    pub address: ExtendedAddress,
    pub data: u8,
    pub rw: ReadWrite,
    pub bus_enable: bool,
    pub io_address: u8,
    pub io_data: u8,
    pub io_rw: ReadWrite,
    pub io_enable: bool
}

#[derive(Debug, Default)]
pub struct EmuOptions {
    flags: u8,
}

#[allow(dead_code)]
impl EmuOptions {
    pub fn new() -> Self {
        Self { flags: 0 }
    }

    pub fn new_value(flags: u8) -> Self {
        Self { flags }
    }

    fn exit_on_hlt(&self) -> bool {
        self.flags & 1 > 0
    }

    fn set_exit_on_hlt(&mut self) {
        self.flags |= 1;
    }

    fn unset_exit_on_hlt(&mut self) {
        self.flags &= 0b11111110;
    }
}

#[derive(Default)]
pub struct CPU {
    pub ra: u8,
    pub rb: u8,
    pub rc: u8,
    pub rd: u8,
    flags: Flag,
    pc: ExtendedAddress,
    sp: StackAddress,
    devices: AddressMappedDevices,
    io_devices: IOMappedDevices,
    instruction: Instruction,
    state: CPUState,
    cycle: u8,
    // temp8: u8,
    temp16: u16,
    temp_addr: ExtendedAddress,
    options: EmuOptions,
}

// Public
impl CPU {
    pub fn new(options: Option<EmuOptions>) -> Self {
        Self {
            cycle: 1,
            options: options.unwrap_or_default(),
            ..Default::default()
        }
    }

    pub fn add_device<T: AddressMappedDevice + 'static>(&mut self, device: T) {
        self.devices.push(Arc::new(Mutex::new(device)));
    }

    pub fn add_io_device<T: IOMappedDevice + 'static>(&mut self, device: T) {
        self.io_devices.push(Arc::new(Mutex::new(device)));
    }

    pub fn cycle(&mut self, pins: &mut Pins) {
        match self.state {
            CPUState::Reset => self.reset_handler(pins),
            CPUState::Fetch => self.fetch_handler(pins),
            CPUState::Execute => self.execute_handler(pins),
            CPUState::Halt => {
                if self.options.exit_on_hlt() {
                    std::process::exit(0)
                } else {
                    self.cycle -= 1;

                }
            }
        }

        self.cycle += 1
    }

    pub fn read(&self, address: ExtendedAddress) -> DeviceResult {
        for device in &self.devices {
            let res = device.lock().unwrap().read(address);

            if let DeviceResult::NotMyAddress = res {
                continue;
            }

            return res;
        }

        DeviceResult::NoValidDevice
    }

    pub fn write(&mut self, address: ExtendedAddress, data: u8) -> DeviceResult {
        for device in &self.devices {
            let res = device.lock().unwrap().write(address, data);

            if let DeviceResult::NotMyAddress = res {
                continue;
            }

            return res;
        }

        DeviceResult::NoValidDevice
    }

    pub fn read_io(&self, address: u8) -> DeviceResult {
        for device in &self.io_devices {
            let dev_unwrapped = device.lock().unwrap();

            if dev_unwrapped.address() != address {
                continue;
            }

            return dev_unwrapped.read();
        }

        DeviceResult::NoValidDevice
    }

    pub fn write_io(&mut self, address: u8, data: u8) -> DeviceResult {
        for device in &self.io_devices {
            let mut dev_unwrapped = device.lock().unwrap();

            if dev_unwrapped.address() != address {
                continue;
            }

            return dev_unwrapped.write(data);
        }

        DeviceResult::NoValidDevice
    }

    pub fn reset(&mut self, pins: &mut Pins) {
        self.ra = 0;
        self.rb = 0;
        self.rc = 0;
        self.rd = 0;

        self.pc.set_18bit_value(0);

        self.state = CPUState::Reset;
        self.cycle = 1;
        self.flags.0 .0 = 0;

        pins.address.set_16bit_value(0);
        pins.data = 0;
        pins.rw = ReadWrite::Read;
        pins.bus_enable = true;
        pins.io_address = 0;
        pins.io_data = 0;
        pins.rw = ReadWrite::Read;
        pins.io_enable = false;
    }
}

// Private
impl CPU {
    fn reset_handler(&mut self, pins: &mut Pins) {
        match self.cycle {
            1 => {
                pins.address.set_16bit_value(0x0000);
                pins.rw = ReadWrite::Read;
            }
            2 => {
                self.temp_addr.set_extended_value(pins.data);

                pins.address.set_16bit_value(0x0001);
                pins.rw = ReadWrite::Read;
            }
            3 => {
                self.temp_addr.set_hi_byte(pins.data);

                pins.address.set_16bit_value(0x0002);
                pins.rw = ReadWrite::Read;
            }
            4 => {
                self.temp_addr.set_low_byte(pins.data);

                self.pc = self.temp_addr;
                self.finish(pins);
            }
            _ => panic!("Tried to execute cycle {} for reset_handler", self.cycle),
        }
    }

    fn fetch_handler(&mut self, pins: &mut Pins) {
        match self.cycle {
            1 => {
                pins.address = self.pc;
                pins.rw = ReadWrite::Read;
                pins.bus_enable = true;
            }
            2 => {
                self.instruction.opcode = pins.data;
                self.pc.increment();
                pins.address = self.pc;
                pins.rw = ReadWrite::Read;
            }
            3 => {
                self.instruction.metadata.set_offset(pins.data);
                self.pc.increment();
                pins.address = self.pc;
                pins.rw = ReadWrite::Read;
            }
            4 => {
                self.instruction.metadata.set_lo_byte(pins.data);
                self.pc.increment();

                self.finish(pins);
                self.state = CPUState::Execute;
            }
            _ => panic!("Tried to execute cycle {} for fetch_handler", self.cycle),
        }
    }

    fn execute_handler(&mut self, pins: &mut Pins) {
        pins.bus_enable = true;
        match self.instruction.opcode {
            instructions::MOVI => self.MOV(pins, AddressingMode::Immediate),
            instructions::MOVR => self.MOV(pins, AddressingMode::Register),
            instructions::MOVA => self.MOV(pins, AddressingMode::Absolute),
            instructions::STA => self.ST(pins, AddressingMode::Absolute),
            instructions::ANDI => self.AND(pins, AddressingMode::Immediate),
            instructions::ANDR => self.AND(pins, AddressingMode::Register),
            instructions::ANDA => self.AND(pins, AddressingMode::Absolute),
            instructions::ORI => self.OR(pins, AddressingMode::Immediate),
            instructions::ORR => self.OR(pins, AddressingMode::Register),
            instructions::ORA => self.OR(pins, AddressingMode::Absolute),
            instructions::XORI => self.XOR(pins, AddressingMode::Immediate),
            instructions::XORR => self.XOR(pins, AddressingMode::Register),
            instructions::XORA => self.XOR(pins, AddressingMode::Absolute),
            instructions::PSHI => self.PSH(pins, AddressingMode::Immediate),
            instructions::PSHR => self.PSH(pins, AddressingMode::Register),
            instructions::POPR => self.POP(pins),
            instructions::HLT => self.HLT(pins),
            instructions::ADDI => self.ADD(pins, AddressingMode::Immediate),
            instructions::ADDR => self.ADD(pins, AddressingMode::Register),
            instructions::ADDA => self.ADD(pins, AddressingMode::Absolute),
            instructions::SUBI => self.SUB(pins, AddressingMode::Immediate),
            instructions::SUBR => self.SUB(pins, AddressingMode::Register),
            instructions::SUBA => self.SUB(pins, AddressingMode::Absolute),
            instructions::CMPI => self.CMP(pins, AddressingMode::Immediate),
            instructions::CMPR => self.CMP(pins, AddressingMode::Register),
            instructions::CMPA => self.CMP(pins, AddressingMode::Absolute),
            instructions::INCR => self.CMP(pins, AddressingMode::Register),
            instructions::INCA => self.CMP(pins, AddressingMode::Absolute),
            instructions::DECR => self.CMP(pins, AddressingMode::Register),
            instructions::DECA => self.CMP(pins, AddressingMode::Absolute),
            instructions::SBLR => self.SBL(pins, AddressingMode::Register),
            instructions::SBLA => self.SBL(pins, AddressingMode::Absolute),
            instructions::SBRR => self.SBR(pins, AddressingMode::Register),
            instructions::SBRA => self.SBR(pins, AddressingMode::Absolute),
            instructions::ROLR => self.ROL(pins, AddressingMode::Register),
            instructions::ROLA => self.ROL(pins, AddressingMode::Absolute),
            instructions::RORR => self.ROR(pins, AddressingMode::Register),
            instructions::RORA => self.ROR(pins, AddressingMode::Absolute),
            instructions::CLC => self.CLC(pins),
            instructions::CLI => self.CLI(pins),
            instructions::CLV => self.CLV(pins),
            instructions::SEI => self.SEI(pins),
            instructions::JMP => self.JMP(pins),
            instructions::JSR => self.JSR(pins),
            instructions::BIZ => self.BIZ(pins),
            instructions::BIN => self.BIN(pins),
            instructions::BIC => self.BIC(pins),
            instructions::BIO => self.BIO(pins),
            instructions::BIL => self.BIL(pins),
            instructions::BIG => self.BIG(pins),
            instructions::BNZ => self.BNZ(pins),
            instructions::BNN => self.BNN(pins),
            instructions::BNC => self.BNC(pins),
            instructions::BNO => self.BNO(pins),
            instructions::BNL => self.BNL(pins),
            instructions::BNG => self.BNG(pins),
            instructions::RTS => self.RTS(pins),
            instructions::IN => self.IN(pins),
            instructions::OUT => self.OUT(pins),
            _ => panic!("Unknown opcode: {:#04x}", self.instruction.opcode),
        }
    }

    fn finish(&mut self, pins: &mut Pins) {
        self.cycle = 0;
        pins.bus_enable = false;

        self.state = CPUState::Fetch;
    }

    fn decode_register(&mut self, mut reg: u8) -> &mut u8 {
        reg &= 0b111;

        match reg {
            0x0 => &mut self.ra,
            0x1 => &mut self.rb,
            0x2 => &mut self.rc,
            0x3 => &mut self.rd,
            0x4 => &mut self.flags.0 .0,
            _ => panic!("Invalid register #"),
        }
    }

    fn set_flag(&mut self, flag: Flag, value: u8, value2: Option<u8>) {
        match flag {
            Flag::Z => {
                if value == 0 {
                    self.flags.set(flag, true);
                }
            }
            Flag::N => {
                if value & 0x80 > 0 {
                    self.flags.set(flag, true);
                }
            }
            Flag::L => {
                if value < value2.unwrap() {
                    self.flags.set(flag, true);
                }
            }
            Flag::G => {
                if value > value2.unwrap() {
                    self.flags.set(flag, true);
                }
            }
            Flag::I => todo!(),
            _ => panic!("Unhandled flag value given"),
        }
    }

    fn jump_if_flag(&mut self, pins: &mut Pins, flag: Flag, inst_name: &str) {
        match self.cycle {
            1 => {
                self.temp_addr
                    .set_extended_value(self.instruction.metadata.ext_addr());
                pins.address = self.pc;
                pins.rw = ReadWrite::Read;
            }
            2 => {
                self.temp_addr.set_hi_byte(pins.data);
                self.pc.increment();

                pins.address = self.pc;
                pins.rw = ReadWrite::Read;
            }
            3 => {
                self.temp_addr.set_low_byte(pins.data);
                self.temp_addr
                    .offset(self.instruction.metadata.offset() as i8);
                
                if self.flags.contains(flag) {
                    self.pc = self.temp_addr;
                } else {
                    self.pc.increment();
                }

                self.finish(pins);
            },
            _ => panic!("{} tried to execute non-existent cycle {}", inst_name, self.cycle),
        }
    }

    fn jump_not_flag(&mut self, pins: &mut Pins, flag: Flag, inst_name: &str) {
        match self.cycle {
            1 => {
                self.temp_addr
                    .set_extended_value(self.instruction.metadata.ext_addr());
                pins.address = self.pc;
                pins.rw = ReadWrite::Read;
            }
            2 => {
                self.temp_addr.set_hi_byte(pins.data);
                self.pc.increment();

                pins.address = self.pc;
                pins.rw = ReadWrite::Read;
            }
            3 => {
                self.temp_addr.set_low_byte(pins.data);
                self.temp_addr
                    .offset(self.instruction.metadata.offset() as i8);
                
                if self.flags.contains(flag) {
                    self.pc.increment();
                } else {
                    self.pc = self.temp_addr;
                }

                self.finish(pins);
            },
            _ => panic!("{} tried to execute non-existent cycle {}", inst_name, self.cycle),
        }
    }
}
