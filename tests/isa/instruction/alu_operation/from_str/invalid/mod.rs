use pir_8_emu::isa::instruction::{ParseInstructionError, AluOperation};
use rand::distributions::{Alphanumeric, Distribution};
use std::str::FromStr;
use rand::thread_rng;
use std::ops::Range;

mod missing_token_operation;
mod unrecognised_token;
mod too_many_tokens;


#[test]
fn invalid_character() {
    assert_eq!(AluOperation::from_str("  \n"), Err(ParseInstructionError::InvalidCharacter(2)));
    assert_eq!(AluOperation::from_str("  \x0B"), Err(ParseInstructionError::InvalidCharacter(2)));

    assert_eq!(AluOperation::from_str("  Ą"), Err(ParseInstructionError::InvalidCharacter(2)));
    assert_eq!(AluOperation::from_str("  Ж"), Err(ParseInstructionError::InvalidCharacter(2)));
}

#[test]
fn empty_string() {
    let mut empty = String::new();
    for _ in 0..100 {
        assert_eq!(AluOperation::from_str(&empty), Err(ParseInstructionError::EmptyString));
        empty.push_str(" \t");
    }
}


fn unrecognised_token(base: &str, valid: &[&str], lens: Range<usize>, discriminator: fn(&str) -> bool,
                      err: fn(usize, &str) -> ParseInstructionError) {
        for pad_left in 1..5 {
            for pad_center in 1..5 {
                for pad_right in 1..5 {
                    for token_len in lens.clone() {
                        for _ in 0..10 {
                            let mut token = String::new();
                            while token.is_empty() || valid.iter().find(|v| v.eq_ignore_ascii_case(&token)).is_some() || !discriminator(&token) {
                                token = Alphanumeric.sample_iter(thread_rng()).take(token_len).collect();
                            }

                            let instr = format!("{e:wl$}{}{e:wc$}{}{e:wr$}", base, token, e = "", wl = pad_left, wc = pad_center, wr = pad_right);

                            assert_eq!(AluOperation::from_str(&instr),
                                       Err(err(pad_left + base.len() + pad_center + 1, &instr)),
                                       "{:?}",
                                       instr);
                        }
                    }
                }
            }
    }
}

fn missing_token(base: &str, err: fn(usize) -> ParseInstructionError) {
    for pad_left in 1..5 {
        for pad_right in 1..5 {
            let instr = format!("{e:wl$}{}{e:wr$}", base, e = "", wl = pad_left, wr = pad_right);

            assert_eq!(AluOperation::from_str(&instr), Err(err(pad_left + base.len() + 1)), "{:?}", instr);
        }
    }
}
