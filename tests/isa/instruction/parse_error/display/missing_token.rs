use pir_8_emu::isa::instruction::ParseInstructionError;


static TOKENS: &[&str] = &["AAA", "BBB"];
static STOKENS: &[&str] = &["A", "B"];


#[test]
fn msg_left_listspec_left_expected_left() {
    let mut exp = "Missing token; expected: AAA, BBB ^".to_string();
    for i in 0..100 {
        assert_eq!(ParseInstructionError::MissingToken(13 + "; expected: ".len() + 8 + 1 + i, TOKENS).to_string(),
                   exp,
                   "{}",
                   i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_left_listspec_left_expected_right() {
    let mut exp = "Missing token; expected: ^ AAA, BBB".to_string();
    for i in (13 + "; expected: ".len() + 1)..(13 + "; expected: ".len() + 8 + 1) {
        assert_eq!(ParseInstructionError::MissingToken(i, TOKENS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_left_listspec_right_expected_right() {
    let mut exp = "Missing token ^ expected: AAA, BBB".to_string();
    for i in (13 + 1)..(13 + "; expected: ".len() + 1) {
        assert_eq!(ParseInstructionError::MissingToken(i, TOKENS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_right_listspec_left_expected_left() {
    let mut exp = "Expected: A, B ^ Missing token".to_string();
    for i in ("Expected: ".len() + 4 + 1)..(13 + 1) {
        assert_eq!(ParseInstructionError::MissingToken(i, STOKENS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_right_listspec_right_expected_left() {
    let mut exp = "AAA, BBB ^ <- expected: Missing token".to_string();
    for i in (8 + 1)..(13 + 1) {
        assert_eq!(ParseInstructionError::MissingToken(i, TOKENS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}

#[test]
fn msg_right_listspec_right_expected_right() {
    let mut exp = "^ Missing token; expected: AAA, BBB".to_string();
    for i in 0..(8 + 1) {
        assert_eq!(ParseInstructionError::MissingToken(i, TOKENS).to_string(), exp, "{}", i);
        exp = format!(" {}", exp);
    }
}
