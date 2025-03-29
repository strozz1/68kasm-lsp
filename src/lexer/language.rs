static MOVE_INST: &[&str] = &[
    "MOVE", "MOVE.L", "MOVE.W", "MOVE.B", "MOVEA", "MOVEQ", "MOVEM", "MOVEM.B", "MOVEM.W",
    "MOVEM.L", "MOVEP",
];
static BRANCH_INST: &[&str] = &[
    "BREAK", "BEQ", "BGE", "BGT", "BHI", "BHS", "BIL", "BLT", "BMI", "BNE", "BPL", "BRA", "BSR",
    "JMP", "JSR", "BLE",
];
static LOGIC_INST: &[&str] = &[
    "AND", "ANDI", "XOR", "XORI", "EQU", "RTR", "RTE", "RTS", "CMP", "CMP.B", "CMP.W", "CMP.L",
    "CMPI", "NEG", "CMPA", "CMPM", "EOR", "EORI",
];
static ARITM_INST: &[&str] = &[
    "ADD", "ADD.L", "ADD.W", "ADD.B", "ADDI", "ADCS", "SUB", "SUB.B", "SUB.W", "SUB.L", "MULS",
    "MULU", "SUBI",
];
static DATA_INST: &[&str] = &[
    "ORG", "DC.L", "DC", "DC.W", "DC.B", "DS.L", "DS", "DS.W", "DS.B",
];
static MISC_ISNT: &[&str] = &[
    "CALL", "LINK", "UNLK", "TRAP", "CLR", "NOP", "ABCD", "ASL", "ASR", "Bcc", "CHK", "DIVS",
    "DIVU", "EXG", "LEA", "LEAU", "LSL", "LSR", "OR", "ORI", "ROXL", "ROXR", "SBCD", "SCC", "SEQ",
    "SEQ", "SGT", "SLE", "SLS", "SNE", "SPL", "SRO", "STO", "SWAP", "TAS", "TRAPV", "TST","BCLR"
];

pub fn is_instruction(raw: &str) -> Result<InstructionSpecs, &str> {
    for &i in MOVE_INST.iter() {
        if raw == i {
            return Ok(InstructionSpecs {
                instr: i,
                needs_size: false,
                valid_operands: Vec::new(),
            });
        }
    }

    for &i in ARITM_INST.iter() {
        if raw == i {
            return Ok(InstructionSpecs {
                instr: i,
                needs_size: false,
                valid_operands: Vec::new(),
            });
        }
    }
    for &i in BRANCH_INST.iter() {
        if raw == i {
            return Ok(InstructionSpecs {
                instr: i,
                needs_size: false,
                valid_operands: Vec::new(),
            });
        }
    }
    for &i in LOGIC_INST.iter() {
        if raw == i {
            return Ok(InstructionSpecs {
                instr: i,
                needs_size: false,
                valid_operands: Vec::new(),
            });
        }
    }

    for &i in DATA_INST.iter() {
        if raw == i {
            return Ok(InstructionSpecs {
                instr: i,
                needs_size: false,
                valid_operands: Vec::new(),
            });
        }
    }
    for &i in MISC_ISNT.iter() {
        if raw == i {
            return Ok(InstructionSpecs {
                instr: i,
                needs_size: false,
                valid_operands: Vec::new(),
            });
        }
    }

    return Err("not a valid instruction");
}

#[derive(Debug,Clone)]
pub struct InstructionSpecs {
    instr: &'static str,
    needs_size: bool,
    valid_operands: Vec<String>,
}
