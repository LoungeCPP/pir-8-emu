use pir_8_emu::isa::instruction::{ParseInstructionError, Instruction};
use rand::distributions::{Alphanumeric, Distribution};
use pir_8_emu::isa::GeneralPurposeRegister;
use self::super::super::alt_gp_registers;
use rand::thread_rng;
use std::ops::Range;

mod unrecognised_token_instruction;
mod unrecognised_register_letter;
mod unrecognised_token_register;
mod missing_token_instruction;
mod missing_register_letter;
mod too_many_tokens;


#[test]
fn invalid_character() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::from_str("  \n", regs), Err(ParseInstructionError::InvalidCharacter(2)));
        assert_eq!(Instruction::from_str("  \x0B", regs), Err(ParseInstructionError::InvalidCharacter(2)));

        assert_eq!(Instruction::from_str("  Ą", regs), Err(ParseInstructionError::InvalidCharacter(2)));
        assert_eq!(Instruction::from_str("  Ж", regs), Err(ParseInstructionError::InvalidCharacter(2)));
    }
}

#[test]
fn empty_string() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        let mut empty = String::new();
        for _ in 0..100 {
            assert_eq!(Instruction::from_str(&empty, regs), Err(ParseInstructionError::EmptyString));
            empty.push_str(" \t");
        }
    }
}


fn unrecognised_token(base: &str, valid: &[&str], lens: Range<usize>, discriminator: fn(&str, &[GeneralPurposeRegister; 8]) -> bool,
                      err: fn(usize, &str, &[GeneralPurposeRegister; 8]) -> ParseInstructionError) {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for pad_left in 1..5 {
            for pad_center in 1..5 {
                for pad_right in 1..5 {
                    for token_len in lens.clone() {
                        for _ in 0..10 {
                            let mut token = String::new();
                            while token.is_empty() || valid.iter().find(|v| v.eq_ignore_ascii_case(&token)).is_some() || !discriminator(&token, &regs) {
                                token = Alphanumeric.sample_iter(thread_rng()).take(token_len).collect();
                            }

                            let instr = format!("{e:wl$}{}{e:wc$}{}{e:wr$}", base, token, e = "", wl = pad_left, wc = pad_center, wr = pad_right);

                            assert_eq!(Instruction::from_str(&instr, regs),
                                       Err(err(pad_left + base.len() + pad_center + 1, &instr, &regs)),
                                       "{:?}",
                                       instr);
                        }
                    }
                }
            }
        }
    }
}

fn unrecognised_register_letter(base: &str) {
    unrecognised_token(base,
                       &[],
                       1..1,
                       |r, regs| regs.iter().find(|v| v.letter().eq_ignore_ascii_case(&r.chars().next().unwrap())).is_some(),
                       |len, r, regs| {
        ParseInstructionError::UnrecognisedRegisterLetter(len,
                                                          r.chars().next().unwrap(),
                                                          [regs[0].letter(),
                                                           regs[1].letter(),
                                                           regs[2].letter(),
                                                           regs[3].letter(),
                                                           regs[4].letter(),
                                                           regs[5].letter(),
                                                           regs[6].letter(),
                                                           regs[7].letter()])
    })
}

fn missing_token(base: &str, err: fn(usize, &[GeneralPurposeRegister; 8]) -> ParseInstructionError) {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for pad_left in 1..5 {
            for pad_right in 1..5 {
                let instr = format!("{e:wl$}{}{e:wr$}", base, e = "", wl = pad_left, wr = pad_right);

                assert_eq!(Instruction::from_str(&instr, regs),
                           Err(err(pad_left + base.len() + 1, &regs)),
                           "{:?}",
                           instr);
            }
        }
    }
}

fn too_many_tokens_aaa(base: &str) {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for aaa in regs {
            for pad_left in 1..5 {
                for pad_center in 1..5 {
                    for pad_right in 1..5 {
                        for pad_rright in 1..5 {
                            for token_len in 1..5 {
                                for _ in 0..10 {
                                    let aaa = aaa.letter();
                                    let token: String = Alphanumeric.sample_iter(thread_rng()).take(token_len).collect();

                                    let instr = format!("{e:wl$}{}{e:wc$}{}{e:wr$}{}{e:wrr$}",
                                                        base,
                                                        aaa,
                                                        token,
                                                        e = "",
                                                        wl = pad_left,
                                                        wc = pad_center,
                                                        wr = pad_right,
                                                        wrr = pad_rright);

                                    assert_eq!(Instruction::from_str(&instr, regs),
                                               Err(ParseInstructionError::TooManyTokens(pad_left + base.len() + pad_center + 1 + pad_right + 1)),
                                               "{:?}",
                                               instr);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
