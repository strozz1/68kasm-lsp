pub fn is_instruction(raw: &str) -> bool {
    let instructions = vec![
        "ABCD", "ADD", "ADD.L", "ADD.W", "ADD.B", "ADDI", "ADCS", "AND", "ANDI", "ASL", "ASR",
        "Bcc", "BEQ", "BGE", "BGT", "BHI", "BHS", "BIL", "BLT", "BMI", "BNE", "BPL", "BRA", "BSR",
        "CALL", "CHK", "CLR", "CMP", "CMP.B","CMP.W","CMP.L","CMPI", "CMPA", "CMPM", "DIVS", "DIVU", "EOR", "EORI", "EXG",
        "JMP", "JSR", "LEA", "LEAU", "LINK", "LSL", "LSR", "MOVE", "MOVE.L", "MOVE.W", "MOVE.B",
        "MOVEA", "MOVEQ", "MOVEM", "MOVEM.B", "MOVEM.W", "MOVEM.L", "MOVEP", "MULS", "MULU", "NEG", "NOP", "OR", "ORI", "ROXL",
        "ROXR", "RTR", "RTE", "RTS", "SBCD", "SCC", "SEQ", "SEQ", "SGT", "SLE", "SLS", "SNE",
        "SPL", "SRO", "STO", "SUBI", "SWAP", "TAS", "TRAP", "ORG", "DC.L", "DC", "DC.W", "DC.B",
        "DS.L", "DS", "DS.W", "DS.B", "TRAPV", "TST", "UNLK", "XOR", "XORI", "EQU", "SUB", "SUB.B",
        "SUB.W", "SUB.L","BREAK","BLE"
    ];
    if instructions.contains(&raw.to_uppercase().as_str()) {
        return true;
    }

    false
}
