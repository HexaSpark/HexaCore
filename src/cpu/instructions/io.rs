use crate::cpu::{AddressingMode, Pins, ReadWrite::{Read, Write}, CPU};

impl CPU {
    pub fn IN(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        match self.cycle {
            1 => {
                pins.address = self.pc;
                pins.rw = Read;
                self.word = false;
            },
            2 => {
                self.pc.increment();
                pins.io_address = pins.data as u8;
                pins.io_rw = Read;
                pins.io_enable = true;
            },
            3 => {
                super::set_register(self.decode_register(self.instruction.metadata.reg0()), pins.io_data as u16);
                pins.io_enable = false;
                self.finish(pins);
            },
            _ => panic!("IN tried to execute non-existent cycle {}", self.cycle),
        }
    }

    pub fn OUT(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        match self.cycle {
            1 => {
                pins.address = self.pc;
                pins.rw = Read;
                self.word = false;
            },
            2 => {
                self.pc.increment();
                pins.io_address = pins.data as u8;
                pins.io_data = super::get_register(self.decode_register(self.instruction.metadata.reg0())) as u8;
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