use pir_8_emu::isa::instruction::{ParseInstructionError, Instruction};
use self::super::super::super::alt_gp_registers;
use pir_8_emu::isa::GeneralPurposeRegister;
use pir_8_emu::util::parse_with_prefix;
use self::super::unrecognised_token;


#[test]
fn toplevel_raw() {
    static TOKENS_TOP: &[&str] = &["MADR",
                                   "JMPZ",
                                   "JMPP",
                                   "JMPG",
                                   "JMPC",
                                   "JMZG",
                                   "JMZL",
                                   "JMPL",
                                   "JUMP",
                                   "LOAD",
                                   "SAVE",
                                   "ALU",
                                   "MOVE",
                                   "PORT",
                                   "COMP",
                                   "STCK",
                                   "CLRF",
                                   "HALT",
                                   "[raw instruction literal]"];

    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for i in 0b0001_0000_0000..0b0100_0000_0000 {
            assert_eq!(Instruction::from_str(&format!("{}", i), regs),
                       Err(ParseInstructionError::UnrecognisedToken(1, TOKENS_TOP)),
                       "{} {:#x}",
                       regs[0].letter(),
                       i);
            assert_eq!(Instruction::from_str(&format!("{:#x}", i), regs),
                       Err(ParseInstructionError::UnrecognisedToken(1, TOKENS_TOP)),
                       "{} {:#x}",
                       regs[0].letter(),
                       i);
            assert_eq!(Instruction::from_str(&format!("{:#X}", i), regs),
                       Err(ParseInstructionError::UnrecognisedToken(1, TOKENS_TOP)),
                       "{} {:#x}",
                       regs[0].letter(),
                       i);
            assert_eq!(Instruction::from_str(&format!("{:#o}", i), regs),
                       Err(ParseInstructionError::UnrecognisedToken(1, TOKENS_TOP)),
                       "{} {:#x}",
                       regs[0].letter(),
                       i);
            assert_eq!(Instruction::from_str(&format!("{:#b}", i), regs),
                       Err(ParseInstructionError::UnrecognisedToken(1, TOKENS_TOP)),
                       "{} {:#x}",
                       regs[0].letter(),
                       i);
        }
    }
}

#[test]
fn toplevel() {
    static TOKENS_TOP: &[&str] = &["MADR",
                                   "JMPZ",
                                   "JMPP",
                                   "JMPG",
                                   "JMPC",
                                   "JMZG",
                                   "JMZL",
                                   "JMPL",
                                   "JUMP",
                                   "LOAD",
                                   "SAVE",
                                   "ALU",
                                   "MOVE",
                                   "PORT",
                                   "COMP",
                                   "STCK",
                                   "CLRF",
                                   "HALT",
                                   "[raw instruction literal]"];

    unrecognised_token("",
                       TOKENS_TOP,
                       1..30,
                       |s, _| parse_with_prefix::<u8>(s).is_none(),
                       |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_TOP));
}

#[test]
fn alu_raw() {
    static TOKENS_ALU: &[&str] = &["ADD", "SUB", "ADDC", "SUBC", "OR", "XOR", "AND", "NOT", "SOR", "[raw operation literal]"];

    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for i in 0b0001_0000..=0b1111_1111 {
            for pad in 1..5 {
                assert_eq!(Instruction::from_str(&format!("ALU{e:w$}{}    ", i, e = "", w = pad), regs),
                           Err(ParseInstructionError::UnrecognisedToken(3 + pad + 1, TOKENS_ALU)),
                           "{} {:#x}",
                           regs[0].letter(),
                           i);
                assert_eq!(Instruction::from_str(&format!("ALU{e:w$}{:#0x}", i, e = "", w = pad), regs),
                           Err(ParseInstructionError::UnrecognisedToken(3 + pad + 1, TOKENS_ALU)),
                           "{} {:#x}",
                           regs[0].letter(),
                           i);
                assert_eq!(Instruction::from_str(&format!("ALU{e:w$}{:#0X}", i, e = "", w = pad), regs),
                           Err(ParseInstructionError::UnrecognisedToken(3 + pad + 1, TOKENS_ALU)),
                           "{} {:#x}",
                           regs[0].letter(),
                           i);
                assert_eq!(Instruction::from_str(&format!("ALU{e:w$}{:#0o}", i, e = "", w = pad), regs),
                           Err(ParseInstructionError::UnrecognisedToken(3 + pad + 1, TOKENS_ALU)),
                           "{} {:#x}",
                           regs[0].letter(),
                           i);
                assert_eq!(Instruction::from_str(&format!("ALU{e:w$}{:#0b}", i, e = "", w = pad), regs),
                           Err(ParseInstructionError::UnrecognisedToken(3 + pad + 1, TOKENS_ALU)),
                           "{} {:#x}",
                           regs[0].letter(),
                           i);
            }
        }
    }
}

#[test]
fn alu() {
    static TOKENS_ALU: &[&str] = &["ADD", "SUB", "ADDC", "SUBC", "OR", "XOR", "AND", "NOT", "SOR", "[raw operation literal]"];

    unrecognised_token("ALU",
                       TOKENS_ALU,
                       1..25,
                       |s, _| parse_with_prefix::<u8>(s).is_none(),
                       |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_ALU));
}

#[test]
fn alu_sor() {
    static TOKENS_SOR: &[&str] = &["LEFT", "RIGHT"];

    for pad in 1..5 {
        unrecognised_token(&format!("ALU{e:w$}SOR", e = "", w = pad),
                           TOKENS_SOR,
                           1..10,
                           |s, _| parse_with_prefix::<u8>(s).is_none(),
                           |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_SOR));
    }
}

#[test]
fn alu_sor_type() {
    static TOKENS_SOR_TYPE: &[&str] = &["LSF", "ASF", "RTC", "RTW"];

    for dir in &["LEFT", "RIGHT"] {
        for pad_left in 1..5 {
            for pad_right in 1..5 {
                unrecognised_token(&format!("ALU{e:wl$}SOR{e:wr$}{}", dir, e = "", wl = pad_left, wr = pad_right),
                                   TOKENS_SOR_TYPE,
                                   1..5,
                                   |s, _| parse_with_prefix::<u8>(s).is_none(),
                                   |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_SOR_TYPE));
            }
        }
    }
}

#[test]
fn madr() {
    static TOKENS_MADR: &[&str] = &["WRITE", "READ"];

    unrecognised_token("MADR",
                       TOKENS_MADR,
                       1..10,
                       |s, _| parse_with_prefix::<u8>(s).is_none(),
                       |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_MADR));
}

#[test]
fn load() {
    static TOKENS_LOAD: &[&str] = &["IMM", "IND"];

    unrecognised_token("LOAD",
                       TOKENS_LOAD,
                       1..5,
                       |s, _| parse_with_prefix::<u8>(s).is_none(),
                       |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_LOAD));
}

#[test]
fn port() {
    static TOKENS_PORT: &[&str] = &["IN", "OUT"];

    unrecognised_token("PORT",
                       TOKENS_PORT,
                       1..5,
                       |s, _| parse_with_prefix::<u8>(s).is_none(),
                       |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_PORT));
}

#[test]
fn stck() {
    static TOKENS_STCK: &[&str] = &["PUSH", "POP"];

    unrecognised_token("STCK",
                       TOKENS_STCK,
                       1..10,
                       |s, _| parse_with_prefix::<u8>(s).is_none(),
                       |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_STCK));
}

#[test]
fn stck_register_pair() {
    static TOKENS_STCK_REG_PAIR: &[&str] = &["A&B", "C&D"];

    for d in &["PUSH", "POP"] {
        for pad in 1..5 {
            unrecognised_token(&format!("STCK{e:w$}{}", d, e = "", w = pad),
                               TOKENS_STCK_REG_PAIR,
                               1..5,
                               |s, _| parse_with_prefix::<u8>(s).is_none(),
                               |len, _, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_STCK_REG_PAIR));
        }
    }
}
