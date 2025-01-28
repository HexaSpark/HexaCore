use crate::cpu::{
    AddressingMode, Pins,
    ReadWrite::{Read, Write},
    CPU,
};

impl CPU {
    pub fn PSH(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => self.mode_immediate(pins, Some(false)),
                2 => {
                    pins.address = self.sp.into();
                    pins.rw = Write;
                }
                3 => {
                    self.sp.increment_amount(2);
                    self.pc.increment_amount(2);
                    self.word = false;

                    self.finish(pins);
                }
                _ => panic!("PSH(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => match self.cycle {
                0 => {
                    pins.address = self.sp.into();
                    pins.data = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    pins.rw = Write;
                }
                1 => {
                    self.sp.increment_amount(2);
                    self.finish(pins);
                }
                _ => panic!("PSH(R) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!("Invalid addressing mode for PSH instruction"),
        }
    }

    pub fn PSHB(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => self.mode_immediate(pins, Some(true)),
                2 => {
                    pins.address = self.sp.into();
                    pins.rw = Write;
                }
                3 => {
                    self.sp.increment();
                    self.pc.increment();

                    self.finish(pins);
                }
                _ => panic!("PSH(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => match self.cycle {
                0 => {
                    pins.address = self.sp.into();
                    pins.data = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    pins.rw = Write;
                }
                1 => {
                    self.sp.increment();
                    self.finish(pins);
                }
                _ => panic!("PSH(R) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!("Invalid addressing mode for PSH instruction"),
        }
    }

    // pub fn POP(&mut self, pins: &mut Pins) {
    //     match self.cycle {
    //         1 => {
    //             self.sp.decrement();
    //             pins.address = self.sp.into();
    //             pins.rw = Read;
    //         }
    //         2 => {
    //             *self.decode_register(self.instruction.metadata.reg0()) = pins.data;
    //             self.finish(pins);
    //         }
    //         _ => panic!("POP tried to execute non-existent cycle {}", self.cycle),
    //     }
    // }

    // pub fn CSK(&mut self, pins: &mut Pins) {
    //     match self.cycle {

    //     }
    // }
}
