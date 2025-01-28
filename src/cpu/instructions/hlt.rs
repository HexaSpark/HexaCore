use crate::cpu::{AddressingMode, Pins, CPU};

impl CPU {
    pub fn HLT(&mut self, _pins: &mut Pins, _mode: AddressingMode) {
        self.state = crate::cpu::CPUState::Halt;
    }
}
