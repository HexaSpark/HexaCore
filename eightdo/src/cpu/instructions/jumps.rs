use crate::cpu::{AddressingMode, Flag, Pins, ReadWrite::{Read, Write}, CPU};

impl CPU {
    pub fn JMP(&mut self, pins: &mut Pins) {
        match self.cycle {
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
                
                self.pc = self.temp_addr;
                self.finish(pins);
            },
            _ => panic!("JMP tried to execute non-existent cycle {}", self.cycle),
        }
    }

    pub fn BIZ(&mut self, pins: &mut Pins) {
        self.jump_if_flag(pins, Flag::Z, "BIZ");
    }

    pub fn BIN(&mut self, pins: &mut Pins) {
        self.jump_if_flag(pins, Flag::N, "BIN");
    }

    pub fn BIC(&mut self, pins: &mut Pins) {
        self.jump_if_flag(pins, Flag::C, "BIC");
    }

    pub fn BIO(&mut self, pins: &mut Pins) {
        self.jump_if_flag(pins, Flag::O, "BIO");
    }

    pub fn BIL(&mut self, pins: &mut Pins) {
        self.jump_if_flag(pins, Flag::L, "BIL");
    }

    pub fn BIG(&mut self, pins: &mut Pins) {
        self.jump_if_flag(pins, Flag::G, "BIG");
    }

    pub fn BNZ(&mut self, pins: &mut Pins) {
        self.jump_not_flag(pins, Flag::Z, "BNZ");
    }

    pub fn BNN(&mut self, pins: &mut Pins) {
        self.jump_not_flag(pins, Flag::N, "BNN");
    }

    pub fn BNC(&mut self, pins: &mut Pins) {
        self.jump_not_flag(pins, Flag::C, "BNC");
    }

    pub fn BNO(&mut self, pins: &mut Pins) {
        self.jump_not_flag(pins, Flag::O, "BNO");
    }

    pub fn BNL(&mut self, pins: &mut Pins) {
        self.jump_not_flag(pins, Flag::L, "BNL");
    }

    pub fn BNG(&mut self, pins: &mut Pins) {
        self.jump_not_flag(pins, Flag::G, "BNG");
    }

    pub fn JSR(&mut self, pins: &mut Pins) {
        match self.cycle {
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
                
                self.pc = self.temp_addr;
                self.finish(pins);
            },
            4 => {
                pins.address = self.sp.into();
                pins.data = self.pc.get_extended_value();
                pins.rw = Write;
            },
            5 => {
                self.sp.increment();
                pins.address = self.sp.into();
                pins.data = self.pc.get_hi_byte();
                pins.rw = Write;
            },
            6 => {
                self.sp.increment();
                pins.address = self.sp.into();
                pins.data = self.pc.get_low_byte();
                pins.rw = Write;
            },
            7 => {
                self.sp.increment();
                self.pc = self.temp_addr;
                self.finish(pins);
            }
            _ => panic!("JSR tried to execute non-existent cycle {}", self.cycle),
        }
    }
}