use pir_8_emu::isa::instruction::ParseInstructionError;


static REGISTERS: [char; 8] = ['F', 'S', 'X', 'Y', 'A', 'B', 'C', 'D'];


#[test]
fn msg_left_listspec_left_expected_left() {
    let mut exp = "Missing register letter; expected: F, S, X, Y, A, B, C, D ^".to_string();
    for i in 0..100 {
        assert_eq!(ParseInstructionError::MissingRegisterLetter(23 + "; expected: ".len() + 22 + 1 + i, REGISTERS).to_string(),
                   exp,
                   "{}",
                   i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_left_listspec_left_expected_right() {
    let mut exp = "Missing register letter; expected: ^ F, S, X, Y, A, B, C, D".to_string();
    for i in (23 + "; expected: ".len() + 1)..(23 + "; expected: ".len() + 22 + 1) {
        assert_eq!(ParseInstructionError::MissingRegisterLetter(i, REGISTERS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_left_listspec_right_expected_right() {
    let mut exp = "Missing register letter ^ expected: F, S, X, Y, A, B, C, D".to_string();
    for i in (23 + 1)..(23 + "; expected: ".len()) {
        assert_eq!(ParseInstructionError::MissingRegisterLetter(i, REGISTERS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

// msg_right_listspec_left_expected_left:
// ("Expected: ".len() + 22 + 1)..(23 + 1)

#[test]
fn msg_right_listspec_right_expected_left() {
    let mut exp = "F, S, X, Y, A, B, C, D ^ <- expected: Missing register letter".to_string();
    for i in (22 + 1)..(23 + 1) {
        assert_eq!(ParseInstructionError::MissingRegisterLetter(i, REGISTERS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_right_listspec_right_expected_right() {
    let mut exp = "^ Missing register letter; expected: F, S, X, Y, A, B, C, D".to_string();
    for i in 0..(22 + 1) {
        assert_eq!(ParseInstructionError::MissingRegisterLetter(i, REGISTERS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}
