use crate::cpu::{AddressingMode, Flag, Pins, ReadWrite::Read, CPU};

impl CPU {
    pub fn ADD(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => self.mode_immediate(pins, Some(false)),
                2 => {
                    let val = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let temp =
                        val as u32 + pins.data as u32 + self.flags.contains(Flag::C) as u32;

                    self.flags.set(Flag::C, temp > 0xFFFF);
                    self.flags.set(
                        Flag::O,
                        (!(val ^ pins.data) ^ temp as u16) > 0,
                    );
                    self.set_flag(Flag::Z, temp as u16, false, None);
                    self.set_flag(Flag::N, temp as u16, false, None);

                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), temp as u16);

                    self.pc.increment_amount(2);
                    self.finish(pins);
                }
                _ => panic!("ADD(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let val1 = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                let val2 = super::get_register(self.decode_register(self.instruction.metadata.reg1()));
                let temp =
                        val1 as u32 + val2 as u32 + self.flags.contains(Flag::C) as u32;

                    self.flags.set(Flag::C, temp > 0xFFFF);
                    self.flags.set(
                        Flag::O,
                        (!(val1 ^ val2) ^ temp as u16) > 0,
                    );
                self.set_flag(Flag::Z, temp as u16, false, None);
                self.set_flag(Flag::N, temp as u16, false, None);

                let temp16 = self.temp16;
                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), temp16);

                self.pc.increment();
                self.finish(pins);
            }
            AddressingMode::Absolute => match self.cycle {
                1 => {
                    self.word = false;
                    pins.address = self.pc;
                    pins.rw = Read;
                }
                2 => {
                    self.temp_addr.set_extended_value(pins.data as u8);
                    self.pc.increment();

                    pins.address = self.pc;
                    pins.rw = Read;
                    self.word = true;
                }
                3 => {
                    self.temp_addr.set_seg_value(pins.data);
                    self.pc.increment_amount(2);

                    if self.instruction.metadata.reg_offset() > 0 {
                        let r1 = self.instruction.metadata.reg1();
                        if r1 & 0b100 > 0 {
                            let val = super::get_register(self.decode_register(r1));
                            self.temp_addr.offset_word(val as i16);
                        }
                    } else {
                        let mut value: u8 = self.instruction.metadata.offset();
                        if self.instruction.metadata.offset_sign() > 0 {
                            value |= 0x80;
                        }

                        self.temp_addr.offset(value as i8);
                    }

                    pins.address = self.temp_addr;
                    pins.rw = Read;
                }
                4 => {
                    let val = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let temp =
                        val as u32 + pins.data as u32 + self.flags.contains(Flag::C) as u32;

                    self.flags.set(Flag::C, temp > 0xFFFF);
                    self.flags.set(
                        Flag::O,
                        (!(val ^ pins.data) ^ temp as u16) > 0,
                    );
                    self.set_flag(Flag::Z, temp as u16, false, None);
                    self.set_flag(Flag::N, temp as u16, false, None);

                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), temp as u16);

                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("ADD(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!()
        }
    }

    pub fn ADDB(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => match self.cycle {
                1 => self.mode_immediate(pins, Some(true)),
                2 => {
                    let val = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let temp =
                        val + pins.data + self.flags.contains(Flag::C) as u16;

                    self.flags.set(Flag::C, temp > 0xFF);
                    self.flags.set(
                        Flag::O,
                        (!(val ^ pins.data) ^ temp) > 0,
                    );
                    self.set_flag(Flag::Z, temp & 0xFF, true, None);
                    self.set_flag(Flag::N, temp & 0xFF, true, None);

                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), temp  & 0xFF);

                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("ADD(I) tried to execute non-existent cycle {}", self.cycle),
            },
            AddressingMode::Register => {
                let val1 = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                let val2 = super::get_register(self.decode_register(self.instruction.metadata.reg1()));
                let temp = val1 + val2 + self.flags.contains(Flag::C) as u16;

                self.flags.set(Flag::C, temp > 0xFF);
                self.flags.set(
                    Flag::O,
                    (!(val1 ^ val2) ^ temp) > 0,
                );
                self.set_flag(Flag::Z, temp & 0xFF, true, None);
                self.set_flag(Flag::N, temp & 0xFF, true, None);

                super::set_register(self.decode_register(self.instruction.metadata.reg0()), temp & 0xFF);

                self.pc.increment();
                self.finish(pins);
            }
            AddressingMode::Absolute => match self.cycle {
                1 => {
                    self.word = false;
                    pins.address = self.pc;
                    pins.rw = Read;
                }
                2 => {
                    self.temp_addr.set_extended_value(pins.data as u8);
                    self.pc.increment();

                    pins.address = self.pc;
                    pins.rw = Read;
                    self.word = true;
                }
                3 => {
                    self.temp_addr.set_seg_value(pins.data);
                    self.pc.increment_amount(2);

                    if self.instruction.metadata.reg_offset() > 0 {
                        let r1 = self.instruction.metadata.reg1();
                        if r1 & 0b100 > 0 {
                            let val = super::get_register(self.decode_register(r1));
                            self.temp_addr.offset_word(val as i16);
                        }
                    } else {
                        let mut value: u8 = self.instruction.metadata.offset();
                        if self.instruction.metadata.offset_sign() > 0 {
                            value |= 0x80;
                        }

                        self.temp_addr.offset(value as i8);
                    }

                    pins.address = self.temp_addr;
                    pins.rw = Read;
                    self.word = false;
                }
                4 => {
                    let val = super::get_register(self.decode_register(self.instruction.metadata.reg0()));
                    let temp =
                        val + pins.data + self.flags.contains(Flag::C) as u16;

                    self.flags.set(Flag::C, temp > 0xFF);
                    self.flags.set(
                        Flag::O,
                        (!(val ^ pins.data) ^ temp) > 0,
                    );
                    self.set_flag(Flag::Z, temp & 0xFF, true, None);
                    self.set_flag(Flag::N, temp & 0xFF, true, None);

                    super::set_register(self.decode_register(self.instruction.metadata.reg0()), temp & 0xFF);

                    self.pc.increment();
                    self.finish(pins);
                }
                _ => panic!("ADD(A) tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!()
        }
    }
}
