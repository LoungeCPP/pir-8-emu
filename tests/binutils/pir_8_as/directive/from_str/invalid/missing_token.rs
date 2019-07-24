use pir_8_emu::isa::instruction::ParseInstructionError;
use rand::distributions::{Alphanumeric, Distribution};
use self::super::missing_token;
use rand::thread_rng;


#[test]
fn origin() {
    static TOKENS_ORIGIN: &[&str] = &["[16-bit origin address]"];

    missing_token("origin", |len| ParseInstructionError::MissingToken(len, TOKENS_ORIGIN));
}

#[test]
fn label() {
    static TOKENS_LABEL: &[&str] = &["save", "load", "load-offset"];

    missing_token("label", |len| ParseInstructionError::MissingToken(len, TOKENS_LABEL));
}

#[test]
fn label_name() {
    static TOKENS_LABEL_NAME: &[&str] = &["[label name]"];

    for op in &["save", "load", "load-offset"] {
        for pad in 1..5 {
            missing_token(&format!("label{e:w$}{}", op, e = "", w = pad),
                          |len| ParseInstructionError::MissingToken(len, TOKENS_LABEL_NAME));
        }
    }
}

#[test]
fn label_offset() {
    static TOKENS_LABEL_OFFSET: &[&str] = &["[signed 16-bit label offset]"];

    for pad_left in 1..5 {
        for pad_right in 1..5 {
            for token_len in 1..5 {
                for _ in 0..5 {
                    let token: String = Alphanumeric.sample_iter(thread_rng()).take(token_len).collect();

                    missing_token(&format!("label{e:wl$}load-offset{e:wr$}{}", token, e = "", wl = pad_left, wr = pad_right),
                                  |len| ParseInstructionError::MissingToken(len, TOKENS_LABEL_OFFSET));
                }
            }
        }
    }
}
