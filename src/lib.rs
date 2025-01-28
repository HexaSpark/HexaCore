pub mod cpu;
pub mod device;

const INST_INFO: &str = "{\"opcodes\":{\"86\":\"andb|A\",\"59\":\"mov|I\",\"115\":\"sbl|R\",\"63\":\"rol|R\",\"181\":\"sblb|R\",\"26\":\"bin|A\",\"155\":\"bio|A\",\"50\":\"cmp|R\",\"20\":\"and|A\",\"157\":\"incb|A\",\"118\":\"pshb|I\",\"40\":\"sbr|A\",\"96\":\"decb|R\",\"30\":\"dec|R\",\"58\":\"xor|A\",\"31\":\"bnn|A\",\"123\":\"sbrb|R\",\"32\":\"bno|A\",\"56\":\"bng|A\",\"29\":\"bnl|A\",\"180\":\"bnc|A\",\"51\":\"mov|A\",\"44\":\"orb|I\",\"18\":\"add|I\",\"94\":\"andb|I\",\"107\":\"cmpb|I\",\"109\":\"subb|A\",\"125\":\"movb|I\",\"234\":\"or|I\",\"75\":\"xor|R\",\"53\":\"orb|R\",\"99\":\"cmpb|A\",\"150\":\"rorb|A\",\"117\":\"movb|A\",\"84\":\"addb|I\",\"36\":\"orb|A\",\"132\":\"xorb|I\",\"112\":\"rolb|A\",\"27\":\"add|R\",\"229\":\"clv|M\",\"127\":\"pshb|R\",\"42\":\"stb|A\",\"76\":\"addb|A\",\"33\":\"cmp|A\",\"69\":\"ror|R\",\"148\":\"ror|A\",\"52\":\"psh|I\",\"61\":\"psh|R\",\"210\":\"clc|M\",\"79\":\"decb|A\",\"216\":\"cli|M\",\"72\":\"jmp|A\",\"60\":\"sub|R\",\"139\":\"bnz|A\",\"28\":\"and|I\",\"103\":\"andb|R\",\"226\":\"or|A\",\"37\":\"and|R\",\"129\":\"rolb|R\",\"124\":\"xorb|A\",\"225\":\"sei|M\",\"108\":\"inc|R\",\"135\":\"rorb|R\",\"43\":\"sub|A\",\"38\":\"biz|A\",\"8\":\"hlt|M\",\"134\":\"movb|R\",\"83\":\"sub|I\",\"68\":\"mov|R\",\"126\":\"subb|R\",\"13\":\"dec|A\",\"34\":\"sbl|A\",\"15\":\"bic|A\",\"24\":\"bil|A\",\"224\":\"in|I\",\"93\":\"addb|R\",\"110\":\"incb|R\",\"106\":\"sbrb|A\",\"232\":\"st|A\",\"19\":\"big|A\",\"65\":\"out|I\",\"10\":\"add|A\",\"249\":\"rts|M\",\"116\":\"cmpb|R\",\"91\":\"inc|A\",\"141\":\"xorb|R\",\"46\":\"rol|A\",\"66\":\"xor|I\",\"57\":\"sbr|R\",\"243\":\"or|R\",\"100\":\"sblb|A\",\"48\":\"jsr|A\",\"149\":\"subb|I\",\"41\":\"cmp|I\"},\"info\":[{\"name\":\"movb\",\"size\":2,\"opcode\":{\"RR\":134,\"RI\":125,\"RA\":117},\"byte\":true},{\"name\":\"mov\",\"size\":2,\"opcode\":{\"RA\":51,\"RR\":68,\"RI\":59},\"byte\":false},{\"name\":\"stb\",\"size\":2,\"opcode\":{\"RA\":42},\"byte\":true},{\"name\":\"st\",\"size\":2,\"opcode\":{\"RA\":232},\"byte\":false},{\"name\":\"andb\",\"size\":2,\"opcode\":{\"RA\":86,\"RI\":94,\"RR\":103},\"byte\":true},{\"name\":\"and\",\"size\":2,\"opcode\":{\"RA\":20,\"RI\":28,\"RR\":37},\"byte\":false},{\"name\":\"orb\",\"size\":2,\"opcode\":{\"RA\":36,\"RI\":44,\"RR\":53},\"byte\":true},{\"name\":\"or\",\"size\":2,\"opcode\":{\"RI\":234,\"RA\":226,\"RR\":243},\"byte\":false},{\"name\":\"xorb\",\"size\":2,\"opcode\":{\"RI\":132,\"RA\":124,\"RR\":141},\"byte\":true},{\"name\":\"xor\",\"size\":2,\"opcode\":{\"RR\":75,\"RA\":58,\"RI\":66},\"byte\":false},{\"name\":\"pshb\",\"size\":1,\"opcode\":{\"I\":118,\"R\":127},\"byte\":true},{\"name\":\"psh\",\"size\":1,\"opcode\":{\"I\":52,\"R\":61},\"byte\":false},{\"name\":\"addb\",\"size\":2,\"opcode\":{\"RI\":84,\"RA\":76,\"RR\":93},\"byte\":true},{\"name\":\"add\",\"size\":2,\"opcode\":{\"RI\":18,\"RR\":27,\"RA\":10},\"byte\":false},{\"name\":\"subb\",\"size\":2,\"opcode\":{\"RR\":126,\"RA\":109,\"RI\":149},\"byte\":true},{\"name\":\"sub\",\"size\":2,\"opcode\":{\"RI\":83,\"RA\":43,\"RR\":60},\"byte\":false},{\"name\":\"cmpb\",\"size\":2,\"opcode\":{\"RA\":99,\"RI\":107,\"RR\":116},\"byte\":true},{\"name\":\"cmp\",\"size\":2,\"opcode\":{\"RA\":33,\"RI\":41,\"RR\":50},\"byte\":false},{\"name\":\"incb\",\"size\":1,\"opcode\":{\"R\":110,\"A\":157},\"byte\":true},{\"name\":\"inc\",\"size\":1,\"opcode\":{\"R\":108,\"A\":91},\"byte\":false},{\"name\":\"decb\",\"size\":1,\"opcode\":{\"A\":79,\"R\":96},\"byte\":true},{\"name\":\"dec\",\"size\":1,\"opcode\":{\"R\":30,\"A\":13},\"byte\":false},{\"name\":\"sblb\",\"size\":1,\"opcode\":{\"A\":100,\"R\":181},\"byte\":true},{\"name\":\"sbl\",\"size\":1,\"opcode\":{\"R\":115,\"A\":34},\"byte\":false},{\"name\":\"sbrb\",\"size\":1,\"opcode\":{\"A\":106,\"R\":123},\"byte\":true},{\"name\":\"sbr\",\"size\":1,\"opcode\":{\"R\":57,\"A\":40},\"byte\":false},{\"name\":\"rolb\",\"size\":1,\"opcode\":{\"A\":112,\"R\":129},\"byte\":true},{\"name\":\"rol\",\"size\":1,\"opcode\":{\"R\":63,\"A\":46},\"byte\":false},{\"name\":\"rorb\",\"size\":1,\"opcode\":{\"A\":150,\"R\":135},\"byte\":true},{\"name\":\"ror\",\"size\":1,\"opcode\":{\"R\":69,\"A\":148},\"byte\":false},{\"name\":\"clc\",\"size\":0,\"opcode\":{\"M\":210},\"byte\":false},{\"name\":\"cli\",\"size\":0,\"opcode\":{\"M\":216},\"byte\":false},{\"name\":\"clv\",\"size\":0,\"opcode\":{\"M\":229},\"byte\":false},{\"name\":\"sei\",\"size\":0,\"opcode\":{\"M\":225},\"byte\":false},{\"name\":\"jmp\",\"size\":1,\"opcode\":{\"A\":72},\"byte\":false},{\"name\":\"jsr\",\"size\":1,\"opcode\":{\"A\":48},\"byte\":false},{\"name\":\"biz\",\"size\":1,\"opcode\":{\"A\":38},\"byte\":false},{\"name\":\"bin\",\"size\":1,\"opcode\":{\"A\":26},\"byte\":false},{\"name\":\"bic\",\"size\":1,\"opcode\":{\"A\":15},\"byte\":false},{\"name\":\"bio\",\"size\":1,\"opcode\":{\"A\":155},\"byte\":false},{\"name\":\"bil\",\"size\":1,\"opcode\":{\"A\":24},\"byte\":false},{\"name\":\"big\",\"size\":1,\"opcode\":{\"A\":19},\"byte\":false},{\"name\":\"bnz\",\"size\":1,\"opcode\":{\"A\":139},\"byte\":false},{\"name\":\"bnn\",\"size\":1,\"opcode\":{\"A\":31},\"byte\":false},{\"name\":\"bnc\",\"size\":1,\"opcode\":{\"A\":180},\"byte\":false},{\"name\":\"bno\",\"size\":1,\"opcode\":{\"A\":32},\"byte\":false},{\"name\":\"bnl\",\"size\":1,\"opcode\":{\"A\":29},\"byte\":false},{\"name\":\"bng\",\"size\":1,\"opcode\":{\"A\":56},\"byte\":false},{\"name\":\"rts\",\"size\":0,\"opcode\":{\"M\":249},\"byte\":false},{\"name\":\"in\",\"size\":2,\"opcode\":{\"RI\":224},\"byte\":true},{\"name\":\"out\",\"size\":2,\"opcode\":{\"RI\":65},\"byte\":true},{\"name\":\"hlt\",\"size\":0,\"opcode\":{\"M\":8},\"byte\":false}]}";

pub mod info {
    use std::{collections::HashMap, fs::File};

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug)]
    #[derive(Serialize, Deserialize)]
    pub struct InstructionInfo {
        pub name: String,
        pub size: u8,
        pub opcode: HashMap<String, u8>,
        pub byte: bool
    }

    #[derive(Serialize, Deserialize, Default)]
    pub struct InstructionInfoFile {
        pub opcodes: HashMap<u8, String>,
        pub info: Vec<InstructionInfo>
    }

    impl InstructionInfo {
        pub fn new(name: impl Into<String>, size: u8, opcode: HashMap<String, u8>, byte: bool) -> Self {
            Self {
                name: name.into(),
                size,
                opcode,
                byte
            }
        }
    }

    pub fn get_instructions() -> InstructionInfoFile {
        serde_json::from_str(crate::INST_INFO).unwrap()
    }

    pub fn get_instruction_info(insts: &[InstructionInfo], instruction: &str) -> Option<InstructionInfo> {
        let inst = insts.iter().find(|&x| x.name == instruction);

        inst?;

        Some(inst.unwrap().clone())
    }
}