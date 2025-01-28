#![allow(dead_code, reason = "Missing instructions which use these methods")]

pub mod instructions;
use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{device::{DeviceResult, IOMappedDevice}, info::InstructionInfoFile};
use bitflags::bitflags;
use proc_bitfield::bitfield;

use super::device::AddressMappedDevice;

type AddressMappedDevices = Vec<Rc<RefCell<dyn AddressMappedDevice>>>;
type IOMappedDevices = Vec<Rc<RefCell<dyn IOMappedDevice>>>;

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct Flag: u16 {
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

        pub reg0: u8 @ 0..4,
        pub reg1: u8 @ 4..8,
        pub offset: u8 @ 8..14,
        pub offset_sign: u8 @ 14..15, 
        pub reg_offset: u8 @ 15..16,
    }
}

#[derive(Debug)]
enum AddressingMode {
    Implied,
    Immediate,
    Register,
    Absolute,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum CPUState {
    #[default]
    Reset,
    Fetch,
    Execute,
    Halt,
    Interrupt,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum InterruptStatus {
    #[default]
    None,
    Normal,
    NonMaskable,
}

enum RegisterReturn<'a> {
    Full(&'a mut Register),
    Partial(&'a mut u8)
}

#[derive(Clone, Copy, Default)]
struct Register {
    low: u8,
    high: u8,
    word: u16
}

impl Register {
    pub fn get_low(&self) -> u8 {
        self.low
    }

    pub fn get_high(&self) -> u8 {
        self.high
    }

    pub fn get_word(&self) -> u16 {
        self.word
    }

    pub fn set_low(&mut self, value: u8) {
        self.low = value;
        self.update_word();
    }

    pub fn set_high(&mut self, value: u8) {
        self.high = value;
        self.update_word();
    }

    pub fn set_word(&mut self, value: u16) {
        self.low = value as u8;
        self.high = (value >> 8) as u8;
        self.update_word();
    }

    pub fn set_split_word(&mut self, high: u8, low: u8) {
        self.low = low;
        self.high = high;
        self.update_word();
    }

    fn get_mut_low(&mut self) -> &mut u8 {
        &mut self.low
    }

    fn get_mut_high(&mut self) -> &mut u8 {
        &mut self.high
    }

    fn update_word(&mut self) {
        self.word = ((self.high as u16) << 8) | self.low as u16;
    }
}

impl std::fmt::Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Register").field("data", &(((self.high as u16) << 8) | self.low as u16)).field("low", &self.low).field("high", &self.high).finish()
    }
}

#[derive(Debug, Default)]
struct Instruction {
    pub opcode: u8,
    pub metadata: CPUMetadata,
}

#[derive(Debug, Default)]
pub struct IRQ {
    pub req: bool,
    pub nmi: bool,
    pub ack: bool,
    pub data: u8, // Limited to 4 bits
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StackAddress {
    value: u16,
    page: u8
}

#[allow(dead_code)]
impl StackAddress {
    pub fn new(address: u16, page: Option<u8>) -> Self {
        Self {
            value: address,
            page: page.unwrap_or(1)
        }
    }

    fn set(&mut self, address: u16) {
        self.value = address;
    }

    fn increment(&mut self) {
        (self.value, _) = self.value.overflowing_add(1)
    }

    fn increment_amount(&mut self, amount: u16) {
        (self.value, _) = self.value.overflowing_add(amount)
    }

    fn decrement(&mut self) {
        (self.value, _) = self.value.overflowing_sub(1);
    }

    fn offset(&mut self, offset: i8) {
        if offset < 0 {
            self.value -= offset.unsigned_abs() as u16;
        } else {
            self.value += offset as u16;
        }
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

// impl From<StackAddress> for u16 {
//     fn from(val: StackAddress) -> Self {
//         val.value
//     }
// }

impl From<StackAddress> for u32 {
    fn from(val: StackAddress) -> Self {
        ((val.page as u32) << 16) | (val.value as u32)
    }
}

impl From<StackAddress> for ExtendedAddress {
    fn from(value: StackAddress) -> Self {
        ExtendedAddress::new_split_ext_address(value.value, value.page)
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

    pub fn new_split_ext_address(address: u16, extended: u8) -> Self {
        Self {
            value: (address as u32) | (extended as u32) << 16,
        }
    }

    pub fn new_ext_address(address: u32) -> Self {
        Self {
            value: address & 0xFFFFFF,
        }
    }

    pub fn get_16bit_value(&mut self) -> u16 {
        (self.value & 0xFFFF) as u16
    }

    pub fn set_16bit_value(&mut self, address: u16) {
        self.value = address as u32
    }

    // tbh idk what to call it
    pub fn set_seg_value(&mut self, address: u16) {
        self.value &= 0xFF0000;
        self.value |= address as u32;
    }

    pub fn get_low_byte(&self) -> u8 {
        self.value as u8
    }

    pub fn set_low_byte(&mut self, byte: u8) {
        self.value &= 0xFFFF00;
        self.value |= byte as u32
    }

    pub fn get_hi_byte(&self) -> u8 {
        ((self.value & 0xFF00) >> 8) as u8
    }

    pub fn set_hi_byte(&mut self, byte: u8) {
        self.value &= 0xFF00FF;
        self.value |= (byte as u32) << 8
    }

    pub fn set_18bit_value(&mut self, address: u32) {
        self.value = address & 0xFFFFFF
    }

    pub fn get_extended_value(&self) -> u8 {
        ((self.value & 0xFF0000) >> 16) as u8
    }

    pub fn set_extended_value(&mut self, extended: u8) {
        self.value &= 0xFFFF;
        self.value |= (extended as u32) << 16;
    }

    pub fn increment(&mut self) -> &mut Self {
        self.value += 1;
        self.value &= 0xFFFFFF;

        self
    }

    pub fn increment_amount(&mut self, amount: u8) -> &mut Self {
        self.value += amount as u32;
        self.value &= 0xFFFFFF;

        self
    }

    pub fn offset(&mut self, offset: i8) -> &mut Self {
        if offset.is_negative() {
            self.value -= offset.unsigned_abs() as u32;
        } else {
            self.value += offset as u32;
        }

        self
    }

    pub fn offset_word(&mut self, offset: i16) -> &mut Self {
        if offset.is_negative() {
            self.value -= offset.unsigned_abs() as u32;
        } else {
            self.value += offset as u32;
        }

        self
    }
}

impl From<ExtendedAddress> for u16 {
    fn from(val: ExtendedAddress) -> Self {
        (val.value & 0xFFFF) as u16
    }
}

impl From<ExtendedAddress> for u32 {
    fn from(val: ExtendedAddress) -> Self {
        val.value & 0xFFFFFF
    }
}

impl Display for ExtendedAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#08x}", self.value)
    }
}

impl PartialEq<u32> for ExtendedAddress {
    fn eq(&self, other: &u32) -> bool {
        self.value == (other & 0xFFFFFF)
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
    pub data: u16,
    pub rw: ReadWrite,
    pub bus_enable: bool,
    pub io_address: u8,
    pub io_data: u8,
    pub io_rw: ReadWrite,
    pub io_enable: bool,
    pub irq: IRQ,
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
    ra: Register,
    rb: Register,
    rc: Register,
    rd: Register,
    flags: Flag,
    pc: ExtendedAddress,
    sp: StackAddress,
    devices: AddressMappedDevices,
    io_devices: IOMappedDevices,
    instruction: Instruction,
    state: CPUState,
    int_status: InterruptStatus,
    int_num: u8,
    cycle: u8,
    temp16: u16,
    temp_addr: ExtendedAddress,
    options: EmuOptions,
    word: bool,
    inst_info: InstructionInfoFile
}

// Public
impl CPU {
    pub fn new(inst_info: InstructionInfoFile, options: Option<EmuOptions>) -> Self {
        Self {
            cycle: 1,
            options: options.unwrap_or_default(),
            inst_info,
            ..Default::default()
        }
    }

    pub fn add_device<T: AddressMappedDevice + 'static>(&mut self, device: T) {
        self.devices.push(Rc::new(RefCell::new(device)));
    }

    pub fn add_shared_device<T: AddressMappedDevice + 'static>(&mut self, device: Rc<RefCell<T>>) {
        self.devices.push(device);
    }

    pub fn add_io_device<T: IOMappedDevice + 'static>(&mut self, device: T) {
        self.io_devices.push(Rc::new(RefCell::new(device)));
    }

    pub fn add_shared_io_device<T: IOMappedDevice + 'static>(&mut self, device: Rc<RefCell<T>>) {
        self.io_devices.push(device);
    }

    pub fn cycle(&mut self, pins: &mut Pins) {
        if pins.irq.req && self.int_status == InterruptStatus::None {
            self.int_status = InterruptStatus::Normal;
        } else if pins.irq.nmi && self.int_status == InterruptStatus::None {
            self.int_status = InterruptStatus::NonMaskable;
        }

        match self.state {
            CPUState::Reset => self.reset_handler(pins),
            CPUState::Fetch => self.fetch_handler(pins),
            CPUState::Execute => self.execute_handler(pins),
            CPUState::Interrupt => self.interrupt_handler(pins),
            CPUState::Halt => {
                if self.options.exit_on_hlt() {
                    println!("Flags: {:?}", self.flags);
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
            let res = device.borrow_mut().read(address, self.word);

            if let DeviceResult::NotMyAddress = res {
                continue;
            }

            return res;
        }

        DeviceResult::NoValidDevice
    }

    pub fn write(&mut self, address: ExtendedAddress, data: u16) -> DeviceResult {
        for device in &self.devices {
            let res = device.borrow_mut().write(address, data, self.word);

            if let DeviceResult::NotMyAddress = res {
                continue;
            }

            return res;
        }

        DeviceResult::NoValidDevice
    }

    pub fn read_io(&self, address: u8) -> DeviceResult {
        for device in &self.io_devices {
            let mut dev_unwrapped = device.borrow_mut();

            if dev_unwrapped.io_address() != address {
                continue;
            }

            return dev_unwrapped.io_read();
        }

        DeviceResult::NoValidDevice
    }

    pub fn write_io(&mut self, address: u8, data: u8) -> DeviceResult {
        for device in &self.io_devices {
            let mut dev_unwrapped = device.borrow_mut();

            if dev_unwrapped.io_address() != address {
                continue;
            }

            return dev_unwrapped.io_write(data);
        }

        DeviceResult::NoValidDevice
    }

    pub fn reset(&mut self, pins: &mut Pins) {
        self.ra.set_word(0);
        self.rb.set_word(0);
        self.rc.set_word(0);
        self.rd.set_word(0);

        self.pc.set_18bit_value(0);
        self.sp = StackAddress::new(0x0000, None);

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
    fn update_regs(&mut self) {
        self.ra.update_word();
        self.rb.update_word();
        self.rc.update_word();
        self.rd.update_word();
    }

    fn reset_handler(&mut self, pins: &mut Pins) {
        match self.cycle {
            1 => {
                self.word = false;
                pins.address.set_16bit_value(0x0000);
                pins.rw = ReadWrite::Read;
            }
            2 => {
                self.temp_addr.set_extended_value(pins.data as u8);

                self.word = true;
                pins.address.set_16bit_value(0x0001);
                pins.rw = ReadWrite::Read;
            }
            3 => {
                self.temp_addr.set_seg_value(pins.data);

                self.pc = self.temp_addr;
                self.finish(pins);
            }
            _ => panic!("Tried to execute cycle {} for reset_handler", self.cycle),
        }
    }

    fn fetch_handler(&mut self, pins: &mut Pins) {
        match self.cycle {
            1 => {
                self.word = false;
                pins.address = self.pc;
                pins.rw = ReadWrite::Read;
                pins.bus_enable = true;
            }
            2 => {
                self.instruction.opcode = pins.data as u8;
                self.pc.increment();
                self.word = true;
                pins.address = self.pc;
                pins.rw = ReadWrite::Read;
            }
            3 => {
                self.instruction.metadata.set_data(pins.data);
                self.pc.increment_amount(2);

                self.finish(pins);
                self.state = CPUState::Execute;
            }
            _ => panic!("Tried to execute cycle {} for fetch_handler", self.cycle),
        }
    }

    fn execute_handler(&mut self, pins: &mut Pins) {
        pins.bus_enable = true;

        let parts: Vec<&str> = self.inst_info.opcodes.get(&self.instruction.opcode).unwrap_or_else(|| panic!("Unknown opcode: {:#04x}", self.instruction.opcode)).split("|").collect();
        let (inst_name, inst_mode) = (parts[0], parts[1]);

        let func: fn(&mut CPU, &mut Pins, AddressingMode) = match inst_name {
            "mov" => CPU::MOV,
            "movb" => CPU::MOVB,
            "st" => CPU::ST,
            "stb" => CPU::STB,
            "and" => CPU::AND,
            "andb" => CPU::ANDB,
            "or" => CPU::OR,
            "orb" => CPU::ORB,
            "xor" => CPU::XOR,
            "xorb" => CPU::XORB,
            "psh" => CPU::PSH,
            "pshb" => CPU::PSHB,
            "add" => CPU::ADD,
            "addb" => CPU::ADDB,
            "sub" => CPU::SUB,
            "subb" => CPU::SUBB,
            "cmp" => CPU::CMP,
            "cmpb" => CPU::CMPB,
            "inc" => CPU::INC,
            "incb" => CPU::INCB,
            "dec" => CPU::DEC,
            "decb" => CPU::DECB,
            "sbl" => CPU::SBL,
            "sblb" => CPU::SBLB,
            "sbr" => CPU::SBR,
            "sbrb" => CPU::SBRB,
            "rol" => CPU::ROL,
            "rolb" => CPU::ROLB,
            "ror" => CPU::ROR,
            "rorb" => CPU::RORB,
            "clc" => CPU::CLC,
            "cli" => CPU::CLI,
            "clv" => CPU::CLV,
            "sei" => CPU::SEI,
            "jmp" => CPU::JMP,
            "jsr" => CPU::JSR,
            "biz" => CPU::BIZ,
            "bin" => CPU::BIN,
            "bic" => CPU::BIC,
            "bio" => CPU::BIO,
            "bil" => CPU::BIL,
            "big" => CPU::BIG,
            "bnz" => CPU::BNZ,
            "bnn" => CPU::BNN,
            "bnc" => CPU::BNC,
            "bno" => CPU::BNO,
            "bnl" => CPU::BNL,
            "bng" => CPU::BNG,
            "rts" => CPU::RTS,
            "in" => CPU::IN,
            "out" => CPU::OUT,
            "hlt" => CPU::HLT,
            _ => panic!("Unimplemented function pointer to instruction function for instruction `{inst_name}`")
        };

        match inst_mode {
            "M" => (func)(self, pins, AddressingMode::Implied),
            "I" => (func)(self, pins, AddressingMode::Immediate),
            "R" => (func)(self, pins, AddressingMode::Register),
            "A" => (func)(self, pins, AddressingMode::Absolute),
            _ => panic!("Unimplemented addressing mode")
        }
    }

    fn interrupt_handler(&mut self, pins: &mut Pins) {
        self.word = false;
        match self.int_status {
            InterruptStatus::Normal => match self.cycle {
                1 => {
                    pins.irq.ack = true;
                }
                2 => {
                    pins.bus_enable = true;
                    pins.irq.ack = false;
                    self.int_num = pins.irq.data;
                    pins.address = ExtendedAddress::new_16bit_address((pins.irq.data * 3) as u16);
                    pins.rw = ReadWrite::Read;
                }
                3 => {
                    self.temp_addr.set_extended_value(pins.data as u8);
                    pins.address.increment();
                    pins.rw = ReadWrite::Read;
                }
                4 => {
                    self.temp_addr.set_hi_byte(pins.data as u8);
                    pins.address.increment();
                    pins.rw = ReadWrite::Read;
                }
                5 => {
                    self.temp_addr.set_low_byte(pins.data as u8);
                    pins.address = self.sp.into();
                    pins.data = self.flags.bits();
                    pins.rw = ReadWrite::Write;
                }
                6 => {
                    self.sp.increment();
                    pins.address = self.sp.into();
                    pins.data = self.pc.get_extended_value() as u16;
                    pins.rw = ReadWrite::Write;
                }
                7 => {
                    self.sp.increment();
                    pins.address = self.sp.into();
                    pins.data = self.pc.get_hi_byte() as u16;
                    pins.rw = ReadWrite::Write;
                }
                8 => {
                    self.sp.increment();
                    pins.address = self.sp.into();
                    pins.data = self.pc.get_low_byte() as u16;
                    pins.rw = ReadWrite::Write;
                }
                9 => {
                    self.sp.increment();
                    self.pc = self.temp_addr;
                    self.int_status = InterruptStatus::None;
                    self.finish(pins);
                }
                _ => panic!("Unknown normal interrupt cycle: {}", self.cycle),
            },
            InterruptStatus::NonMaskable => match self.cycle {
                1 => {
                    pins.irq.ack = true;
                }
                2 => {
                    pins.irq.ack = false;
                    pins.address = ExtendedAddress::new_16bit_address(0x03);
                    pins.rw = ReadWrite::Read;
                }
                3 => {
                    self.temp_addr.set_extended_value(pins.data as u8);
                    pins.address.increment();
                    pins.rw = ReadWrite::Read;
                }
                4 => {
                    self.temp_addr.set_hi_byte(pins.data as u8);
                    pins.address.increment();
                    pins.rw = ReadWrite::Read;
                }
                5 => {
                    self.temp_addr.set_low_byte(pins.data as u8);
                    pins.address = self.sp.into();
                    pins.data = self.flags.bits();
                    pins.rw = ReadWrite::Write;
                }
                6 => {
                    self.sp.increment();
                    pins.address = self.sp.into();
                    pins.data = self.pc.get_extended_value() as u16;
                    pins.rw = ReadWrite::Write;
                }
                7 => {
                    self.sp.increment();
                    pins.address = self.sp.into();
                    pins.data = self.pc.get_hi_byte() as u16;
                    pins.rw = ReadWrite::Write;
                }
                8 => {
                    self.sp.increment();
                    pins.address = self.sp.into();
                    pins.data = self.pc.get_low_byte() as u16;
                    pins.rw = ReadWrite::Write;
                }
                9 => {
                    self.sp.increment();
                    self.pc = self.temp_addr;
                    self.finish(pins);
                }
                _ => panic!("Unknown non-maskable interrupt cycle: {}", self.cycle),
            },
            _ => panic!("Cannot handle interrupt with status {:?}", self.int_status),
        }
    }

    fn finish(&mut self, pins: &mut Pins) {
        self.cycle = 0;
        pins.bus_enable = false;

        if self.int_status != InterruptStatus::None {
            self.state = CPUState::Interrupt;
            return;
        }

        if self.state == CPUState::Halt {
            return;
        }

        self.state = CPUState::Fetch;
    }

    fn decode_register(&mut self, mut reg: u8) -> RegisterReturn {
        reg &= 0xF;

        match reg {
            0b0000 => RegisterReturn::Full(&mut self.ra),
            0b0001 => RegisterReturn::Full(&mut self.rb),
            0b0010 => RegisterReturn::Full(&mut self.rc),
            0b0011 => RegisterReturn::Full(&mut self.rd),
            0b0100 => RegisterReturn::Full(&mut self.ra),
            0b0101 => RegisterReturn::Full(&mut self.rb),
            0b0110 => RegisterReturn::Full(&mut self.rc),
            0b0111 => RegisterReturn::Full(&mut self.rd),
            0b1000 => RegisterReturn::Partial(self.ra.get_mut_high()),
            0b1001 => RegisterReturn::Partial(self.ra.get_mut_low()),
            0b1010 => RegisterReturn::Partial(self.rb.get_mut_high()),
            0b1011 => RegisterReturn::Partial(self.rb.get_mut_low()),
            0b1100 => RegisterReturn::Partial(self.rc.get_mut_high()),
            0b1101 => RegisterReturn::Partial(self.rc.get_mut_low()),
            0b1110 => RegisterReturn::Partial(self.rd.get_mut_high()),
            0b1111 => RegisterReturn::Partial(self.rd.get_mut_low()),
            _ => panic!("Invalid register #"),
        }
    }

    fn set_flag(&mut self, flag: Flag, value: u16, value_is_byte: bool, value2: Option<u16>) {
        match flag {
            Flag::Z => {
                if value == 0 {
                    self.flags.set(flag, true);
                }
            }
            Flag::N => {
                if (value_is_byte && value & 0x80 > 0) || (!value_is_byte && value & 0x8000 > 0) {
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
            _ => panic!("Unhandled flag value given"),
        }
    }

    fn jump_if_flag(&mut self, pins: &mut Pins, flag: Flag, inst_name: &str) {
        match self.cycle {
            1..=2 => self.mode_absolute(pins, Some(false), None),
            3 => {
                self.mode_absolute(pins, Some(false), None);
                self.mode_absolute_jmp_cond(pins, flag, true);
            }
            _ => panic!(
                "{} tried to execute non-existent cycle {}",
                inst_name, self.cycle
            ),
        }
    }

    fn jump_not_flag(&mut self, pins: &mut Pins, flag: Flag, inst_name: &str) {
        match self.cycle {
            1..=2 => self.mode_absolute(pins, Some(false), None),
            3 => {
                self.mode_absolute(pins, Some(false), None);
                self.mode_absolute_jmp_cond(pins, flag, false);
            }
            _ => panic!(
                "{} tried to execute non-existent cycle {}",
                inst_name, self.cycle
            ),
        }
    }
}

type AbsoluteFunc = fn (&mut CPU, pins: &mut Pins, byte_instruction: bool);
// Addressing Modes
impl CPU {
    fn mode_absolute(&mut self, pins: &mut Pins, byte_instruction: Option<bool>, func: Option<AbsoluteFunc>) {
        match self.cycle {
            1 => {
                self.word = false;
                pins.address = self.pc;
                pins.rw = ReadWrite::Read;
            }
            2 => {
                self.temp_addr.set_extended_value(pins.data as u8);
                self.pc.increment();

                pins.address = self.pc;
                pins.rw = ReadWrite::Read;
                self.word = true;
            }
            3 => {
                self.temp_addr.set_seg_value(pins.data);
                self.pc.increment_amount(2);

                if self.instruction.metadata.reg_offset() > 0 {
                    let r1 = self.instruction.metadata.reg1();
                    if r1 & 0b100 > 0 {
                        let val = instructions::get_register(self.decode_register(r1));
                        self.temp_addr.offset_word(val as i16);
                    }
                } else {
                    let mut value: u8 = self.instruction.metadata.offset();
                    if self.instruction.metadata.offset_sign() > 0 {
                        value |= 0x80;
                    }

                    self.temp_addr.offset(value as i8);
                }

                if let Some(f) = func {
                    f(self, pins, !byte_instruction.unwrap_or(false));
                }
                
                self.word = !byte_instruction.unwrap_or(false);
            },
            _ => panic!("Addressing mode does not have cycle {}, cycles available: 1-3", self.cycle)
        }
    }

    fn mode_absolute_data(&mut self, pins: &mut Pins, _byte_instruction: bool) {
        pins.address = self.temp_addr;
        pins.rw = ReadWrite::Read;
    }

    fn mode_absolute_jmp(&mut self, pins: &mut Pins, _byte_instruction: bool) {
        self.pc = self.temp_addr;
        self.finish(pins);
    }

    fn mode_absolute_st(&mut self, pins: &mut Pins, _byte_instruction: bool) {
        pins.address = self.temp_addr;
        pins.data = instructions::get_register(self.decode_register(self.instruction.metadata.reg0()));
        pins.rw = ReadWrite::Write;
    }

    fn mode_absolute_jmp_cond(&mut self, pins: &mut Pins, flag: Flag, contains: bool) {
        if contains {
            if self.flags.contains(flag) {
                self.pc = self.temp_addr;
            } else {
                self.pc.increment();
            }
        } else if !self.flags.contains(flag) {
            self.pc = self.temp_addr;
        } else {
            self.pc.increment();
        }

        self.word = false;

        self.finish(pins);
    }

    fn mode_absolute_jsr(&mut self, pins: &mut Pins, _byte_instruction: bool) {
        pins.address = self.sp.into();
        pins.data = self.flags.bits();
        pins.rw = ReadWrite::Write;
    }

    fn mode_immediate(&mut self, pins: &mut Pins, byte_instruction: Option<bool>) {
        pins.address = self.pc;
        pins.rw = ReadWrite::Read;
        self.word = !byte_instruction.unwrap_or(false);
    }
}
