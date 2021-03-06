use pir_8_emu::isa::instruction::{ParseInstructionError, Instruction};
use self::super::{too_many_tokens_rrr, unrecognised_token};
use rand::distributions::{Alphanumeric, Distribution};
use self::super::super::super::alt_gp_registers;
use pir_8_emu::isa::GeneralPurposeRegister;
use rand::thread_rng;


#[test]
fn raw() {
    for i in 0..=0b1111_1111 {
        unrecognised_token(&format!("{}", i), &[], 1..5, |_, _| true, |len, _, _| ParseInstructionError::TooManyTokens(len));
        unrecognised_token(&format!("{:#0x}", i),
                           &[],
                           1..3,
                           |_, _| true,
                           |len, _, _| ParseInstructionError::TooManyTokens(len));
        unrecognised_token(&format!("{:#0X}", i),
                           &[],
                           1..3,
                           |_, _| true,
                           |len, _, _| ParseInstructionError::TooManyTokens(len));
        unrecognised_token(&format!("{:#0o}", i),
                           &[],
                           1..3,
                           |_, _| true,
                           |len, _, _| ParseInstructionError::TooManyTokens(len));
        unrecognised_token(&format!("{:#0b}", i),
                           &[],
                           1..3,
                           |_, _| true,
                           |len, _, _| ParseInstructionError::TooManyTokens(len));
    }
}

#[test]
fn load_immediate_byte() {
    for pad_left in 1..3 {
        for pad_right in 1..3 {
            too_many_tokens_rrr(&format!("LOAD{e:wl$}IMM{e:wr$}BYTE", e = "", wl = pad_left, wr = pad_right));
        }
    }
}

#[test]
fn load_indirect() {
    for pad in 1..3 {
        too_many_tokens_rrr(&format!("LOAD{e:w$}IND", e = "", w = pad));
    }
}

#[test]
fn load_immediate_wide() {
    static TOKENS_LOAD_IMMEDIATE_WIDE_REGISTER_PAIR: &[&str] = &["A&B", "C&D", "X&Y", "ADR"];

    for tok in TOKENS_LOAD_IMMEDIATE_WIDE_REGISTER_PAIR {
        for pad_left in 1..3 {
            for pad_center in 1..3 {
                for pad_right in 1..3 {
                    unrecognised_token(&format!("LOAD{e:wl$}IMM{e:wc$}WIDE{e:wr$}{}",
                                                tok,
                                                e = "",
                                                wl = pad_left,
                                                wc = pad_center,
                                                wr = pad_right),
                                       &[],
                                       1..5,
                                       |_, _| true,
                                       |len, _, _| ParseInstructionError::TooManyTokens(len));
                }
            }
        }
    }
}

#[test]
fn jump_clrf_halt() {
    static TOKENS_TOP: &[&str] = &["JMPZ", "JMPP", "JMPG", "JMPC", "JMZG", "JMZL", "JMPL", "JUMP", "CLRF", "HALT"];

    for tok in TOKENS_TOP {
        unrecognised_token(tok, &[], 1..5, |_, _| true, |len, _, _| ParseInstructionError::TooManyTokens(len));
    }
}

#[test]
fn save() {
    too_many_tokens_rrr("SAVE");
}

#[test]
fn alu_raw() {
    for i in 0..=0b1111 {
        for pad in 1..5 {
            unrecognised_token(&format!("ALU{e:w$}{}", i, e = "", w = pad),
                               &[],
                               1..5,
                               |_, _| true,
                               |len, _, _| ParseInstructionError::TooManyTokens(len));
            unrecognised_token(&format!("ALU{e:w$}{:#0x}", i, e = "", w = pad),
                               &[],
                               1..5,
                               |_, _| true,
                               |len, _, _| ParseInstructionError::TooManyTokens(len));
            unrecognised_token(&format!("ALU{e:w$}{:#0X}", i, e = "", w = pad),
                               &[],
                               1..5,
                               |_, _| true,
                               |len, _, _| ParseInstructionError::TooManyTokens(len));
            unrecognised_token(&format!("ALU{e:w$}{:#0o}", i, e = "", w = pad),
                               &[],
                               1..5,
                               |_, _| true,
                               |len, _, _| ParseInstructionError::TooManyTokens(len));
            unrecognised_token(&format!("ALU{e:w$}{:#0b}", i, e = "", w = pad),
                               &[],
                               1..5,
                               |_, _| true,
                               |len, _, _| ParseInstructionError::TooManyTokens(len));
        }
    }
}

#[test]
fn alu() {
    static TOKENS_ALU: &[&str] = &["ADD", "SUB", "ADDC", "SUBC", "OR", "XOR", "AND", "NOT"];

    for tok in TOKENS_ALU {
        for pad in 1..5 {
            unrecognised_token(&format!("ALU{e:w$}{}", tok, e = "", w = pad),
                               &[],
                               1..5,
                               |_, _| true,
                               |len, _, _| ParseInstructionError::TooManyTokens(len));
        }
    }
}

#[test]
fn alu_sor() {
    static TOKENS_SOR_DIRECTION: &[&str] = &["LEFT", "RIGHT"];
    static TOKENS_SOR_TYPE: &[&str] = &["LSF", "ASF", "RTC", "RTW"];

    for d in TOKENS_SOR_DIRECTION {
        for tt in TOKENS_SOR_TYPE {
            for pad_left in 1..3 {
                for pad_center in 1..3 {
                    for pad_right in 1..3 {
                        unrecognised_token(&format!("ALU{e:wl$}SOR{e:wc$}{}{e:wr$}{}", d, tt, e = "", wl = pad_left, wc = pad_center, wr = pad_right),
                                           &[],
                                           1..5,
                                           |_, _| true,
                                           |len, _, _| ParseInstructionError::TooManyTokens(len));
                    }
                }
            }
        }
    }
}

#[test]
fn move_() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for rrr in regs {
            for bbb in regs {
                for pad_left in 1..3 {
                    for pad_center in 1..3 {
                        for pad_right in 1..3 {
                            for pad_rright in 1..3 {
                                for pad_rrright in 1..3 {
                                    for token_len in 1..3 {
                                        for _ in 0..10 {
                                            let rrr = rrr.letter();
                                            let bbb = bbb.letter();
                                            let token: String = Alphanumeric.sample_iter(thread_rng()).take(token_len).collect();

                                            let instr = format!("{e:wl$}MOVE{e:wc$}{}{e:wr$}{}{e:wrr$}{}{e:wrrr$}",
                                                                rrr,
                                                                bbb,
                                                                token,
                                                                e = "",
                                                                wl = pad_left,
                                                                wc = pad_center,
                                                                wr = pad_right,
                                                                wrr = pad_rright,
                                                                wrrr = pad_rrright);

                                            assert_eq!(Instruction::from_str(&instr, regs),
                                                       Err(ParseInstructionError::TooManyTokens(pad_left + 4 + pad_center + 1 + pad_right + 1 + pad_rright +
                                                                                                1)),
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
    }
}

#[test]
fn madr() {
    static TOKENS_MADR_DIRECTION: &[&str] = &["WRITE", "READ"];
    static TOKENS_MADR_REG_PAIR: &[&str] = &["A&B", "C&D"];

    for d in TOKENS_MADR_DIRECTION {
        for r in TOKENS_MADR_REG_PAIR {
            for pad_left in 1..3 {
                for pad_right in 1..3 {
                    unrecognised_token(&format!("MADR{e:wl$}{}{e:wr$}{}", d, r, e = "", wl = pad_left, wr = pad_right),
                                       &[],
                                       1..5,
                                       |_, _| true,
                                       |len, _, _| ParseInstructionError::TooManyTokens(len));
                }
            }
        }
    }
}

#[test]
fn port() {
    static TOKENS_PORT_DIRECTION: &[&str] = &["IN", "OUT"];

    for d in TOKENS_PORT_DIRECTION {
        for pad in 1..3 {
            too_many_tokens_rrr(&format!("PORT{e:wl$}{}", d, e = "", wl = pad));
        }
    }
}

#[test]
fn comp() {
    too_many_tokens_rrr("COMP");
}

#[test]
fn stck() {
    static TOKENS_STCK_DIRECTION: &[&str] = &["PUSH", "POP"];
    static TOKENS_STCK_REG_PAIR: &[&str] = &["A&B", "C&D"];

    for d in TOKENS_STCK_DIRECTION {
        for r in TOKENS_STCK_REG_PAIR {
            for pad_left in 1..3 {
                for pad_right in 1..3 {
                    unrecognised_token(&format!("STCK{e:wl$}{}{e:wr$}{}", d, r, e = "", wl = pad_left, wr = pad_right),
                                       &[],
                                       1..5,
                                       |_, _| true,
                                       |len, _, _| ParseInstructionError::TooManyTokens(len));
                }
            }
        }
    }
}
