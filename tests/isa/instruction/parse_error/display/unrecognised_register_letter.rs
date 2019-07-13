use pir_8_emu::isa::instruction::ParseInstructionError;


static REGISTERS: [char; 8] = ['F', 'S', 'X', 'Y', 'A', 'B', 'C', 'D'];


#[test]
fn msg_left_listspec_left_expected_left() {
    for letter in "QWERTYUIOP".chars() {
        let mut exp = format!("Register {} not found; expected: F, S, X, Y, A, B, C, D ^", letter);
        for i in 0..100 {
            assert_eq!(ParseInstructionError::UnrecognisedRegisterLetter(20 + "; expected: ".len() + 22 + 1 + i, letter, REGISTERS).to_string(),
                       exp,
                       "{}",
                       i);
            exp = format!(" {}", exp);
        }
    }
}

#[test]
fn msg_left_listspec_left_expected_right() {
    for letter in "QWERTYUIOP".chars() {
        let mut exp = format!("Register {} not found; expected: ^ F, S, X, Y, A, B, C, D", letter);
        for i in (20 + "; expected: ".len() + 1)..(20 + "; expected: ".len() + 22 + 1) {
            assert_eq!(ParseInstructionError::UnrecognisedRegisterLetter(i, letter, REGISTERS).to_string(),
                       exp,
                       "{}",
                       i);
            exp = format!(" {}", exp);
        }
    }
}

#[test]
fn msg_left_listspec_right_expected_right() {
    for letter in "QWERTYUIOP".chars() {
        let mut exp = format!("Register {} not found ^ expected: F, S, X, Y, A, B, C, D", letter);
        for i in (20 + 1)..(20 + "; expected: ".len() + 1) {
            assert_eq!(ParseInstructionError::UnrecognisedRegisterLetter(i, letter, REGISTERS).to_string(),
                       exp,
                       "{}",
                       i);
            exp = format!(" {}", exp);
        }
    }
}

// msg_right_listspec_left_expected_left:
// for i in ("Expected: ".len() + 22 + 1)..(20 + 1) {

#[test]
fn msg_right_listspec_right_expected_left() {
    for letter in "QWERTYUIOP".chars() {
        let mut exp = format!("F, S, X, Y, A, B, C, D ^ <- expected: Register {} not found", letter);
        for i in (22 + 1)..(20 + 1) {
            assert_eq!(ParseInstructionError::UnrecognisedRegisterLetter(i, letter, REGISTERS).to_string(),
                       exp,
                       "{}",
                       i);
            exp = format!(" {}", exp);
        }
    }
}

/// Never occurs, "Register Q not found" is shorter than "F, S, X, Y, A, B, C, D" so will always take priority
#[test]
fn msg_right_listspec_right_expected_right() {
    for letter in "QWERTYUIOP".chars() {
        for i in 0..(22 + 1) {
            assert!(!ParseInstructionError::UnrecognisedRegisterLetter(i, letter, REGISTERS).to_string().starts_with("^ Expected"),
                    "{}",
                    i);
        }
    }
}
