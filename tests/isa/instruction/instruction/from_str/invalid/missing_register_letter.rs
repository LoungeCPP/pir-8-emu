use pir_8_emu::isa::instruction::{ParseInstructionError, Instruction};
use self::super::super::super::alt_gp_registers;
use pir_8_emu::isa::GeneralPurposeRegister;
use self::super::missing_token;


#[test]
fn load_immedate_byte() {
    for pad_left in 1..5 {
        for pad_right in 1..5 {
            missing_token(&format!("LOAD{e:wl$}IMM{e:wr$}BYTE", e = "", wl = pad_left, wr = pad_right), |len, regs| {
                ParseInstructionError::MissingRegisterLetter(len,
                                                             [regs[0].letter(),
                                                              regs[1].letter(),
                                                              regs[2].letter(),
                                                              regs[3].letter(),
                                                              regs[4].letter(),
                                                              regs[5].letter(),
                                                              regs[6].letter(),
                                                              regs[7].letter()])
            });
        }
    }
}

#[test]
fn load_indirect() {
    for pad in 1..5 {
        missing_token(&format!("LOAD{e:w$}IND", e = "", w = pad), |len, regs| {
            ParseInstructionError::MissingRegisterLetter(len,
                                                         [regs[0].letter(),
                                                          regs[1].letter(),
                                                          regs[2].letter(),
                                                          regs[3].letter(),
                                                          regs[4].letter(),
                                                          regs[5].letter(),
                                                          regs[6].letter(),
                                                          regs[7].letter()])
        });
    }
}

#[test]
fn save() {
    missing_token("SAVE", |len, regs| {
        ParseInstructionError::MissingRegisterLetter(len,
                                                     [regs[0].letter(),
                                                      regs[1].letter(),
                                                      regs[2].letter(),
                                                      regs[3].letter(),
                                                      regs[4].letter(),
                                                      regs[5].letter(),
                                                      regs[6].letter(),
                                                      regs[7].letter()])
    });
}

#[test]
fn move_aaa() {
    missing_token("MOVE", |len, regs| {
        ParseInstructionError::MissingRegisterLetter(len,
                                                     [regs[0].letter(),
                                                      regs[1].letter(),
                                                      regs[2].letter(),
                                                      regs[3].letter(),
                                                      regs[4].letter(),
                                                      regs[5].letter(),
                                                      regs[6].letter(),
                                                      regs[7].letter()])
    });
}

#[test]
fn move_bbb() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for pad_left in 1..5 {
            for pad_center in 1..5 {
                for pad_right in 1..5 {
                    for aaa in regs {
                        let aaa = aaa.letter();

                        let instr = format!("{e:wl$}MOVE{e:wc$}{}{e:wr$}", aaa, e = "", wl = pad_left, wc = pad_center, wr = pad_right);

                        assert_eq!(Instruction::from_str(&instr, regs),
                                   Err(ParseInstructionError::MissingRegisterLetter(pad_left + 4 + pad_center + 1 + 1,
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

#[test]
fn comp() {
    missing_token("MOVE", |len, regs| {
        ParseInstructionError::MissingRegisterLetter(len,
                                                     [regs[0].letter(),
                                                      regs[1].letter(),
                                                      regs[2].letter(),
                                                      regs[3].letter(),
                                                      regs[4].letter(),
                                                      regs[5].letter(),
                                                      regs[6].letter(),
                                                      regs[7].letter()])
    });
}

#[test]
fn port() {
    for dir in &["IN", "OUT"] {
        for pad in 1..5 {
            missing_token(&format!("PORT{e:w$}{}", dir, e = "", w = pad), |len, regs| {
                ParseInstructionError::MissingRegisterLetter(len,
                                                             [regs[0].letter(),
                                                              regs[1].letter(),
                                                              regs[2].letter(),
                                                              regs[3].letter(),
                                                              regs[4].letter(),
                                                              regs[5].letter(),
                                                              regs[6].letter(),
                                                              regs[7].letter()])
            });
        }
    }
}
