use pir_8_emu::isa::instruction::ParseInstructionError;


#[test]
fn left() {
    let mut exp = "Too many tokens ^".to_string();
    for i in 0..100 {
        assert_eq!(ParseInstructionError::TooManyTokens(15 + 1 + 1 + i).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn right() {
    let mut exp = "^ Too many tokens".to_string();
    for i in 0..15 + 1 + 1 {
        assert_eq!(ParseInstructionError::TooManyTokens(i).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}
