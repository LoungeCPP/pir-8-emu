use pir_8_emu::isa::instruction::ParseInstructionError;


static TOKENS: &[&str] = &["AAA", "BBB"];
static STOKENS: &[&str] = &["A", "B"];


#[test]
fn msg_left_listspec_left_expected_left() {
    let mut exp = "Unrecognised token; expected: AAA, BBB ^".to_string();
    for i in 0..100 {
        assert_eq!(ParseInstructionError::UnrecognisedToken(18 + "; expected: ".len() + 8 + 1 + i, TOKENS).to_string(),
                   exp,
                   "{}",
                   i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_left_listspec_left_expected_right() {
    let mut exp = "Unrecognised token; expected: ^ AAA, BBB".to_string();
    for i in (18 + "; expected: ".len() + 1)..(18 + "; expected: ".len() + 8 + 1) {
        assert_eq!(ParseInstructionError::UnrecognisedToken(i, TOKENS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_left_listspec_right_expected_right() {
    let mut exp = "Unrecognised token ^ expected: AAA, BBB".to_string();
    for i in (18 + 1)..(18 + "; expected: ".len() + 1) {
        assert_eq!(ParseInstructionError::UnrecognisedToken(i, TOKENS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_right_listspec_left_expected_left() {
    let mut exp = "Expected: A, B ^ Unrecognised token".to_string();
    for i in ("Expected: ".len() + 4 + 1)..(18 + 1) {
        assert_eq!(ParseInstructionError::UnrecognisedToken(i, STOKENS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_right_listspec_right_expected_left() {
    let mut exp = "AAA, BBB ^ <- expected: Unrecognised token".to_string();
    for i in (8 + 1)..(18 + 1) {
        assert_eq!(ParseInstructionError::UnrecognisedToken(i, TOKENS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_right_listspec_right_expected_right() {
    let mut exp = "^ Unrecognised token; expected: AAA, BBB".to_string();
    for i in 0..(8 + 1) {
        assert_eq!(ParseInstructionError::UnrecognisedToken(i, TOKENS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}
