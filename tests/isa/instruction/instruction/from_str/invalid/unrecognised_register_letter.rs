use pir_8_emu::isa::instruction::{ParseInstructionError, Instruction};
use rand::distributions::{Alphanumeric, Distribution};
use self::super::super::super::alt_gp_registers;
use self::super::unrecognised_register_letter;
use pir_8_emu::isa::GeneralPurposeRegister;
use rand::thread_rng;


#[test]
fn load_immedate_byte() {
    for pad_left in 1..5 {
        for pad_right in 1..5 {
            unrecognised_register_letter(&format!("LOAD{e:wl$}IMM{e:wr$}BYTE", e = "", wl = pad_left, wr = pad_right));
        }
    }
}

#[test]
fn load_indirect() {
    for pad in 1..5 {
        unrecognised_register_letter(&format!("LOAD{e:w$}IND", e = "", w = pad));
    }
}

#[test]
fn save() {
    unrecognised_register_letter("SAVE");
}

#[test]
fn move_qqq() {
    unrecognised_register_letter("MOVE");
}

#[test]
fn move_rrr() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for pad_left in 1..5 {
            for pad_center in 1..5 {
                for pad_right in 1..5 {
                    for pad_rright in 1..5 {
                        for qqq in regs {
                            for _ in 0..10 {
                                let qqq = qqq.letter();
                                let rrr = Alphanumeric.sample_iter(thread_rng())
                                    .find(|rrr| regs.iter().find(|v| v.letter().eq_ignore_ascii_case(rrr)).is_none())
                                    .unwrap();

                                let instr = format!("{e:wl$}MOVE{e:wc$}{}{e:wr$}{}{e:wrr$}",
                                                    qqq,
                                                    rrr,
                                                    e = "",
                                                    wl = pad_left,
                                                    wc = pad_center,
                                                    wr = pad_right,
                                                    wrr = pad_rright);

                                assert_eq!(Instruction::from_str(&instr, regs),
                                           Err(ParseInstructionError::UnrecognisedRegisterLetter(pad_left + 4 + pad_center + 1 + pad_right + 1,
                                                                                                 rrr,
                                                                                                 [regs[0].letter(),
                                                                                                  regs[1].letter(),
                                                                                                  regs[2].letter(),
                                                                                                  regs[3].letter(),
                                                                                                  regs[4].letter(),
                                                                                                  regs[5].letter(),
                                                                                                  regs[6].letter(),
                                                                                                  regs[7].letter()])),
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

#[test]
fn comp() {
    unrecognised_register_letter("COMP");
}

#[test]
fn port() {
    for dir in &["IN", "OUT"] {
        for pad in 1..5 {
            unrecognised_register_letter(&format!("PORT{e:w$}{}", dir, e = "", w = pad));
        }
    }
}
