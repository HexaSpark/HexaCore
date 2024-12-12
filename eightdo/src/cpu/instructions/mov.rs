use crate::cpu::{
    AddressingMode, Flag, Pins,
    ReadWrite::{Read, Write},
    CPU,
};

impl CPU {
    pub fn MOV(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => {
                    pins.address = self.pc;
                    pins.rw = Read;
                }
                2 => {
                    *self.decode_register(self.instruction.metadata.reg0()) = pins.data;

                    self.set_flag(Flag::Z, pins.data, None);
                    self.set_flag(Flag::N, pins.data, None);

                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("MOV(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let r0 = self.instruction.metadata.reg0();
                let r1 = self.instruction.metadata.reg1();
                let val = *self.decode_register(r1);

                *self.decode_register(r0) = val;

                self.set_flag(Flag::Z, val, None);
                self.set_flag(Flag::N, val, None);

                self.finish(pins);
            }
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
                    pins.rw = Read;
                }
                4 => {
                    *self.decode_register(self.instruction.metadata.reg0()) = pins.data;

                    self.set_flag(Flag::Z, pins.data, None);
                    self.set_flag(Flag::N, pins.data, None);

                    self.finish(pins);
                }
                _ => panic!("MOV(A) tried to execute non-existent cycle {}", self.cycle),
            },
            // _ => panic!("Invalid addressing mode for MOV instruction")
        }
    }
}
