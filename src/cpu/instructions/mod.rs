#![allow(private_interfaces, non_snake_case)]

mod add;
mod and;
mod cmp;
mod dec;
mod hlt;
mod inc;
mod mov;
mod or;
mod st;
mod sub;
mod xor;
mod sbl;
mod sbr;
mod rol;
mod ror;
mod stack;
mod flags;
mod jumps;
mod io;

pub(crate) fn get_register(register_ret: super::RegisterReturn) -> u16 {
    match register_ret {
        super::RegisterReturn::Full(reg) => reg.get_word(),
        super::RegisterReturn::Partial(reg8) => (*reg8) as u16
    }
}

pub(crate) fn set_register(register_ret: super::RegisterReturn, data: u16) {
    match register_ret {
        super::RegisterReturn::Full(reg) => {
            reg.set_word(data);
        },
        super::RegisterReturn::Partial(reg8) => {
            *reg8 = data as u8;
        }
    }
}