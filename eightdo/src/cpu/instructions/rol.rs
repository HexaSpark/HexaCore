use crate::cpu::{
    AddressingMode, Flag, Pins,
    ReadWrite::{Read, Write},
    CPU,
};

impl CPU {
    pub fn ROL(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Register => {
                let reg_num = self.instruction.metadata.reg0();
                let reg = *self.decode_register(reg_num);

                self.flags.set(Flag::C, (reg & 0x80) > 0);

                let res = reg.rotate_left(1);

                self.set_flag(Flag::Z, res, None);
                self.set_flag(Flag::N, res, None);

                *self.decode_register(reg_num) = res;

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
                    self.flags.set(Flag::C, (pins.data & 0x80) > 0);

                    pins.data = pins.data.rotate_left(1);

                    self.set_flag(Flag::Z, pins.data, None);
                    self.set_flag(Flag::N, pins.data, None);

                    pins.address = self.temp_addr;
                    pins.rw = Write;
                }
                5 => {
                    self.finish(pins);
                }
                _ => panic!("ROL(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!("Invalid addressing mode for ROL instruction"),
        }
    }
}
