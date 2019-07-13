use pir_8_emu::isa::instruction::ParseInstructionError;
use self::super::unrecognised_token;


#[test]
fn raw() {
    for i in 0..=0b1111 {
        unrecognised_token(&format!("{}", i), &[], 1..5, |_| true, |len, _| ParseInstructionError::TooManyTokens(len));
        unrecognised_token(&format!("{:#0x}", i), &[], 1..5, |_| true, |len, _| ParseInstructionError::TooManyTokens(len));
        unrecognised_token(&format!("{:#0X}", i), &[], 1..5, |_| true, |len, _| ParseInstructionError::TooManyTokens(len));
        unrecognised_token(&format!("{:#0o}", i), &[], 1..5, |_| true, |len, _| ParseInstructionError::TooManyTokens(len));
        unrecognised_token(&format!("{:#0b}", i), &[], 1..5, |_| true, |len, _| ParseInstructionError::TooManyTokens(len));
    }
}

#[test]
fn toplevel() {
    static TOKENS_ALU: &[&str] = &["ADD", "SUB", "NOT", "OR", "XOR", "AND"];

    for tok in TOKENS_ALU {
        unrecognised_token(tok, &[], 1..5, |_| true, |len, _| ParseInstructionError::TooManyTokens(len));
    }
}

#[test]
fn sor() {
    static TOKENS_SOR_DIRECTION: &[&str] = &["LEFT", "RIGHT"];
    static TOKENS_SOR_TYPE: &[&str] = &["LSF", "ASF", "RTC", "RTW"];

    for d in TOKENS_SOR_DIRECTION {
        for tt in TOKENS_SOR_TYPE {
            for pad_left in 1..3 {
                for pad_right in 1..3 {
                    unrecognised_token(&format!("SOR{e:wl$}{}{e:wr$}{}", d, tt, e = "", wl = pad_left, wr = pad_right),
                                       &[],
                                       1..5,
                                       |_| true,
                                       |len, _| ParseInstructionError::TooManyTokens(len));
                }
            }
        }
    }
}
