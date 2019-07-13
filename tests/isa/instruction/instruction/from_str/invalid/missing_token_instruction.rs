use pir_8_emu::isa::instruction::ParseInstructionError;
use self::super::missing_token;


#[test]
fn alu() {
    static TOKENS_ALU: &[&str] = &["ADD", "SUB", "NOT", "OR", "XOR", "AND", "SOR", "[raw operation literal]"];

    missing_token("ALU", |len, _| ParseInstructionError::MissingToken(len, TOKENS_ALU));
}

#[test]
fn alu_sor() {
    static TOKENS_SOR: &[&str] = &["LEFT", "RIGHT"];

    for pad in 1..5 {
        missing_token(&format!("ALU{e:w$}SOR", e = "", w = pad),
                      |len, _| ParseInstructionError::MissingToken(len, TOKENS_SOR));
    }
}

#[test]
fn alu_sor_type() {
    static TOKENS_SOR_TYPE: &[&str] = &["LSF", "ASF", "RTC", "RTW"];

    for dir in &["LEFT", "RIGHT"] {
        for pad_left in 1..5 {
            for pad_right in 1..5 {
                missing_token(&format!("ALU{e:wl$}SOR{e:wr$}{}", dir, e = "", wl = pad_left, wr = pad_right),
                              |len, _| ParseInstructionError::MissingToken(len, TOKENS_SOR_TYPE));
            }
        }
    }
}

#[test]
fn load() {
    static TOKENS_LOAD: &[&str] = &["IMM", "IND"];

    missing_token("LOAD", |len, _| ParseInstructionError::MissingToken(len, TOKENS_LOAD));
}

#[test]
fn port() {
    static TOKENS_PORT: &[&str] = &["IN", "OUT"];

    missing_token("PORT", |len, _| ParseInstructionError::MissingToken(len, TOKENS_PORT));
}

#[test]
fn stck() {
    static TOKENS_STCK: &[&str] = &["PUSH", "POP"];

    missing_token("STCK", |len, _| ParseInstructionError::MissingToken(len, TOKENS_STCK));
}

#[test]
fn stck_register_pair() {
    static TOKENS_STCK_REG_PAIR: &[&str] = &["A&B", "C&D"];

    for d in &["PUSH", "POP"] {
        for pad in 1..5 {
            missing_token(&format!("STCK{e:w$}{}", d, e = "", w = pad),
                          |len, _| ParseInstructionError::MissingToken(len, TOKENS_STCK_REG_PAIR));
        }
    }
}
