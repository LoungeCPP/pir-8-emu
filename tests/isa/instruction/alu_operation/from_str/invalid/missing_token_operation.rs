use pir_8_emu::isa::instruction::ParseInstructionError;
use self::super::missing_token;


#[test]
fn sor() {
    static TOKENS_SOR: &[&str] = &["LEFT", "RIGHT"];

    missing_token("SOR", |len| ParseInstructionError::MissingToken(len, TOKENS_SOR));
}

#[test]
fn sor_type() {
    static TOKENS_SOR_TYPE: &[&str] = &["LSF", "ASF", "RTC", "RTW"];

    for dir in &["LEFT", "RIGHT"] {
        for pad in 1..5 {
            missing_token(&format!("SOR{e:w$}{}", dir, e = "", w = pad),
                          |len| ParseInstructionError::MissingToken(len, TOKENS_SOR_TYPE));
        }
    }
}
