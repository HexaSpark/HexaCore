#![allow(private_interfaces, non_snake_case)]

pub use self::consts::*;

mod add;
mod and;
mod hlt;
mod mov;
mod or;
mod pop;
mod psh;
mod st;
mod xor;
mod sub;

mod consts {
    pub const MOVI: u8 = gen_opcode("MOVI");
    pub const MOVR: u8 = gen_opcode("MOVR");
    pub const MOVA: u8 = gen_opcode("MOVA");
    pub const STA: u8 = gen_opcode("STA");
    pub const ANDI: u8 = gen_opcode("ANDI");
    pub const ANDR: u8 = gen_opcode("ANDR");
    pub const ANDA: u8 = gen_opcode("ANDA");
    pub const ORI: u8 = gen_opcode("ORI");
    pub const ORR: u8 = gen_opcode("ORR");
    pub const ORA: u8 = gen_opcode("ORA");
    pub const XORI: u8 = gen_opcode("XORI");
    pub const XORR: u8 = gen_opcode("XORR");
    pub const XORA: u8 = gen_opcode("XORA");
    pub const PSHI: u8 = gen_opcode("PSHI");
    pub const PSHR: u8 = gen_opcode("PSHR");
    pub const POPR: u8 = gen_opcode("POPR");
    pub const HLT: u8 = gen_opcode("HLT ");
    pub const ADDI: u8 = gen_opcode("ADDI");
    pub const ADDR: u8 = gen_opcode("ADDR");
    pub const ADDA: u8 = gen_opcode("ADDA");
    pub const SUBI: u8 = gen_opcode("SUBI ");
    pub const SUBR: u8 = gen_opcode("SUBR");
    pub const SUBA: u8 = gen_opcode("SUBA");

    const fn gen_opcode(opcode: &str) -> u8 {
        let mut res: u8 = 0;
        let mut i = 0;

        while i < opcode.len() {
            let x = opcode.as_bytes()[i];
            (res, _) = res.overflowing_add(x);
            i += 1;
        }

        res
    }
}
