use pir_8_emu::isa::instruction::{ParseInstructionError, AluOperation};
use pir_8_emu::util::parse_with_prefix;
use self::super::unrecognised_token;
use std::str::FromStr;


#[test]
fn toplevel_raw() {
    static TOKENS_ALU: &[&str] = &["ADD", "SUB", "NOT", "OR", "XOR", "AND", "SOR", "[raw operation literal]"];

    for i in 0b0001_0000..=0b1111_1111 {
        assert_eq!(AluOperation::from_str(&format!("{}", i)),
                   Err(ParseInstructionError::UnrecognisedToken(1, TOKENS_ALU)),
                   "{:#x}",
                   i);
        assert_eq!(AluOperation::from_str(&format!("{:#0x}", i)),
                   Err(ParseInstructionError::UnrecognisedToken(1, TOKENS_ALU)),
                   "{:#x}",
                   i);
        assert_eq!(AluOperation::from_str(&format!("{:#0X}", i)),
                   Err(ParseInstructionError::UnrecognisedToken(1, TOKENS_ALU)),
                   "{:#x}",
                   i);
        assert_eq!(AluOperation::from_str(&format!("{:#0o}", i)),
                   Err(ParseInstructionError::UnrecognisedToken(1, TOKENS_ALU)),
                   "{:#x}",
                   i);
        assert_eq!(AluOperation::from_str(&format!("{:#0b}", i)),
                   Err(ParseInstructionError::UnrecognisedToken(1, TOKENS_ALU)),
                   "{:#x}",
                   i);
    }
}

#[test]
fn toplevel() {
    static TOKENS_ALU: &[&str] = &["ADD", "SUB", "NOT", "OR", "XOR", "AND", "SOR", "[raw operation literal]"];

    unrecognised_token("",
                       TOKENS_ALU,
                       1..25,
                       |s| parse_with_prefix::<u8>(s).is_none(),
                       |len, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_ALU));
}

#[test]
fn sor() {
    static TOKENS_SOR: &[&str] = &["LEFT", "RIGHT"];

    unrecognised_token("SOR",
                       TOKENS_SOR,
                       1..10,
                       |s| parse_with_prefix::<u8>(s).is_none(),
                       |len, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_SOR));
}

#[test]
fn sor_type() {
    static TOKENS_SOR_TYPE: &[&str] = &["LSF", "ASF", "RTC", "RTW"];

    for dir in &["LEFT", "RIGHT"] {
        for pad in 1..5 {
            unrecognised_token(&format!("SOR{e:w$}{}", dir, e = "", w = pad),
                               TOKENS_SOR_TYPE,
                               1..5,
                               |s| parse_with_prefix::<u8>(s).is_none(),
                               |len, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_SOR_TYPE));
        }
    }
}
