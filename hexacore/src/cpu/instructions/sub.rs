use crate::cpu::{AddressingMode, Flag, Pins, ReadWrite::Read, CPU};

impl CPU {
    pub fn SUB(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => {
                    pins.address = self.pc;
                    pins.rw = Read;
                }
                2 => {
                    let val = *self.decode_register(self.instruction.metadata.reg0());
                    let data = (pins.data as u16) ^ 0x00FF;
                    self.temp16 = val as u16 + data + self.flags.contains(Flag::C) as u16;

                    self.flags.set(Flag::C, self.temp16 > 0xFF);
                    self.flags.set(
                        Flag::O,
                        ((!((val as u16) ^ data) & ((val as u16) ^ self.temp16)) & 0x8000) > 0,
                    );
                    self.set_flag(Flag::Z, self.temp16 as u8, None);
                    self.set_flag(Flag::N, self.temp16 as u8, None);

                    *self.decode_register(self.instruction.metadata.reg0()) = self.temp16 as u8;

                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("ADD(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let val1 = *self.decode_register(self.instruction.metadata.reg0());
                let val2 =
                    (*self.decode_register(self.instruction.metadata.reg1()) as u16) ^ 0x00FF;
                self.temp16 = val1 as u16 + val2 + self.flags.contains(Flag::C) as u16;

                self.flags.set(Flag::C, self.temp16 > 0xFF);
                self.flags.set(
                    Flag::O,
                    ((!((val1 as u16) ^ val2) & ((val1 as u16) ^ self.temp16)) & 0x8000) > 0,
                );
                self.set_flag(Flag::Z, self.temp16 as u8, None);
                self.set_flag(Flag::N, self.temp16 as u8, None);

                *self.decode_register(self.instruction.metadata.reg0()) = self.temp16 as u8;

                self.pc.increment();
                self.finish(pins);
            }
            AddressingMode::Absolute => match self.cycle {
                1 => {
                    self.temp_addr
                        .set_extended_value(self.instruction.metadata.ext_addr());
                    pins.address = self.pc;
                    pins.rw = Read;
                }
                2 => {
                    self.temp_addr.set_hi_byte(pins.data);
                    self.pc.increment();

                    pins.address = self.pc;
                    pins.rw = Read;
                }
                3 => {
                    self.temp_addr.set_low_byte(pins.data);
                    self.temp_addr
                        .offset(self.instruction.metadata.offset() as i8);
                    let r1 = self.instruction.metadata.reg1();
                    if r1 & 0b100 > 0 {
                        let val = *self.decode_register(r1) as i8;
                        self.temp_addr.offset(val);
                    }
                    self.pc.increment();

                    pins.address = self.temp_addr;
                    pins.rw = Read;
                }
                4 => {
                    let val = *self.decode_register(self.instruction.metadata.reg0());
                    let data = (pins.data as u16) ^ 0x00FF;
                    self.temp16 = val as u16 + data + self.flags.contains(Flag::C) as u16;

                    self.flags.set(Flag::C, self.temp16 > 0xFF);
                    self.flags.set(
                        Flag::O,
                        ((!((val as u16) ^ data) & ((val as u16) ^ self.temp16)) & 0x8000) > 0,
                    );
                    self.set_flag(Flag::Z, self.temp16 as u8, None);
                    self.set_flag(Flag::N, self.temp16 as u8, None);

                    *self.decode_register(self.instruction.metadata.reg0()) = self.temp16 as u8;

                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("ADD(A) tried to execute non-existent cycle {}", self.cycle),
            },
        }
    }
}
