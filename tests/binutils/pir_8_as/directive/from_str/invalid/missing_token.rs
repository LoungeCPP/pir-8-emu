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
fn fragment() {
    static TOKENS_FRAGMENT: &[&str] = &["full", "high", "low"];

    for op in &["load", "load-offset"] {
        for pad in 1..5 {
            missing_token(&format!("label{e:w$}{}", op, e = "", w = pad),
                          |len| ParseInstructionError::MissingToken(len, TOKENS_FRAGMENT));
        }
    }
}

#[test]
fn label_name() {
    static TOKENS_LABEL_NAME: &[&str] = &["[label name]"];
    static TOKENS_FRAGMENT: &[&str] = &["full", "high", "low"];

    let test = |op| for pad in 1..5 {
        missing_token(&format!("label{e:w$}{}", op, e = "", w = pad),
                      |len| ParseInstructionError::MissingToken(len, TOKENS_LABEL_NAME));
    };

    for op in &["load", "load-offset"] {
        for frag in TOKENS_FRAGMENT {
            for pad in 1..5 {
                test(format!("{}{e:w$}{}", op, frag, e = "", w = pad));
            }
        }
    }
    test("save".to_string());
}

#[test]
fn label_offset() {
    static TOKENS_LABEL_OFFSET: &[&str] = &["[signed 16-bit label offset]"];
    static TOKENS_FRAGMENT: &[&str] = &["full", "high", "low"];

    for frag in TOKENS_FRAGMENT {
        for pad_left in 1..5 {
            for pad_center in 1..5 {
                for pad_right in 1..5 {
                    for token_len in 1..5 {
                        for _ in 0..5 {
                            let token: String = Alphanumeric.sample_iter(thread_rng()).take(token_len).collect();

                            missing_token(&format!("label{e:wl$}load-offset{e:wc$}{}{e:wr$}{}",
                                                   frag,
                                                   token,
                                                   e = "",
                                                   wl = pad_left,
                                                   wc = pad_center,
                                                   wr = pad_right),
                                          |len| ParseInstructionError::MissingToken(len, TOKENS_LABEL_OFFSET));
                        }
                    }
                }
            }
        }
    }
}
