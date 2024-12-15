use crate::cpu::{Pins, ReadWrite::Read, CPU};

impl CPU {
    pub fn POP(&mut self, pins: &mut Pins) {
        match self.cycle {
            1 => {
                self.sp.decrement();
                pins.address = self.sp.into();
                pins.rw = Read;
            }
            2 => {
                *self.decode_register(self.instruction.metadata.reg0()) = pins.data;
                self.finish(pins);
            }
            _ => panic!("POP tried to execute non-existent cycle {}", self.cycle),
        }
    }
}
