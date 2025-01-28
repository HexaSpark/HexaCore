use crate::cpu::{AddressingMode, Flag, Pins, CPU};

impl CPU {
    pub fn CLC(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.flags.set(Flag::C, false);
        self.finish(pins);
    }

    pub fn CLI(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.flags.set(Flag::I, false);
        self.finish(pins);
    }

    pub fn CLV(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.flags.set(Flag::O, false);
        self.finish(pins);
    }

    pub fn SEI(&mut self, pins: &mut Pins, _mode: AddressingMode) {
        self.flags.set(Flag::I, true);
        self.finish(pins);
    }
}