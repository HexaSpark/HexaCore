use crate::cpu::{
    AddressingMode, Flag, Pins,
    ReadWrite::{Read, Write},
    CPU,
};

impl CPU {
    pub fn DEC(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Register => {
                let reg_num = self.instruction.metadata.reg0();
                let reg = super::get_register(self.decode_register(reg_num));

                let (res, _) = reg.overflowing_sub(1);

                self.set_flag(Flag::Z, res, false, None);
                self.set_flag(Flag::N, res, false, None);

                super::set_register(self.decode_register(reg_num), res);

                self.finish(pins);
            }
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(false), Some(CPU::mode_absolute_data)),
                4 => {
                    (pins.data, _) = pins.data.overflowing_sub(1);

                    self.set_flag(Flag::Z, pins.data, false, None);
                    self.set_flag(Flag::N, pins.data, false, None);

                    pins.address = self.temp_addr;
                    pins.rw = Write;
                }
                5 => {
                    self.finish(pins);
                }
                _ => panic!("DEC(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!("Invalid addressing mode for DEC instruction"),
        }
    }

    pub fn DECB(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Register => {
                let reg_num = self.instruction.metadata.reg0();
                let reg = super::get_register(self.decode_register(reg_num)) as u8;

                let (res, _) = reg.overflowing_sub(1);

                self.set_flag(Flag::Z, res as u16, true, None);
                self.set_flag(Flag::N, res as u16, true, None);

                super::set_register(self.decode_register(reg_num), res.into());

                self.finish(pins);
            }
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(true), Some(CPU::mode_absolute_data)),
                4 => {
                    let mut val = pins.data as u8;
                    (val, _) = val.overflowing_sub(1);

                    self.set_flag(Flag::Z, val as u16, true, None);
                    self.set_flag(Flag::N, val as u16, true, None);

                    pins.address = self.temp_addr;
                    pins.rw = Write;
                }
                5 => {
                    self.finish(pins);
                }
                _ => panic!("DECB(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!("Invalid addressing mode for DECB instruction"),
        }
    }
}
