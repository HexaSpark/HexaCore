use crate::cpu::{AddressingMode, Flag, Pins, ReadWrite::Read, CPU};

impl CPU {
    pub fn CMP(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => {
                    pins.address = self.pc;
                    pins.rw = Read;
                }
                2 => {
                    let reg = *self.decode_register(self.instruction.metadata.reg0());
                    let (reg_n_flag, _) = reg.overflowing_sub(pins.data);

                    self.flags.set(Flag::Z, reg == pins.data);
                    self.flags.set(Flag::N, (reg_n_flag & 0x80) > 0);
                    self.flags.set(Flag::G, reg > pins.data);
                    self.flags.set(Flag::L, reg < pins.data);

                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("CMP(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let reg0 = *self.decode_register(self.instruction.metadata.reg0());
                let reg1 = *self.decode_register(self.instruction.metadata.reg1());

                let (reg_n_flag, _) = reg0.overflowing_sub(reg1);

                self.flags.set(Flag::Z, reg0 == reg1);
                self.flags.set(Flag::N, (reg_n_flag & 0x80) > 0);
                self.flags.set(Flag::G, reg0 > reg1);
                self.flags.set(Flag::L, reg0 < reg1);

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
                    let r1 = self.instruction.metadata.reg1();
                    if r1 & 0b100 > 0 {
                        let val = *self.decode_register(r1) as i8;
                        self.temp_addr.offset(val);
                    }
                    self.pc.increment();

                    pins.address = self.temp_addr;
                    pins.rw = Read;
                }
                4 => {
                    let reg = *self.decode_register(self.instruction.metadata.reg0());
                    let (reg_n_flag, _) = reg.overflowing_sub(pins.data);

                    self.flags.set(Flag::Z, reg == pins.data);
                    self.flags.set(Flag::N, (reg_n_flag & 0x80) > 0);
                    self.flags.set(Flag::G, reg > pins.data);
                    self.flags.set(Flag::L, reg < pins.data);

                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("CMP(A) tried to execute non-existent cycle {}", self.cycle),
            },
        }
    }
}
