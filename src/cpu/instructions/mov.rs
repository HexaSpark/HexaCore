use crate::cpu::{AddressingMode, Flag, Pins, ReadWrite::Read, CPU};

impl CPU {
    pub fn MOV(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => self.mode_immediate(pins, Some(false)),
                2 => {
                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), pins.data);

                    self.set_flag(Flag::Z, pins.data, false, None);
                    self.set_flag(Flag::N, pins.data, false, None);

                    self.pc.increment_amount(2);
                    self.finish(pins);
                },
                _ => panic!("MOV(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let r0 = self.instruction.metadata.reg0();
                let r1 = self.instruction.metadata.reg1();
                let val = super::get_register(self.decode_register(r0));

                super::set_register(self.decode_register(r1), val);

                self.set_flag(Flag::Z, val, false, None);
                self.set_flag(Flag::N, val, false, None);

                self.finish(pins);
            },
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(false), Some(CPU::mode_absolute_data)),
                4 => {
                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), pins.data);

                    self.set_flag(Flag::Z, pins.data, false, None);
                    self.set_flag(Flag::N, pins.data, false, None);

                    self.finish(pins);
                }
                _ => panic!("MOV(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!()
        }
    }

    pub fn MOVB(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => self.mode_immediate(pins, Some(true)),
                2 => {
                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), pins.data);
                    self.update_regs();

                    self.set_flag(Flag::Z, pins.data, true, None);
                    self.set_flag(Flag::N, pins.data, true, None);

                    self.pc.increment();
                    self.finish(pins);
                },
                _ => panic!("MOVB(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let r0 = self.instruction.metadata.reg0();
                let r1 = self.instruction.metadata.reg1();
                let val = super::get_register(self.decode_register(r0));

                super::set_register(self.decode_register(r1), val);
                self.update_regs();

                self.set_flag(Flag::Z, val, true, None);
                self.set_flag(Flag::N, val, true, None);

                self.finish(pins);
            },
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(true), Some(CPU::mode_absolute_data)),
                4 => {
                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), pins.data);
                    self.update_regs();

                    self.set_flag(Flag::Z, pins.data, true, None);
                    self.set_flag(Flag::N, pins.data, true, None);

                    self.finish(pins);
                }
                _ => panic!("MOVB(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!()
        }
    }
}
