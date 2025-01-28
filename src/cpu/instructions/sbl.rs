use crate::cpu::{
    AddressingMode, Flag, Pins,
    ReadWrite::{Read, Write},
    CPU,
};

impl CPU {
    pub fn SBL(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Register => {
                let reg_num = self.instruction.metadata.reg0();
                let reg = super::get_register(self.decode_register(reg_num));

                self.flags.set(Flag::C, (reg & 0x8000) > 0);

                let res = reg << 1;

                self.set_flag(Flag::Z, res, false, None);
                self.set_flag(Flag::N, res, false, None);

                super::set_register(self.decode_register(reg_num), res);

                self.finish(pins);
            }
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(false), Some(CPU::mode_absolute_data)),
                4 => {
                    self.flags.set(Flag::C, (pins.data & 0x8000) > 0);

                    pins.data <<= 1;

                    self.set_flag(Flag::Z, pins.data, false, None);
                    self.set_flag(Flag::N, pins.data, false, None);

                    pins.address = self.temp_addr;
                    pins.rw = Write;
                }
                5 => {
                    self.finish(pins);
                }
                _ => panic!("SBL(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!("Invalid addressing mode for SBL instruction"),
        }
    }

    pub fn SBLB(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Register => {
                let reg_num = self.instruction.metadata.reg0();
                let reg = super::get_register(self.decode_register(reg_num));

                self.flags.set(Flag::C, (reg & 0x80) > 0);

                let res = reg << 1;

                self.set_flag(Flag::Z, res, true, None);
                self.set_flag(Flag::N, res, true, None);

                super::set_register(self.decode_register(reg_num), res & 0xFF);

                self.finish(pins);
            }
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(true), Some(CPU::mode_absolute_data)),
                4 => {
                    self.flags.set(Flag::C, (pins.data & 0x80) > 0);

                    pins.data <<= 1;
                    pins.data &= 0xFF;

                    self.set_flag(Flag::Z, pins.data, true, None);
                    self.set_flag(Flag::N, pins.data, true, None);

                    pins.address = self.temp_addr;
                    pins.rw = Write;
                }
                5 => {
                    self.finish(pins);
                }
                _ => panic!("SBL(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!("Invalid addressing mode for SBL instruction"),
        }
    }
}
