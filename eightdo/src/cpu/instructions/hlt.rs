use crate::cpu::{
    AddressingMode, Flag, Pins,
    ReadWrite::{Read, Write},
    CPU,
};

impl CPU {
    pub fn HLT(&mut self, pins: &mut Pins) {
        self.state = crate::cpu::CPUState::Halt;
    }
}
