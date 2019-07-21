use pir_8_emu::isa::instruction::ParseInstructionError;
use pir_8_emu::binutils::pir_8_as::AssemblerDirective;
use rand::distributions::{Alphanumeric, Distribution};
use rand::thread_rng;
use std::ops::Range;

mod unrecognised_token;
mod too_many_tokens;
mod missing_token;


#[test]
fn empty_string_no_colon() {
    let mut empty = String::new();
    for _ in 0..100 {
        assert_eq!(AssemblerDirective::from_str(&empty), Err(ParseInstructionError::EmptyString));
        empty.push_str(" \t");
    }
}

#[test]
fn empty_string_colon() {
    let mut empty = ":".to_string();
    for _ in 0..100 {
        empty.insert(0, ' ');
        empty.insert(0, '\t');
        assert_eq!(AssemblerDirective::from_str(&empty), Err(ParseInstructionError::EmptyString));
        empty.push_str(" \t");
    }
}


fn unrecognised_token(base: &str, valid: &[&str], lens: Range<usize>, err: fn(usize, &str) -> ParseInstructionError) {
    for pad_lleft in 0..5 {
        for pad_left in 0..5 {
            for pad_center in 1..5 {
                for pad_right in 1..5 {
                    for token_len in lens.clone() {
                        for _ in 0..5 {
                            let mut token = String::new();
                            while token.is_empty() || valid.iter().find(|v| v.eq_ignore_ascii_case(&token)).is_some() {
                                token = Alphanumeric.sample_iter(thread_rng()).take(token_len).collect();
                            }

                            let instr = format!("{e:wll$}:{e:wl$}{}{e:wc$}{}{e:wr$}",
                                                base,
                                                token,
                                                e = "",
                                                wll = pad_lleft,
                                                wl = pad_left,
                                                wc = pad_center,
                                                wr = pad_right);

                            assert_eq!(AssemblerDirective::from_str(&instr),
                                       Err(err(pad_lleft + 1 + pad_left + base.len() + pad_center + 1, &instr)),
                                       "{:?}",
                                       instr);
                        }
                    }
                }
            }
        }
    }
}

fn missing_token(base: &str, err: fn(usize) -> ParseInstructionError) {
    for pad_left in 0..5 {
        for pad_center in 0..5 {
            for pad_right in 1..5 {
                let instr = format!("{e:wl$}:{e:wc$}{}{e:wr$}", base, e = "", wl = pad_left, wc = pad_center, wr = pad_right);

                assert_eq!(AssemblerDirective::from_str(&instr), Err(err(pad_left + 1 + pad_center + base.len() + 1)), "{:?}", instr);
            }
        }
    }
}
