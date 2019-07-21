use pir_8_emu::isa::instruction::ParseInstructionError;
use self::super::missing_token;


#[test]
fn origin() {
    static TOKENS_ORIGIN: &[&str] = &["[16-bit origin address]"];

    missing_token("origin", |len| ParseInstructionError::MissingToken(len, TOKENS_ORIGIN));
}

#[test]
fn label() {
    static TOKENS_LABEL: &[&str] = &["save", "load"];

    missing_token("label", |len| ParseInstructionError::MissingToken(len, TOKENS_LABEL));
}

#[test]
fn label_name() {
    static TOKENS_SOR_TYPE: &[&str] = &["[label name]"];

    for op in &["save", "load"] {
        for pad in 1..5 {
            missing_token(&format!("label{e:w$}{}", op, e = "", w = pad),
                          |len| ParseInstructionError::MissingToken(len, TOKENS_SOR_TYPE));
        }
    }
}
