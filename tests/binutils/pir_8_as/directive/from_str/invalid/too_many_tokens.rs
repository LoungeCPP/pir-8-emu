use pir_8_emu::isa::instruction::ParseInstructionError;
use rand::distributions::{Alphanumeric, Distribution};
use self::super::unrecognised_token;
use rand::thread_rng;


#[test]
fn origin() {
    for addr in (0..=0xFF).step_by(16) {
        let addr = addr << 8 | addr;

        unrecognised_token(&format!("origin {}", addr), &[], 1..5, |len, _| ParseInstructionError::TooManyTokens(len));
        unrecognised_token(&format!("origin {:#0x}", addr), &[], 1..5, |len, _| ParseInstructionError::TooManyTokens(len));
        unrecognised_token(&format!("origin {:#0X}", addr), &[], 1..5, |len, _| ParseInstructionError::TooManyTokens(len));
        unrecognised_token(&format!("origin {:#0o}", addr), &[], 1..5, |len, _| ParseInstructionError::TooManyTokens(len));
        unrecognised_token(&format!("origin {:#0b}", addr), &[], 1..5, |len, _| ParseInstructionError::TooManyTokens(len));
    }
}

#[test]
fn save_label() {
    label("save");
}

#[test]
fn load_label() {
    label("load");
}


fn label(op: &str) {
    for pad_left in 1..3 {
        for pad_right in 1..3 {
            for token_len in 1..3 {
                for _ in 0..3 {
                    let label: String = Alphanumeric.sample_iter(thread_rng()).take(token_len).collect();

                    unrecognised_token(&format!("label{e:wl$}{}{e:wr$}{}", op, label, e = "", wl = pad_left, wr = pad_right),
                                       &[],
                                       1..3,
                                       |len, _| ParseInstructionError::TooManyTokens(len));
                }
            }
        }
    }
}
