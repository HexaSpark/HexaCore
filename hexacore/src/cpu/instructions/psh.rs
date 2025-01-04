use crate::cpu::{
    AddressingMode, Pins,
    ReadWrite::{Read, Write},
    CPU,
};

impl CPU {
    pub fn PSH(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => {
                    pins.address = self.pc;
                    pins.rw = Read;
                }
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
                    pins.data = *self.decode_register(self.instruction.metadata.reg0());
                    pins.rw = Write;
                }
                1 => {
                    self.sp.increment();
                    self.finish(pins);
                }
                _ => panic!("PSH(I) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!("Invalid addressing mode for PSH instruction"),
        }
    }
}
