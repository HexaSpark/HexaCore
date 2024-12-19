use crate::cpu::{Pins, ReadWrite::{Read, Write}, CPU};

impl CPU {
    pub fn IN(&mut self, pins: &mut Pins) {
        match self.cycle {
            1 => {
                pins.address = self.pc;
                pins.rw = Read;
            },
            2 => {
                self.pc.increment();
                pins.io_address = pins.data;
                pins.io_rw = Read;
                pins.io_enable = true;
            },
            3 => {
                *self.decode_register(self.instruction.metadata.reg0()) = pins.io_data;
                pins.io_enable = false;
                self.finish(pins);
            },
            _ => panic!("IN tried to execute non-existent cycle {}", self.cycle),
        }
    }

    pub fn OUT(&mut self, pins: &mut Pins) {
        match self.cycle {
            1 => {
                pins.address = self.pc;
                pins.rw = Read;
            },
            2 => {
                self.pc.increment();
                pins.io_address = pins.data;
                pins.io_data = *self.decode_register(self.instruction.metadata.reg0());
                pins.io_rw = Write;
                pins.io_enable = true;
            },
            3 => {
                pins.io_enable = false;
                self.finish(pins);
            },
            _ => panic!("OUT tried to execute non-existent cycle {}", self.cycle),
        }
    }
}