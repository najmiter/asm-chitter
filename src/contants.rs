use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Tokens {
    pub name: String,
    pub class: String,
    pub content: String,
}

pub fn get_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();

    styles.insert("plain".to_string(), "color: #c8d1d9;".to_string());
    styles.insert("jumps".to_string(), "color: #c79af1;".to_string());
    styles.insert("constant".to_string(), "color: #79c0ff;".to_string());
    styles.insert("citicals".to_string(), "color: #f47067;".to_string());
    styles.insert("operators".to_string(), "color: #a492f7;".to_string());
    styles.insert("datatypes".to_string(), "color: #8edb8c;".to_string());
    styles.insert("arithmetics".to_string(), "color: yellowgreen;".to_string());
    styles.insert(
        "comment".to_string(),
        "color: #6b6b6b; font-style: italic;".to_string(),
    );
    styles.insert(
        "registers".to_string(),
        "color: #fdad54; font-style: italic;".to_string(),
    );
    styles.insert(
        "instructions".to_string(),
        "color: #ec8e2b; font-weight: bold;".to_string(),
    );
    styles.insert(
        "function-label".to_string(),
        "color: #d2a8ff; font-weight: bold;".to_string(),
    );

    styles
}

pub fn asm_data() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();
    map.insert(
        "operators",
        vec![
            "+", "-", "/", "*", "(", ")", "[", "]", "\"", "'", ",", ".", "%", "=", "==", "<", ">",
            "!", "!=", "<=", ">=", "+=", "-=", "*=", "/=", "&", "&&", "|", "||", "^", "~",
        ],
    );
    map.insert(
        "arithmetics",
        vec![
            "ADD", "SUB", "INC", "DEC", "MUL", "IMUL", "DIV", "IDIV", "AND", "OR", "XOR", "NOT",
            "SHL", "SHR",
        ],
    );
    map.insert(
        "criticals",
        vec!["RET", "PROC", "ENDP", "END", "INCLUDE", "SECTION"],
    );
    map.insert(
        "instructions",
        vec![
            "MOV", "MOVS", "MOVSX", "MOVZX", "CMP", "PUSH", "POP", "PUSHAD", "POPAD", "LEA", "NOP",
            "HLT", "INT", "LEAVE", "CLC", "STC", "CLD", "STD", "CLI", "STI", "CMPXCHG", "XCHG",
            "BSWAP", "NOP", "PUSHF", "POPF", "REP", "REPE", "REPZ", "REPNE", "REPNZ", "CMC",
            "CWDE", "CDQ", "WAIT", "CBW", "CWD", "INTO", "IRET", "OFFSET", "PTR", "FLD", "FSTP",
            "SYSCALL", "USES", "COMMENT", "EQU", "GLOBAL",
        ],
    );
    map.insert(
        "datatypes",
        vec![
            "BYTE", "WORD", "DWORD", "QWORD", "DB", "DW", "DD", "DQ", "REAL", "RESB", "RESW",
            "RESD", "RESQ",
        ],
    );
    map.insert(
        "jumps",
        vec![
            "JMP", "JE", "JNE", "JG", "JGE", "JL", "JLE", "JZ", "JNZ", "JS", "JNS", "JC", "JNC",
            "JB", "JA", "CALL", "INVOKE",
        ],
    );
    map.insert(
        "registers",
        vec![
            "AL", "BL", "CL", "DL", "AH", "BH", "CH", "DH", "AX", "BX", "CX", "DX", "EAX", "EBX",
            "ECX", "EDX", "RAX", "RBX", "RCX", "RDX", "DI", "SI", "EDI", "ESI", "EBP", "ESP",
            "RBP", "RSP", "RDI", "RSI", "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15",
            "R8B", "R8W", "R8D", "R9B", "R9W", "R9D", "R10B", "R10W", "R10D", "R11B", "R11W",
            "R11D", "R12B", "R12W", "R12D", "R13B", "R13W", "R13D", "R14B", "R14W", "R14D", "R15B",
            "R15W", "R15D",
        ],
    );
    map
}
