use crate::cpu::{
    AddressingMode, Pins,
    ReadWrite::{Read, Write},
    CPU,
};

impl CPU {
    pub fn ST(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(false), Some(CPU::mode_absolute_st)),
                4 => {
                    self.finish(pins);
                }
                _ => panic!("ST tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!("Invalid addressing mode for ST instruction"),
        }
    }

    pub fn STB(&mut self, pins: &mut Pins, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute => match self.cycle {
                1..=3 => self.mode_absolute(pins, Some(true), Some(CPU::mode_absolute_st)),
                4 => {
                    self.finish(pins);
                }
                _ => panic!("STB tried to execute non-existent cycle {}", self.cycle),
            },
            _ => panic!("Invalid addressing mode for STB instruction"),
        }
    }
}
