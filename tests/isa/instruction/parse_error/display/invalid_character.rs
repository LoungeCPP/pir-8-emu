use pir_8_emu::isa::instruction::ParseInstructionError;


#[test]
fn left() {
    let mut exp = "Invalid (non-ASCII/vertical-WS) character ^".to_string();
    for i in 0..100 {
        assert_eq!(ParseInstructionError::InvalidCharacter(41 + 1 + 1 + i).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn right() {
    let mut exp = "^ Invalid (non-ASCII/vertical-WS) character".to_string();
    for i in 0..41 + 1 + 1 {
        assert_eq!(ParseInstructionError::InvalidCharacter(i).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}
