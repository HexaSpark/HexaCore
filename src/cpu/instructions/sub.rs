use crate::cpu::{AddressingMode, Flag, Pins, ReadWrite::Read, CPU};

impl CPU {
    // FIXME: Subtraction is off by 1. Have to add 1? (Tested with D007 - CA7)
    pub fn SUB(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => self.mode_immediate(pins, Some(false)),
                2 => {
                    let val = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let temp =
                        val as u32 + ((pins.data ^ 0xFFFF) + 1) as u32 + self.flags.contains(Flag::C) as u32;

                    self.flags.set(Flag::C, temp > 0xFFFF);
                    self.flags.set(
                        Flag::O,
                        (!(val ^ (pins.data ^ 0xFF)) ^ temp as u16) > 0,
                    );
                    self.set_flag(Flag::Z, temp as u16, false, None);
                    self.set_flag(Flag::N, temp as u16, false, None);

                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), temp as u16);

                    self.pc.increment_amount(2);
                    self.finish(pins);
                }
                _ => panic!("SUB(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let val1 = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                let val2 = (super::get_register(self.decode_register(self.instruction.metadata.reg1())) ^ 0xFFFF) + 1;
                let temp =
                        val1 as u32 + val2 as u32 + self.flags.contains(Flag::C) as u32;

                    self.flags.set(Flag::C, temp > 0xFFFF);
                    self.flags.set(
                        Flag::O,
                        (!(val1 ^ val2) ^ temp as u16) > 0,
                    );
                self.set_flag(Flag::Z, temp as u16, false, None);
                self.set_flag(Flag::N, temp as u16, false, None);

                super::set_register(self.decode_register(self.instruction.metadata.reg0()), temp as u16);

                self.pc.increment();
                self.finish(pins);
            }
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(false), Some(CPU::mode_absolute_data)),
                4 => {
                    let val = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let temp =
                        val as u32 + ((pins.data ^ 0xFFFF) + 1) as u32 + self.flags.contains(Flag::C) as u32;

                    self.flags.set(Flag::C, temp > 0xFFFF);
                    self.flags.set(
                        Flag::O,
                        (!(val ^ (pins.data ^ 0xFF)) ^ temp as u16) > 0,
                    );
                    self.set_flag(Flag::Z, temp as u16, false, None);
                    self.set_flag(Flag::N, temp as u16, false, None);

                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), temp as u16);

                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("SUB(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!()
        }
    }

    pub fn SUBB(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => self.mode_immediate(pins, Some(true)),
                2 => {
                    let val = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let temp =
                        val + (pins.data ^ 0xFF) + 1 + self.flags.contains(Flag::C) as u16;

                    self.flags.set(Flag::C, temp > 0xFF);
                    self.flags.set(
                        Flag::O,
                        (!(val ^ (pins.data ^ 0xFF)) ^ temp) > 0,
                    );
                    self.set_flag(Flag::Z, temp & 0xFF, true, None);
                    self.set_flag(Flag::N, temp & 0xFF, true, None);

                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), temp & 0xFF);

                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("SUB(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let val1 = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                let val2 = (super::get_register(self.decode_register(self.instruction.metadata.reg1())) ^ 0xFF) + 1;
                let temp = val1 + val2 + self.flags.contains(Flag::C) as u16;

                self.flags.set(Flag::C, temp > 0xFF);
                self.flags.set(
                    Flag::O,
                    (!(val1 ^ val2) ^ temp) > 0,
                );
                self.set_flag(Flag::Z, temp & 0xFF, true, None);
                self.set_flag(Flag::N, temp & 0xFF, true, None);

                super::set_register(self.decode_register(self.instruction.metadata.reg0()), temp & 0xFF);

                self.pc.increment();
                self.finish(pins);
            }
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(true), Some(CPU::mode_absolute_data)),
                4 => {
                    let val = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let temp =
                        val + (pins.data ^ 0xFF) + 1 + self.flags.contains(Flag::C) as u16;

                    self.flags.set(Flag::C, temp > 0xFF);
                    self.flags.set(
                        Flag::O,
                        (!(val ^ (pins.data ^ 0xFF)) ^ temp) > 0,
                    );
                    self.set_flag(Flag::Z, temp & 0xFF, true, None);
                    self.set_flag(Flag::N, temp & 0xFF, true, None);

                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), temp & 0xFF);

                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("SUB(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!()
        }
    }
}
