use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    str::Chars,
    env
};

#[derive(Clone)]
struct InstructionInfo {
    name: String,
    size: u8,
    opcode: HashMap<String, u8>,
}

impl InstructionInfo {
    pub fn new(name: impl Into<String>, size: u8, opcode: HashMap<String, u8>) -> Self {
        Self {
            name: name.into(),
            size,
            opcode,
        }
    }
}

const REGISTERS: [(&str, u8); 5] = [("ra", 0), ("rb", 1), ("rc", 2), ("rd", 3), ("rf", 4)];

const KEYWORDS: [&str; 5] = ["org", "word", "byte", "ascii", "asciiz"];

fn get_instructions() -> Vec<InstructionInfo> {
    fn gen_opcode(opcode: &str) -> u8 {
        let mut res: u8 = 0;
        let mut i = 0;

        while i < opcode.len() {
            let x = opcode.as_bytes()[i];
            (res, _) = res.overflowing_add(x);
            i += 1;
        }

        res
    }

    vec![
        InstructionInfo::new(
            "mov",
            2,
            HashMap::from([
                ("RI".into(), gen_opcode("MOVI")),
                ("RR".into(), gen_opcode("MOVR")),
                ("RA".into(), gen_opcode("MOVA")),
            ]),
        ),
        InstructionInfo::new("st", 2, HashMap::from([("RA".into(), gen_opcode("STA"))])),
        InstructionInfo::new(
            "and",
            2,
            HashMap::from([
                ("RI".into(), gen_opcode("ANDI")),
                ("RR".into(), gen_opcode("ANDR")),
                ("RA".into(), gen_opcode("ANDA")),
            ]),
        ),
        InstructionInfo::new(
            "or",
            2,
            HashMap::from([
                ("RI".into(), gen_opcode("ORI")),
                ("RR".into(), gen_opcode("ORR")),
                ("RA".into(), gen_opcode("ORA")),
            ]),
        ),
        InstructionInfo::new(
            "xor",
            2,
            HashMap::from([
                ("RI".into(), gen_opcode("XORI")),
                ("RR".into(), gen_opcode("XORR")),
                ("RA".into(), gen_opcode("XORA")),
            ]),
        ),
        InstructionInfo::new(
            "psh",
            1,
            HashMap::from([
                ("I".into(), gen_opcode("PSHI")),
                ("R".into(), gen_opcode("PSHR")),
            ]),
        ),
        InstructionInfo::new("mov", 1, HashMap::from([("R".into(), gen_opcode("POPR"))])),
        InstructionInfo::new("hlt", 0, HashMap::from([("M".into(), gen_opcode("HLT "))])),
        InstructionInfo::new(
            "add",
            2,
            HashMap::from([
                ("RI".into(), gen_opcode("ADDI")),
                ("RR".into(), gen_opcode("ADDR")),
                ("RA".into(), gen_opcode("ADDA")),
            ]),
        ),
        InstructionInfo::new(
            "sub",
            2,
            HashMap::from([
                ("RI".into(), gen_opcode("SUBI ")),
                ("RR".into(), gen_opcode("SUBR")),
                ("RA".into(), gen_opcode("SUBA")),
            ]),
        ),
        InstructionInfo::new(
            "cmp",
            2,
            HashMap::from([
                ("RI".into(), gen_opcode("CMPI")),
                ("RR".into(), gen_opcode("CMPR")),
                ("RA".into(), gen_opcode("CMPA")),
            ]),
        ),
        InstructionInfo::new(
            "inc",
            1,
            HashMap::from([
                ("R".into(), gen_opcode("INCR")),
                ("A".into(), gen_opcode("INCA  ")),
            ]),
        ),
        InstructionInfo::new(
            "dec",
            1,
            HashMap::from([
                ("R".into(), gen_opcode("DECR")),
                ("A".into(), gen_opcode("DECA")),
            ]),
        ),
        InstructionInfo::new(
            "sbl",
            1,
            HashMap::from([
                ("R".into(), gen_opcode("SBLR  ")),
                ("A".into(), gen_opcode("SBLA")),
            ]),
        ),
        InstructionInfo::new(
            "sbr",
            1,
            HashMap::from([
                ("R".into(), gen_opcode("SBRR")),
                ("A".into(), gen_opcode("SBRA")),
            ]),
        ),
        InstructionInfo::new(
            "rol",
            1,
            HashMap::from([
                ("R".into(), gen_opcode("ROLR")),
                ("A".into(), gen_opcode("ROLA")),
            ]),
        ),
        InstructionInfo::new(
            "ror",
            1,
            HashMap::from([
                ("R".into(), gen_opcode("RORR")),
                ("A".into(), gen_opcode("RORA ")),
            ]),
        ),
        InstructionInfo::new("clc", 0, HashMap::from([("M".into(), gen_opcode("CLC"))])),
        InstructionInfo::new("cli", 0, HashMap::from([("M".into(), gen_opcode("CLI"))])),
        InstructionInfo::new("clv", 0, HashMap::from([("M".into(), gen_opcode("CLV"))])),
        InstructionInfo::new("sei", 0, HashMap::from([("M".into(), gen_opcode("SEI"))])),
        InstructionInfo::new("jmp", 1, HashMap::from([("A".into(), gen_opcode("JMP"))])),
        InstructionInfo::new("jsr", 1, HashMap::from([("A".into(), gen_opcode("JSR"))])),
        InstructionInfo::new("biz", 1, HashMap::from([("A".into(), gen_opcode("BIZ "))])),
        InstructionInfo::new("bin", 1, HashMap::from([("A".into(), gen_opcode("BIN"))])),
        InstructionInfo::new("bic", 1, HashMap::from([("A".into(), gen_opcode("BIC"))])),
        InstructionInfo::new("bio", 1, HashMap::from([("A".into(), gen_opcode("BIO"))])),
        InstructionInfo::new("bil", 1, HashMap::from([("A".into(), gen_opcode("BIL"))])),
        InstructionInfo::new("big", 1, HashMap::from([("A".into(), gen_opcode("BIG "))])),
        InstructionInfo::new("bnz", 1, HashMap::from([("A".into(), gen_opcode("BNZ  "))])),
        InstructionInfo::new("bnn", 1, HashMap::from([("A".into(), gen_opcode("BNN"))])),
        InstructionInfo::new("bnc", 1, HashMap::from([("A".into(), gen_opcode("BNC"))])),
        InstructionInfo::new("bno", 1, HashMap::from([("A".into(), gen_opcode("BNO"))])),
        InstructionInfo::new("bnl", 1, HashMap::from([("A".into(), gen_opcode("BNL"))])),
        InstructionInfo::new("bng", 1, HashMap::from([("A".into(), gen_opcode("BNG "))])),
        InstructionInfo::new("rts", 0, HashMap::from([("M".into(), gen_opcode("RTS"))])),
        InstructionInfo::new("in", 2, HashMap::from([("RI".into(), gen_opcode("IN"))])),
        InstructionInfo::new("out", 2, HashMap::from([("RI".into(), gen_opcode("OUT"))])),
    ]
}

fn get_instruction_info(instruction: &String) -> Option<InstructionInfo> {
    let insts = get_instructions();

    let inst = insts.iter().find(|&x| &x.name == instruction);

    inst?;

    Some(inst.unwrap().clone())
}

fn get_register(register: &String) -> u8 {
    REGISTERS.iter().find(|&a| a.0 == register).unwrap().1
}

#[derive(Clone)]
enum TokenOption {
    None,
    Prev,
    Some(Token),
}

#[allow(dead_code)]
impl TokenOption {
    pub fn is_some(&self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(&TokenOption::Some(Token::Plus))
    }

    pub fn is_prev(&self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(&TokenOption::Prev)
    }

    pub fn is_none(&self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(&TokenOption::None)
    }

    pub fn unwrap(self) -> Token {
        match self {
            TokenOption::None => panic!("called `TokenOption` on a `None` value"),
            TokenOption::Prev => panic!("called `TokenOption` on a `Prev` value"),
            TokenOption::Some(token) => token,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(i8)]
enum Token {
    None = -1,
    Int(u32),
    Identifier(String),
    Keyword(String),
    Register(String),
    Label(String),
    Directive(String),
    Instruction(String),
    Address(u32),
    // Reference(String),
    String(String),
    Plus,
    Minus,
    Comma,
}

impl Token {
    pub fn str_val(&self) -> &String {
        match self {
            Token::Identifier(s) => s,
            Token::Keyword(s) => s,
            Token::Register(s) => s,
            Token::Label(s) => s,
            Token::Directive(s) => s,
            Token::Instruction(s) => s,
            // Token::Reference(s) => s,
            Token::String(s) => s,
            _ => panic!("Not a string val"),
        }
    }

    // pub fn u32_val(&self) -> &u32 {
    //     match self {
    //         Token::Int(v) => v,
    //         Token::Address(v) => v,
    //         _ => panic!("Not a u32 val")
    //     }
    // }

    // pub fn compare_type(&self, other: Self) -> bool {
    //     std::mem::discriminant(self) == std::mem::discriminant(&other)
    // }
}

struct LexerParser<'a> {
    chars: Chars<'a>,
    current_char: char,
}

type LexerResult = (Vec<Token>, Option<String>);

impl<'a> LexerParser<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut chars = text.chars();
        let current_char = chars.next().unwrap_or('\0');

        Self {
            chars,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.current_char = self.chars.next().unwrap_or('\0')
    }

    pub fn tokenize(&mut self) -> LexerResult {
        let mut tokens = vec![];

        while self.current_char != '\0' {
            if self.current_char.is_whitespace() {
                self.advance();
                continue;
            }

            match self.current_char {
                '0' => {
                    self.advance();

                    if self.current_char == '\0' {
                        return (tokens, Some("Expected 'x' or 'X', found <EOF>".to_owned()));
                    }

                    if self.current_char != 'x' && self.current_char != 'X' {
                        return (
                            tokens,
                            Some(format!("Expected 'x' or 'X', found {}", self.current_char)),
                        );
                    }

                    self.advance();

                    tokens.push(Token::Int(self.make_number()));
                }
                '$' => {
                    self.advance();
                    tokens.push(Token::Address(self.make_number()));
                }
                '"' => {
                    self.advance();
                    tokens.push(self.make_string());
                }
                '.' => {
                    self.advance();
                    let token = self.make_id_kwd_reg_inst();

                    if let Token::Keyword(kw) = token {
                        tokens.push(Token::Directive(kw.to_ascii_lowercase()));
                        continue;
                    }

                    return (
                        tokens,
                        Some(format!("Expected directive, not '{}'", token.str_val())),
                    );
                }
                ':' => {
                    self.advance();

                    if tokens.is_empty() {
                        return (
                            tokens,
                            Some("No label name given, only found ':'".to_owned()),
                        );
                    }

                    let token = tokens.pop().unwrap();

                    if let Token::Identifier(val) = token {
                        tokens.push(Token::Label(val));
                        continue;
                    }

                    return (
                        tokens,
                        Some(format!("Expected Identifier, found '{:?}'", token)),
                    );
                }
                ',' => {
                    self.advance();
                    tokens.push(Token::Comma)
                }
                '+' => {
                    self.advance();
                    tokens.push(Token::Plus)
                }
                '-' => {
                    self.advance();
                    tokens.push(Token::Minus)
                },
                ';' => {
                    self.advance();
                    
                    while self.current_char != '\n' {
                        self.advance();
                    }

                    self.advance();
                }
                _ => {
                    if self.current_char.is_ascii_alphanumeric() || self.current_char == '_' {
                        tokens.push(self.make_id_kwd_reg_inst());
                        continue;
                    }

                    return (
                        tokens,
                        Some(format!("Unknown character: '{}'", self.current_char)),
                    );
                }
            }
        }

        (tokens, None)
    }

    fn make_number(&mut self) -> u32 {
        let mut num = String::new();

        while self.current_char != '\0' && self.current_char.is_ascii_hexdigit() {
            num.push(self.current_char);
            self.advance();
        }

        u32::from_str_radix(&num, 16).unwrap()
    }

    fn make_id_kwd_reg_inst(&mut self) -> Token {
        let mut word = String::new();

        while self.current_char != '\0'
            && (self.current_char.is_ascii_alphanumeric() || self.current_char == '_')
        {
            word.push(self.current_char);
            self.advance();
        }

        let word_low = word.to_ascii_lowercase();
        let str_word = word_low.as_str();

        if KEYWORDS.contains(&str_word) {
            Token::Keyword(word.to_ascii_lowercase())
        } else if REGISTERS.iter().any(|x| x.0 == word) {
            Token::Register(word.to_ascii_lowercase())
        } else {
            let instructions = get_instructions();

            if instructions.iter().any(|info| info.name == str_word) {
                Token::Instruction(word.to_ascii_lowercase())
            } else {
                Token::Identifier(word)
            }
        }
    }

    fn make_string(&mut self) -> Token {
        let mut string = String::new();
        let mut escaped = false;

        let escape_reserved: HashMap<char, char> = HashMap::from([
            ('n', '\n'),
            ('t', '\t'),
        ]);

        while self.current_char != '\0' {
            if escaped {
                if escape_reserved.contains_key(&self.current_char) {
                    string.push(escape_reserved[&self.current_char]);
                } else {
                    string.push(self.current_char);
                }
                
                escaped = false;
                self.advance();
            }

            if self.current_char == '\\' {
                escaped = true;
                self.advance();
                continue;
            } else if self.current_char == '"' {
                self.advance();
                break;
            }

            string.push(self.current_char);
            self.advance();
        }

        Token::String(string)
    }
}

struct Assembler {
    tokens: Vec<Token>,
    idx: usize,
    current_token: Token,
    address: usize,
    labels: HashMap<String, u32>,
}

impl Assembler {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            current_token: tokens.first().unwrap_or(&Token::None).clone(),
            tokens,
            idx: 0,
            address: 0,
            labels: HashMap::new(),
        }
    }

    fn advance(&mut self) {
        self.idx += 1;
        self.current_token = self.tokens.get(self.idx).unwrap_or(&Token::None).clone();
    }

    pub fn assemble(&mut self) -> Vec<(u8, TokenOption)> {
        let mut data: Vec<(u8, TokenOption)> = vec![(0, TokenOption::None); 0x8000];

        while self.current_token != Token::None {
            match &self.current_token {
                Token::Directive(directive) => {
                    if directive == "org" {
                        self.advance();

                        if let Token::Address(address) = self.current_token {
                            self.address = address as usize;
                            self.advance();
                            continue;
                        }

                        panic!("Expected Address, not '{:?}'", self.current_token);
                    } else if directive == "byte" {
                        self.advance();

                        if let Token::Int(value) = self.current_token {
                            if value > 0xFF {
                                panic!("Value {:x} cannot fit inside one byte. Use `.word` for two bytes.", value);
                            }

                            data[self.address].0 = (value & 0xFF) as u8;
                            self.address += 1;
                            self.advance();
                            continue;
                        }

                        panic!("Expected Int, not '{:?}'", self.current_token);
                    } else if directive == "word" {
                        self.advance();

                        if let Token::Address(value) = &self.current_token {
                            if value > &0xFFFF {
                                panic!("Value {:x} cannot fit inside two bytes. Use a .byte and a .word for addresses.", value);
                            }

                            data[self.address].0 = ((value & 0xFF00) >> 8) as u8;
                            data[self.address + 1].0 = (value & 0xFF) as u8;
                            self.address += 2;
                            self.advance();
                            continue;
                        } else if let Token::Identifier(_) = &self.current_token {
                            data[self.address].1 = TokenOption::Some(self.current_token.clone());
                            data[self.address + 1].1 = TokenOption::Prev;
                            self.address += 2;
                            self.advance();
                            continue;
                        }

                        panic!(
                            "Expecting Address or Identifier, not '{:?}'.",
                            self.current_token
                        );
                    } else if directive == "ascii" {
                        self.advance();

                        if let Token::String(string) = &self.current_token {
                            for char in string.chars() {
                                if !char.is_ascii() {
                                    panic!("Found a non-ascii character in \"{string}\"");
                                }

                                data[self.address].0 = char as u8;
                                self.address += 1;
                            }

                            self.advance();
                            continue;
                        }

                        panic!("Expected String, not '{:?}'", self.current_token);
                    } else if directive == "asciiz" {
                        self.advance();

                        if let Token::String(string) = &self.current_token {
                            for char in string.chars() {
                                if !char.is_ascii() {
                                    panic!("Found a non-ascii character in \"{string}\"");
                                }

                                data[self.address].0 = char as u8;
                                self.address += 1;
                            }

                            self.address += 1;
                            self.advance();
                            continue;
                        }

                        panic!("Expected String, not '{:?}'", self.current_token);
                    }
                }
                Token::Label(label) => {
                    self.labels.insert(label.clone(), self.address as u32);
                    self.advance();
                }
                Token::Instruction(instruction) => {
                    let info = get_instruction_info(instruction).unwrap();
                    self.advance();

                    match info.size {
                        0 => {
                            let opcode = *info.opcode.get("M").unwrap();

                            data[self.address].0 = opcode;
                            data[self.address + 1].0 = 0x00;
                            data[self.address + 2].0 = 0x00;
                            self.address += 3;
                        }
                        1 => match &self.current_token {
                            Token::Register(reg) => {
                                let opcode = *info.opcode.get("R").unwrap();

                                data[self.address].0 = opcode;
                                data[self.address + 1].0 = 0;
                                data[self.address + 2].0 = get_register(reg);
                                self.address += 3;
                                self.advance();
                            }
                            Token::Int(num) => {
                                let opcode = *info.opcode.get("I").unwrap();

                                data[self.address].0 = opcode;
                                data[self.address + 1].0 = 0;
                                data[self.address + 2].0 = 0x00;
                                data[self.address + 3].0 = *num as u8;
                                self.address += 4;
                                self.advance();
                            }
                            Token::Address(address) => {
                                let opcode = *info.opcode.get("A").unwrap();
                                let address = *address;
                                let ext = ((address & 0x30000) >> 10) as u8;
                                self.advance();

                                if let Token::Plus = self.current_token {
                                    self.advance();

                                    if let Token::Int(offset) = &self.current_token {
                                        data[self.address + 1].0 = *offset as u8;
                                        self.advance();
                                    } else if let Token::Register(reg) = &self.current_token {
                                        let reg_num = get_register(reg) | 0b100;
                                        data[self.address + 2].0 |= reg_num << 3;
                                        self.advance();
                                    } else {
                                        panic!("Expected Int or Register, not {:?}", self.current_token);
                                    }
                                } else {
                                    data[self.address + 1].0 = 0;
                                }

                                data[self.address].0 = opcode;
                                data[self.address + 2].0 |= ext;
                                data[self.address + 3].0 = ((address & 0xFF00) >> 8) as u8;
                                data[self.address + 4].0 = (address & 0xFF) as u8;
                                self.address += 5;
                            }
                            Token::Identifier(identifier) => {
                                let ident = &identifier.clone();
                                let opcode = *info.opcode.get("A").unwrap();
                                let ident_tok = self.current_token.clone();
                                self.advance();

                                data[self.address].0 = opcode;
                                data[self.address + 1].0 = 0;

                                if let Token::Plus = self.current_token {
                                    self.advance();

                                    if let Token::Int(offset) = self.current_token {
                                        data[self.address + 1].0 = offset as u8;
                                        self.advance();
                                    } else if let Token::Register(reg) = &self.current_token {
                                        let reg_num = get_register(reg) | 0b100;
                                        data[self.address + 2].0 |= reg_num << 3;
                                        self.advance();
                                    } else {
                                        panic!("Expected Int or Register, not {:?}", self.current_token);
                                    }
                                } else {
                                    data[self.address + 1].0 = 0;
                                }

                                if !self.labels.contains_key(ident) {
                                    data[self.address + 3].1 =
                                        TokenOption::Some(ident_tok);
                                    data[self.address + 4].1 = TokenOption::Prev;

                                    self.address += 5;
                                    continue;
                                }

                                let value = self.labels[ident];

                                data[self.address + 2].0 |= ((value & 0x30000) >> 10) as u8;
                                data[self.address + 3].0 = ((value & 0xFF00) >> 8) as u8;
                                data[self.address + 4].0 = (value & 0xFF) as u8;
                                self.address += 5;
                            }
                            _ => panic!("Invalid Token with instruction size 1"),
                        },
                        2 => {
                            let dest_reg: u8;

                            if let Token::Register(reg) = &self.current_token {
                                dest_reg = get_register(reg);
                            } else {
                                panic!("Expecting Register, not {:?}", self.current_token);
                            }

                            self.advance();

                            if let Token::Comma = self.current_token {
                                self.advance();
                            } else {
                                panic!("Expected Comma, not {:?}", self.current_token);
                            }

                            match &self.current_token {
                                Token::Register(reg) => {
                                    let opcode = *info.opcode.get("RR").unwrap();
                                    let reg_2 = get_register(reg) << 3;

                                    data[self.address].0 = opcode;
                                    data[self.address + 1].0 = 0;
                                    data[self.address + 2].0 = dest_reg | reg_2;

                                    self.address += 3;
                                    self.advance();
                                }
                                Token::Int(num) => {
                                    let opcode = *info.opcode.get("RI").unwrap();

                                    data[self.address].0 = opcode;
                                    data[self.address + 1].0 = 0;
                                    data[self.address + 2].0 = dest_reg;
                                    data[self.address + 3].0 = *num as u8;
                                    self.address += 4;
                                    self.advance();
                                }
                                Token::Address(address) => {
                                    let opcode = *info.opcode.get("RA").unwrap();
                                    let address = *address;
                                    let ext = ((address & 0x30000) >> 10) as u8;
                                    self.advance();

                                    if let Token::Plus = self.current_token {
                                        self.advance();
    
                                        if let Token::Int(offset) = self.current_token {
                                            data[self.address + 1].0 = offset as u8;
                                            self.advance();
                                        } else if let Token::Register(reg) = &self.current_token {
                                            let reg_num = get_register(reg) | 0b100;
                                            data[self.address + 2].0 |= reg_num << 3;
                                            self.advance();
                                        } else {
                                            panic!("Expected Int or Register, not {:?}", self.current_token);
                                        }
                                    } else {
                                        data[self.address + 1].0 = 0;
                                    }

                                    data[self.address].0 = opcode;
                                    data[self.address + 2].0 = dest_reg | ext;
                                    data[self.address + 3].0 = ((address & 0xFF00) >> 8) as u8;
                                    data[self.address + 4].0 = (address & 0xFF) as u8;
                                    self.address += 5;
                                }
                                Token::Identifier(identifier) => {
                                    let ident = &identifier.clone();
                                    let opcode = *info.opcode.get("RA").unwrap();
                                    let ident_tok = self.current_token.clone();
                                    self.advance();

                                    data[self.address].0 = opcode;
                                    data[self.address + 2].0 = dest_reg;

                                    if let Token::Plus = self.current_token {
                                        self.advance();
    
                                        if let Token::Int(offset) = self.current_token {
                                            data[self.address + 1].0 = offset as u8;
                                            self.advance();
                                        } else if let Token::Register(reg) = &self.current_token {
                                            let reg_num = get_register(reg) | 0b100;
                                            data[self.address + 2].0 |= reg_num << 3;
                                            self.advance();
                                        } else {
                                            panic!("Expected Int or Register, not {:?}", self.current_token);
                                        }
                                    } else {
                                        data[self.address + 1].0 = 0;
                                    }

                                    if !self.labels.contains_key(ident) {
                                        data[self.address + 3].1 =
                                            TokenOption::Some(ident_tok);
                                        data[self.address + 4].1 = TokenOption::Prev;

                                        self.address += 5;
                                        continue;
                                    }

                                    let value = self.labels[ident];

                                    data[self.address + 2].0 |= ((value & 0x30000) >> 10) as u8;
                                    data[self.address + 3].0 = ((value & 0xFF00) >> 8) as u8;
                                    data[self.address + 4].0 = (value & 0xFF) as u8;
                                    self.address += 5;
                                }
                                _ => panic!("Invalid Token with instruction size 2"),
                            }
                        }
                        _ => panic!("Invalid instruction size"),
                    }
                }
                _ => panic!("Unknown Token to Assemble: {:?}", self.current_token),
            }
        }

        data
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() != 1 {
        panic!("Missing file argument.");
    }

    let content = fs::read_to_string(&args[0]).unwrap();
    let mut lexer = LexerParser::new(&content);
    let (tokens, error) = lexer.tokenize();

    if let Some(error) = error {
        println!("{}", error);
        return;
    }

    println!("Tokens: {:?}", tokens);

    let mut assembler = Assembler::new(tokens);
    let res = assembler.assemble();

    println!("Final Address: 0x{:04x}", assembler.address as u16);
    println!("Labels: {:?}", assembler.labels);

    let mut prog: [u8; 0x8000] = [0; 0x8000];

    for (i, (data, token)) in res.iter().enumerate() {
        if token.is_some() {
            match token.clone().unwrap() {
                Token::Identifier(ident) => {
                    let ident_addr = assembler.labels.get(&ident).unwrap();

                    prog[i] = ((ident_addr & 0xFF00) >> 8) as u8;
                    prog[i + 1] = (ident_addr & 0xFF) as u8;
                    continue;
                }
                _ => panic!("Unknown token in assembler return data."),
            }
        } else if token.is_prev() {
            continue;
        }

        prog[i] = *data;
    }

    let bin_name = args[0].strip_suffix(".8do").unwrap().to_owned() + ".bin";

    let mut file = File::create(bin_name).unwrap();
    file.write_all(&prog).unwrap();
}
