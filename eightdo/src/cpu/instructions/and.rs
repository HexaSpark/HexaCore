use crate::cpu::{AddressingMode, Flag, Pins, ReadWrite::Read, CPU};

impl CPU {
    pub fn AND(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => {
                    pins.address = self.pc;
                    pins.rw = Read;
                }
                2 => {
                    let reg = self.instruction.metadata.reg0();
                    let reg_value = *self.decode_register(reg);
                    let res = reg_value & pins.data;

                    *self.decode_register(reg) = res;

                    self.set_flag(Flag::Z, res, None);
                    self.set_flag(Flag::N, res, None);

                    self.finish(pins);
                }
                _ => panic!("AND(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let r0 = self.instruction.metadata.reg0();
                let r1 = self.instruction.metadata.reg0();
                let r0_val = *self.decode_register(r0);
                let res = r0_val & *self.decode_register(r1);

                *self.decode_register(r0) = res;

                self.set_flag(Flag::Z, res, None);
                self.set_flag(Flag::N, res, None);

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
                    let reg = self.instruction.metadata.reg0();
                    let reg_value = *self.decode_register(reg);
                    let res = reg_value & pins.data;

                    *self.decode_register(reg) = res;

                    self.set_flag(Flag::Z, res, None);
                    self.set_flag(Flag::N, res, None);

                    self.finish(pins);
                }
                _ => panic!("AND(A) tried to execute non-existent cycle {}", self.cycle),
            },
        }
    }
}
