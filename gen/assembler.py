DATA = {
    "instructions": {
        "LDR": {"size": 2, "opcode": {"RI": 0x2B, "RA": 0x23, "RR": 0x34, "RP": 0x52}},
        "STR": {"size": 2, "opcode": {"RA": 0x3A}},
        "ADD": {"size": 2, "opcode": {"RI": 0x12, "RA": 0x0A, "RR": 0x1B}},
        "SUB": {"size": 2, "opcode": {"RI": 0x33, "RA": 0x4B, "RR": 0x3C}},
        "JMP": {"size": 1, "opcode": {"A": 0xE7}},
        "BIZ": {"size": 1, "opcode": {"A": 0xB0}},
        "BNZ": {"size": 1, "opcode": {"A": 0xB1}},
        "BIN": {"size": 1, "opcode": {"A": 0xB2}},
        "BNN": {"size": 1, "opcode": {"A": 0xB3}},
        "BIC": {"size": 1, "opcode": {"A": 0xB4}},
        "BNC": {"size": 1, "opcode": {"A": 0xB5}},
        "BIO": {"size": 1, "opcode": {"A": 0xB6}},
        "BNO": {"size": 1, "opcode": {"A": 0xB7}},
        "BIL": {"size": 1, "opcode": {"A": 0xB8}},
        "BNL": {"size": 1, "opcode": {"A": 0xB9}},
        "BIG": {"size": 1, "opcode": {"A": 0xBA}},
        "BNG": {"size": 1, "opcode": {"A": 0xBB}},
        "SBL": {"size": 1, "opcode": {"A": 0x22, "R": 0x01}},
        "SBR": {"size": 1, "opcode": {"A": 0x28, "R": 0x39}},
        "ROL": {"size": 1, "opcode": {"A": 0x2E, "R": 0x3F}},
        "ROR": {"size": 1, "opcode": {"A": 0x54, "R": 0x45}},
        "CLC": {"size": 0, "opcode": 0xD2},
        "INC": {"size": 1, "opcode": {"A": 0x3B, "R": 0x2C}},
        "DEC": {"size": 1, "opcode": {"A": 0x0D, "R": 0x1E}},
        "HLT": {"size": 0, "opcode": 0xE8},
        "CMP": {"size": 2, "opcode": {"RI": 0x29, "RA": 0x21, "RR": 0x32}},
        "LDO": {"size": 2, "opcode": {"RA": 0x20, "RR": 0x31}},
        "NOP": {"size": 0, "opcode": 0x00},
        "PSH": {"size": 1, "opcode": {"A": 0x4C, "R": 0x3D, "I": 0x74}},
        "POP": {"size": 1, "opcode": {"A": 0x30, "R": 0x41}},
        "JSR": {"size": 1, "opcode": {"A": 0xEF}},
        "RET": {"size": 0, "opcode": 0xEB},
        "BII": {"size": 1, "opcode": {"A": 0xD4}},
        "BNI": {"size": 1, "opcode": {"A": 0xD9}},
        "CLI": {"size": 0, "opcode": 0xD8},
        "SEI": {"size": 0, "opcode": 0xE1},
        "RTI": {"size": 0, "opcode": 0x0F},
        "MUL": {"size": 2, "opcode": {"RI": 0x57, "RA": 0x4F, "RR": 0x40}}
    },
    "registers": ["RA", "RB", "RC", "RD", "RO"],
    "reg_lookup": {"A": 0, "B": 1, "C": 2, "D": 3, "O": 4},
    "keywords": ["ORG", "WORD", "BYTE", "ASCII", "ASCIIZ"]
}

import sys
from enum import Enum
from dataclasses import dataclass
from string import hexdigits, ascii_letters, whitespace

class TokenType(Enum):
    INT = 0 # !
    IDENTIFIER = 1 # !
    KEYWORD = 2 # !
    REGISTER = 3 # !
    LABEL = 4 # !
    DIRECTIVE = 5 # !
    INSTRUCTION = 6 # !
    ADDRESS = 7 # !
    POINTER = 8 # !
    REFERENCE = 9 # !
    OFFSET = 10 # !
    STRING = 11 # !

@dataclass
class Token:
    type_: TokenType
    value: str

    def __repr__(self):
        return f"{self.type_.name}:{self.value}"

class LexerParser:
    def __init__(self, text: str):
        self.text = text
        self.current_char = None
        self.position = -1
        self.__advance()

    def __advance(self):
        self.position += 1
        self.current_char = self.text[self.position] if self.position < len(self.text) else None

    def tokenize(self):
        tokens = []

        while self.current_char is not None:
            if self.current_char == '0':
                self.__advance()
                if self.current_char is None or self.current_char not in "xX":
                    return None, "Expected 'x' or 'X'"
                
                self.__advance()
                
                tokens.append(self.__make_number())
            elif self.current_char == '$':
                self.__advance()
                token = self.__make_number()
                token.type_ = TokenType.ADDRESS
                tokens.append(token)
            elif self.current_char == '.':
                self.__advance()
                token = self.__make_identifier_or_keyword_or_register_or_instruction()
                if token.type_ != TokenType.KEYWORD: 
                    return None, f"Expected directive, not '{token.value}'"
                token.type_ = TokenType.DIRECTIVE

                tokens.append(token)
            elif self.current_char == ':':
                self.__advance()
                if len(tokens) == 0:
                    return None, "No label name given, only found ':'."

                name: Token = tokens[-1]
                
                if name.type_ != TokenType.IDENTIFIER:
                    return None, f"Expected Identifier, found '{name.value}'!"

                name.type_ = TokenType.LABEL

                tokens[-1] = name
            elif self.current_char == '"':
                self.__advance()
                tokens.append(self.__make_string())
            elif self.current_char == '&':
                self.__advance()
                ref = self.__make_identifier_or_keyword_or_register_or_instruction()

                if ref.type_ != TokenType.IDENTIFIER:
                    return None, f"Expected Identifier, found '{name.value}'"

                ref.type_ = TokenType.REFERENCE

                tokens.append(ref)
            elif self.current_char == '*':
                self.__advance()
                pointer = self.__make_identifier_or_keyword_or_register_or_instruction()

                if pointer.type_ != TokenType.IDENTIFIER:
                    return None, f"Expected Identifier, found '{name.value}'"

                pointer.type_ = TokenType.POINTER

                tokens.append(pointer)
            elif self.current_char == '>':
                self.__advance()
                pointer = self.__make_identifier_or_keyword_or_register_or_instruction()

                if pointer.type_ != TokenType.IDENTIFIER:
                    return None, f"Expected Identifier, found '{name.value}'"

                pointer.type_ = TokenType.OFFSET

                tokens.append(pointer)
            elif self.current_char in whitespace:
                self.__advance()
            elif self.current_char in ascii_letters + "_":
                tokens.append(self.__make_identifier_or_keyword_or_register_or_instruction())
            else:
                self.__advance()

        return tokens, None

    def __make_number(self) -> Token:
        num = ""

        while self.current_char is not None and self.current_char in hexdigits:
            num += self.current_char
            self.__advance()

        return Token(TokenType.INT, num)

    def __make_identifier_or_keyword_or_register_or_instruction(self) -> Token:
        word = ""

        while self.current_char is not None and self.current_char in ascii_letters + "_":
            word += self.current_char
            self.__advance()

        if word in DATA['keywords']:
            return Token(TokenType.KEYWORD, word)
        elif word in DATA['registers']:
            return Token(TokenType.REGISTER, word)
        elif word in DATA['instructions']:
            return Token(TokenType.INSTRUCTION, word)
        else:
            return Token(TokenType.IDENTIFIER, word)
        
    def __make_string(self) -> Token:
        string = ""
        escaped = False

        while self.current_char is not None:
            if escaped:
                string += self.current_char
                escaped = False
                continue

            if self.current_char == '"':
                self.__advance()
                break

            string += self.current_char
            self.__advance()

        return Token(TokenType.STRING, string)

class Assembler:
    def __init__(self, tokens: list[Token]):
        self.tokens = tokens
        self.position = -1
        self.token = None
        self.__advance()

        self.address = 0
        self.labels = {}

    def __advance(self):
        self.position += 1
        self.token = self.tokens[self.position] if self.position < len(self.tokens) else None

    def assemble(self):
        data = [0x00 for _ in range(0x7FFF + 1)]
        while self.token is not None:
            match self.token.type_:
                case TokenType.DIRECTIVE:
                    if self.token.value.upper() == "ORG":
                        self.__advance()

                        if self.token.type_ != TokenType.ADDRESS:
                            raise TypeError(f"Expecting Address, not '{self.token.type_}'")
                        
                        self.address = int(self.token.value, 16)
                        self.__advance()
                    elif self.token.value.upper() == "BYTE":
                        self.__advance()
                        
                        if self.token.type_ != TokenType.INT:
                            raise TypeError(f"Expecting Int, not '{self.token.type_}'")
                        
                        data[self.address] = int(self.token.value, 16)
                        self.address += 1
                        self.__advance()
                    elif self.token.value.upper() == "WORD":
                        self.__advance()
                        
                        match self.token.type_:
                            case TokenType.ADDRESS:
                                value = int(self.token.value, 16)
                        
                                data[self.address] = (value & 0xFF00) >> 8
                                data[self.address + 1] = value & 0xFF
                                self.address += 2
                                self.__advance()
                            case TokenType.IDENTIFIER:
                                data[self.address] = self.token
                                data[self.address + 1] = None
                                self.address += 2
                                self.__advance()
                            case _:
                                raise TypeError(f"Expecting Address or Identifier, not '{self.token.type_}'")
                    elif self.token.value.upper() == "ASCII":
                        self.__advance()

                        if self.token.type_ != TokenType.STRING:
                            raise TypeError(f"Expecting String, not '{self.token.type_}'")
                        
                        for char in self.token.value:
                            data[self.address] = ord(char)
                            self.address += 1

                        self.__advance()
                    elif self.token.value.upper() == "ASCIIZ":
                        self.__advance()

                        if self.token.type_ != TokenType.STRING:
                            raise TypeError(f"Expecting String, not '{self.token.type_}'")
                        
                        for char in self.token.value:
                            data[self.address] = ord(char)
                            self.address += 1
                        
                        data[self.address] = 0x00
                        self.address += 1

                        self.__advance()
                case TokenType.LABEL:
                    self.labels[self.token.value] = self.address
                    self.__advance()
                case TokenType.INSTRUCTION:
                    instruction_info = DATA["instructions"][self.token.value]
                    
                    match instruction_info['size']:
                        case 0:
                            data[self.address] = instruction_info['opcode']
                            data[self.address + 1] = 0x3F
                            self.address += 2
                            self.__advance()
                        case 1:
                            self.__advance()

                            match self.token.type_:
                                case TokenType.REGISTER:
                                    data[self.address] = instruction_info['opcode']['R']
                                    data[self.address + 1] = DATA["reg_lookup"][self.token.value[1]]
                                    self.address += 2
                                    self.__advance()
                                case TokenType.INT:
                                    data[self.address] = instruction_info['opcode']['I']
                                    data[self.address + 1] = 0x3F
                                    data[self.address + 2] = int(self.token.value, 16)
                                    self.address += 3
                                    self.__advance()
                                case TokenType.ADDRESS:
                                    data[self.address] = instruction_info['opcode']['A']
                                    data[self.address + 1] = 0x3F
                                    
                                    value = int(self.token.value, 16)

                                    data[self.address + 2] = (value & 0xFF00) >> 8
                                    data[self.address + 3] = value & 0xFF

                                    if value > 0xFFFF:
                                        data[self.address + 1] |= (value & 0x30000) >> 10

                                    self.address += 4
                                    self.__advance()
                                case TokenType.IDENTIFIER:
                                    data[self.address] = instruction_info['opcode']['A']
                                    data[self.address + 1] = 0x3F
                                    
                                    if self.token.value not in self.labels:
                                        data[self.address + 2] = self.token
                                        data[self.address + 3] = None
                                        
                                        self.address += 4
                                        self.__advance()
                                        continue

                                    value = self.labels[self.token.value]
                                    
                                    data[self.address + 2] = (value & 0xFF00) >> 8
                                    data[self.address + 3] = value & 0xFF

                                    if value > 0xFFFF:
                                        data[self.address + 1] |= (value & 0x30000) >> 10

                                    self.address += 4
                                    self.__advance()
                        case 2:
                            self.__advance()

                            if self.token.type_ != TokenType.REGISTER:
                                raise TypeError(f"Expecting Register, not '{self.token.type_}'")
                            
                            metadata = DATA["reg_lookup"][self.token.value[1]]

                            self.__advance()

                            match self.token.type_:
                                case TokenType.REGISTER:
                                    data[self.address] = instruction_info['opcode']['RR']
                                    data[self.address + 1] = metadata | (DATA["reg_lookup"][self.token.value[1]] << 3)
                                    self.address += 2
                                    self.__advance()
                                case TokenType.INT:
                                    data[self.address] = instruction_info['opcode']['RI']
                                    data[self.address + 1] = metadata
                                    data[self.address + 2] = int(self.token.value, 16)
                                    self.address += 3
                                    self.__advance()
                                case TokenType.ADDRESS:
                                    data[self.address] = instruction_info['opcode']['RA']
                                    data[self.address + 1] = metadata
                                    
                                    value = int(self.token.value, 16)

                                    data[self.address + 2] = (value & 0xFF00) >> 8
                                    data[self.address + 3] = value & 0xFF

                                    if value > 0xFFFF:
                                        data[self.address + 1] |= (value & 0x30000) >> 10

                                    self.address += 4
                                    self.__advance()
                                case TokenType.IDENTIFIER:
                                    data[self.address] = instruction_info['opcode']['RA']
                                    data[self.address + 1] = metadata
                                    
                                    if self.token.value not in self.labels:
                                        data[self.address + 2] = self.token
                                        data[self.address + 3] = None
                                        
                                        self.address += 4
                                        self.__advance()
                                        continue

                                    value = self.labels[self.token.value]
                                    
                                    data[self.address + 2] = (value & 0xFF00) >> 8
                                    data[self.address + 3] = value & 0xFF

                                    if value > 0xFFFF:
                                        data[self.address + 1] |= (value & 0x30000) >> 10

                                    self.address += 4
                                    self.__advance()
                case _:
                    self.__advance()

        return data
    
if __name__ == "__main__":
    print(sys.argv)
    sys.argv.pop(0)

    with open(sys.argv[0], "r") as f:
        text = f.read()

    lp = LexerParser(text)
    res, error = lp.tokenize()

    if error:
        print(error)
        exit(1)

    for item in res:
        print(item)

    assembler = Assembler(res)
    data = assembler.assemble()

    print(hex(assembler.address))
    print(assembler.labels)

    post_op = []

    for i, item in enumerate(data):
        if isinstance(item, Token):
            if item.type_ != TokenType.IDENTIFIER:
                raise TypeError("Unknown Token in return data.")
            
            if item.value not in assembler.labels:
                raise ValueError(f"Label '{item.value}' not found in labels.")
            
            address = assembler.labels[item.value]

            post_op.append((address & 0xFF00) >> 8)
            post_op.append(address & 0xFF)
            continue
        elif item is None:
            if len(post_op) == i + 1:
                continue

            raise ValueError(f"Unknown None at position {i}")
            
        post_op.append(item)
    
    with open(sys.argv[0].replace(".8do", ".bin"), "wb") as f:
        f.write(bytes(post_op))