use crate::cpu::{AddressingMode, Flag, Pins, ReadWrite::Read, CPU};

impl CPU {
    pub fn CMP(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => self.mode_immediate(pins, Some(false)),
                2 => {
                    let reg = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let (reg_n_flag, _) = reg.overflowing_sub(pins.data);

                    self.flags.set(Flag::Z, reg == pins.data);
                    self.flags.set(Flag::N, (reg_n_flag & 0x8000) > 0);
                    self.flags.set(Flag::G, reg > pins.data);
                    self.flags.set(Flag::L, reg < pins.data);

                    self.word = false;
                    self.pc.increment_amount(2);
                    self.finish(pins);
                }
                _ => panic!("CMP(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let reg0 = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                let reg1 = super::get_register(self.decode_register(self.instruction.metadata.reg1()));

                let (reg_n_flag, _) = reg0.overflowing_sub(reg1);

                self.flags.set(Flag::Z, reg0 == reg1);
                self.flags.set(Flag::N, (reg_n_flag & 0x8000) > 0);
                self.flags.set(Flag::G, reg0 > reg1);
                self.flags.set(Flag::L, reg0 < reg1);

                self.finish(pins);
            }
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(false), Some(CPU::mode_absolute_data)),
                4 => {
                    let reg = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let (reg_n_flag, _) = reg.overflowing_sub(pins.data);

                    self.flags.set(Flag::Z, reg == pins.data);
                    self.flags.set(Flag::N, (reg_n_flag & 0x8000) > 0);
                    self.flags.set(Flag::G, reg > pins.data);
                    self.flags.set(Flag::L, reg < pins.data);

                    self.word = false;
                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("CMP(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!()
        }
    }

    pub fn CMPB(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => self.mode_immediate(pins, Some(true)),
                2 => {
                    let reg = super::get_register(self.decode_register(self.instruction.metadata.reg0())) as u8;
                    let data = pins.data as u8;
                    let (reg_n_flag, _) = reg.overflowing_sub(data);

                    self.flags.set(Flag::Z, reg == data);
                    self.flags.set(Flag::N, (reg_n_flag & 0x80) > 0);
                    self.flags.set(Flag::G, reg > data);
                    self.flags.set(Flag::L, reg < data);

                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("CMPB(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let reg0 = super::get_register(self.decode_register(self.instruction.metadata.reg0())) as u8;
                let reg1 = super::get_register(self.decode_register(self.instruction.metadata.reg1())) as u8;

                let (reg_n_flag, _) = reg0.overflowing_sub(reg1);

                self.flags.set(Flag::Z, reg0 == reg1);
                self.flags.set(Flag::N, (reg_n_flag & 0x80) > 0);
                self.flags.set(Flag::G, reg0 > reg1);
                self.flags.set(Flag::L, reg0 < reg1);

                self.finish(pins);
            }
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(true), Some(CPU::mode_absolute_data)),
                4 => {
                    let reg = super::get_register(self.decode_register(self.instruction.metadata.reg0())) as u8;
                    let data = pins.data as u8;
                    let (reg_n_flag, _) = reg.overflowing_sub(data);

                    self.flags.set(Flag::Z, reg == data);
                    self.flags.set(Flag::N, (reg_n_flag & 0x80) > 0);
                    self.flags.set(Flag::G, reg > data);
                    self.flags.set(Flag::L, reg < data);

                    self.word = false;
                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("CMPB(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!()
        }
    }
}
