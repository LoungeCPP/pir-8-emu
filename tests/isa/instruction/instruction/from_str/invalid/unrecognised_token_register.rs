use pir_8_emu::isa::instruction::{ParseInstructionError, Instruction};
use rand::distributions::{Alphanumeric, Distribution};
use self::super::super::super::alt_gp_registers;
use pir_8_emu::isa::GeneralPurposeRegister;
use self::super::unrecognised_token;
use rand::thread_rng;


static TOKENS_REGISTER: &[&str] = &["[register letter]"];


#[test]
fn save() {
    unrecognised_token("SAVE",
                       &[],
                       2..5,
                       |_, _| true,
                       |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_REGISTER));
}

#[test]
fn move_aaa() {
    unrecognised_token("MOVE",
                       &[],
                       2..5,
                       |_, _| true,
                       |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_REGISTER));
}

#[test]
fn move_bbb() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for pad_left in 1..5 {
            for pad_center in 1..5 {
                for pad_right in 1..5 {
                    for pad_rright in 1..5 {
                        for aaa in regs {
                            for token_len in 2..5 {
                                for _ in 0..10 {
                                    let aaa = aaa.letter();
                                    let bbb: String = Alphanumeric.sample_iter(thread_rng()).take(token_len).collect();

                                    let instr = format!("{e:wl$}MOVE{e:wc$}{}{e:wr$}{}{e:wrr$}",
                                                        aaa,
                                                        bbb,
                                                        e = "",
                                                        wl = pad_left,
                                                        wc = pad_center,
                                                        wr = pad_right,
                                                        wrr = pad_rright);

                                    assert_eq!(Instruction::from_str(&instr, regs),
                                               Err(ParseInstructionError::UnrecognisedToken(pad_left + 4 + pad_center + 1 + pad_right + 1, TOKENS_REGISTER)),
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

#[test]
fn comp() {
    unrecognised_token("MOVE",
                       &[],
                       2..5,
                       |_, _| true,
                       |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_REGISTER));
}

#[test]
fn load() {
    for var in &["IMM", "IND"] {
        for pad in 1..5 {
            unrecognised_token(&format!("LOAD{e:w$}{}", var, e = "", w = pad),
                               &[],
                               2..5,
                               |_, _| true,
                               |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_REGISTER));
        }
    }
}

#[test]
fn port() {
    for dir in &["IN", "OUT"] {
        for pad in 1..5 {
            unrecognised_token(&format!("PORT{e:w$}{}", dir, e = "", w = pad),
                               &[],
                               2..5,
                               |_, _| true,
                               |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_REGISTER));
        }
    }
}
