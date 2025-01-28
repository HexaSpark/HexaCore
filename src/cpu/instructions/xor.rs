use crate::cpu::{AddressingMode, Flag, Pins, ReadWrite::Read, CPU};

impl CPU {
    pub fn XOR(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => self.mode_immediate(pins, Some(false)),
                2 => {
                    let reg_value = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let res = reg_value ^ pins.data;

                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), res);

                    self.set_flag(Flag::Z, res, false, None);
                    self.set_flag(Flag::N, res, false, None);
                    self.pc.increment_amount(2);

                    self.word = false;
                    self.finish(pins);
                },
                _ => panic!("XOR(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let res = super::get_register(
                    self.decode_register(self.instruction.metadata.reg0())) ^
                    super::get_register(self.decode_register(self.instruction.metadata.reg1())
                );

                super::set_register(self.decode_register(self.instruction.metadata.reg0()), res);

                self.set_flag(Flag::Z, res, false, None);
                self.set_flag(Flag::N, res, false, None);
                self.finish(pins);
            },
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(false), Some(CPU::mode_absolute_data)),
                4 => {
                    let reg_value = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let res = reg_value ^ pins.data;

                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), res);

                    self.set_flag(Flag::Z, res, false, None);
                    self.set_flag(Flag::N, res, false, None);
                    self.pc.increment_amount(2);

                    self.word = false;
                    self.finish(pins);
                }
                _ => panic!("XOR(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!()
        }
    }

    pub fn XORB(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => self.mode_immediate(pins, Some(true)),
                2 => {
                    let reg_value = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let res = reg_value & pins.data;

                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), res);

                    self.set_flag(Flag::Z, res, true, None);
                    self.set_flag(Flag::N, res, true, None);
                    self.pc.increment();

                    self.finish(pins);
                },
                _ => panic!("XORB(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let res = super::get_register(
                    self.decode_register(self.instruction.metadata.reg0())) &
                    super::get_register(self.decode_register(self.instruction.metadata.reg1())
                );

                super::set_register(self.decode_register(self.instruction.metadata.reg0()), res);

                self.set_flag(Flag::Z, res, true, None);
                self.set_flag(Flag::N, res, true, None);
                self.finish(pins);
            },
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(true), Some(CPU::mode_absolute_data)),
                4 => {
                    let reg_value = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let res = reg_value & pins.data;

                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), res);

                    self.set_flag(Flag::Z, res, true, None);
                    self.set_flag(Flag::N, res, true, None);
                    self.pc.increment();

                    self.word = false;
                    self.finish(pins);
                }
                _ => panic!("XORB(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!()
        }
    }
}
