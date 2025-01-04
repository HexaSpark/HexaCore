#![allow(private_interfaces, non_snake_case)]

pub use self::consts::*;

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
mod rti;
mod stack;  
mod flags;
mod jumps;
mod io;

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
    pub const CMPI: u8 = gen_opcode("CMPI");
    pub const CMPR: u8 = gen_opcode("CMPR");
    pub const CMPA: u8 = gen_opcode("CMPA");
    pub const INCR: u8 = gen_opcode("INCR");
    pub const INCA: u8 = gen_opcode("INCA  ");
    pub const DECR: u8 = gen_opcode("DECR");
    pub const DECA: u8 = gen_opcode("DECA");
    pub const SBLR: u8 = gen_opcode("SBLR  ");
    pub const SBLA: u8 = gen_opcode("SBLA");
    pub const SBRR: u8 = gen_opcode("SBRR");
    pub const SBRA: u8 = gen_opcode("SBRA");
    pub const ROLR: u8 = gen_opcode("ROLR");
    pub const ROLA: u8 = gen_opcode("ROLA");
    pub const RORR: u8 = gen_opcode("RORR");
    pub const RORA: u8 = gen_opcode("RORA ");
    pub const CLC: u8 = gen_opcode("CLC");
    pub const CLI: u8 = gen_opcode("CLI");
    pub const CLV: u8 = gen_opcode("CLV");
    pub const SEI: u8 = gen_opcode("SEI");
    pub const JMP: u8 = gen_opcode("JMP");
    pub const JSR: u8 = gen_opcode("JSR");
    pub const BIZ: u8 = gen_opcode("BIZ ");
    pub const BIN: u8 = gen_opcode("BIN");
    pub const BIC: u8 = gen_opcode("BIC");
    pub const BIO: u8 = gen_opcode("BIO");
    pub const BIL: u8 = gen_opcode("BIL");
    pub const BIG: u8 = gen_opcode("BIG ");
    pub const BNZ: u8 = gen_opcode("BNZ  ");
    pub const BNN: u8 = gen_opcode("BNN");
    pub const BNC: u8 = gen_opcode("BNC");
    pub const BNO: u8 = gen_opcode("BNO");
    pub const BNL: u8 = gen_opcode("BNL");
    pub const BNG: u8 = gen_opcode("BNG ");
    pub const RTS: u8 = gen_opcode("RTS");
    pub const IN: u8 = gen_opcode("IN");
    pub const OUT: u8 = gen_opcode("OUT");
    pub const RTI: u8 = gen_opcode("RTI");

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
