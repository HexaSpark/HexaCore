use crate::cpu::{Pins, CPU};

impl CPU {
    pub fn HLT(&mut self, _pins: &mut Pins) {
        self.state = crate::cpu::CPUState::Halt;
    }
}
