use crate::cpu::{Flag, Pins, ReadWrite::{Read, Write}, CPU, AddressingMode};

impl CPU {
    pub fn JMP(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        match self.cycle {
            1..=3 => self.mode_absolute(pins, Some(false), Some(CPU::mode_absolute_jmp)),
            _ => panic!("JMP tried to execute non-existent cycle {}", self.cycle),
        }
    }

    pub fn BIZ(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.jump_if_flag(pins, Flag::Z, "BIZ");
    }

    pub fn BIN(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.jump_if_flag(pins, Flag::N, "BIN");
    }

    pub fn BIC(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.jump_if_flag(pins, Flag::C, "BIC");
    }

    pub fn BIO(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.jump_if_flag(pins, Flag::O, "BIO");
    }

    pub fn BIL(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.jump_if_flag(pins, Flag::L, "BIL");
    }

    pub fn BIG(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.jump_if_flag(pins, Flag::G, "BIG");
    }

    pub fn BNZ(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.jump_not_flag(pins, Flag::Z, "BNZ");
    }

    pub fn BNN(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.jump_not_flag(pins, Flag::N, "BNN");
    }

    pub fn BNC(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.jump_not_flag(pins, Flag::C, "BNC");
    }

    pub fn BNO(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.jump_not_flag(pins, Flag::O, "BNO");
    }

    pub fn BNL(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.jump_not_flag(pins, Flag::L, "BNL");
    }

    pub fn BNG(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.jump_not_flag(pins, Flag::G, "BNG");
    }

    pub fn JSR(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        match self.cycle {
            1..=3 => self.mode_absolute(pins, Some(false), Some(CPU::mode_absolute_jsr)),
            4 => {
                self.sp.increment();
                pins.address = self.sp.into();
                pins.data = self.pc.get_extended_value() as u16;
                pins.rw = Write;
                self.word = false;
            },
            5 => {
                self.sp.increment();
                pins.address = self.sp.into();
                pins.data = self.pc.get_16bit_value();
                pins.rw = Write;
                self.word = true;
            },
            6 => {
                self.sp.increment_amount(2);
                self.pc = self.temp_addr;
                self.finish(pins);
            }
            _ => panic!("JSR tried to execute non-existent cycle {}", self.cycle),
        }
    }

    pub fn RTS(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        match self.cycle {
            1 => {
                self.sp.decrement();
                pins.address = self.sp.into();
                pins.rw = Read;
                self.word = false;
            },
            2 => {
                self.sp.decrement();
                self.temp_addr.set_low_byte(pins.data as u8);

                pins.address = self.sp.into();
                pins.rw = Read;
            }
            3 => {
                self.sp.decrement();
                self.temp_addr.set_hi_byte(pins.data as u8);

                pins.address = self.sp.into();
                pins.rw = Read;
            }
            4 => {
                self.sp.decrement();
                self.temp_addr.set_extended_value(pins.data as u8);

                pins.address = self.sp.into();
                pins.rw = Read;
            },
            5 => {
                self.sp.decrement();
                self.flags.0.0 = pins.data;

                self.pc = self.temp_addr;
                self.finish(pins);
            }
            _ => panic!("RTS tried to execute non-existent cycle {}", self.cycle),
        }
    }
}