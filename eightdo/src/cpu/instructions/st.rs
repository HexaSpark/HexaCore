use crate::cpu::{
    AddressingMode, Pins,
    ReadWrite::{Read, Write},
    CPU,
};

impl CPU {
    pub fn ST(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute => match self.cycle {
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
                    self.pc.increment();

                    pins.address = self.temp_addr;
                    pins.data = *self.decode_register(self.instruction.metadata.reg0());
                    pins.rw = Write;
                }
                4 => {
                    self.finish(pins);
                }
                _ => panic!("ST tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!("Invalid addressing mode for MOV instruction"),
        }
    }
}
