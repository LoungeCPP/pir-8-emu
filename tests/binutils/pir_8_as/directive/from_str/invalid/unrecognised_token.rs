use pir_8_emu::isa::instruction::ParseInstructionError;
use pir_8_emu::binutils::pir_8_as::AssemblerDirective;
use rand::distributions::{Alphanumeric, Distribution};
use self::super::unrecognised_token;
use rand::thread_rng;


#[test]
fn toplevel() {
    static TOKENS_TOPLEVEL: &[&str] = &["origin", "label", "literal"];

    unrecognised_token("",
                       TOKENS_TOPLEVEL,
                       1..25,
                       |len, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_TOPLEVEL));
}

#[test]
fn origin() {
    static TOKENS_ORIGIN: &[&str] = &["[16-bit origin address]"];

    for addr in (0x1FFFF..=0x4FFFF).step_by(0xFF) {
        assert_eq!(AssemblerDirective::from_str(&format!(":origin {}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(9, TOKENS_ORIGIN)),
                   "{:#x}",
                   addr);
        assert_eq!(AssemblerDirective::from_str(&format!(":origin {:#0x}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(9, TOKENS_ORIGIN)),
                   "{:#x}",
                   addr);
        assert_eq!(AssemblerDirective::from_str(&format!(":origin {:#0X}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(9, TOKENS_ORIGIN)),
                   "{:#x}",
                   addr);
        assert_eq!(AssemblerDirective::from_str(&format!(":origin {:#0o}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(9, TOKENS_ORIGIN)),
                   "{:#x}",
                   addr);
        assert_eq!(AssemblerDirective::from_str(&format!(":origin {:#0b}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(9, TOKENS_ORIGIN)),
                   "{:#x}",
                   addr);
    }
}

#[test]
fn label() {
    static TOKENS_LABEL: &[&str] = &["save", "load", "load-offset"];

    unrecognised_token("label",
                       TOKENS_LABEL,
                       1..10,
                       |len, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_LABEL));
}

#[test]
fn label_offset() {
    static TOKENS_OFFSET: &[&str] = &["[signed 16-bit label offset]"];

    for addr in (0x1FFFF..=0x4FFFF).step_by(0xFF) {
        assert_eq!(AssemblerDirective::from_str(&format!(":label load-offset UwU -{}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(24, TOKENS_OFFSET)),
                   "{:#x}",
                   addr);
        assert_eq!(AssemblerDirective::from_str(&format!(":label load-offset UwU -{:#0x}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(24, TOKENS_OFFSET)),
                   "{:#x}",
                   addr);
        assert_eq!(AssemblerDirective::from_str(&format!(":label load-offset UwU -{:#0X}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(24, TOKENS_OFFSET)),
                   "{:#x}",
                   addr);
        assert_eq!(AssemblerDirective::from_str(&format!(":label load-offset UwU -{:#0o}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(24, TOKENS_OFFSET)),
                   "{:#x}",
                   addr);
        assert_eq!(AssemblerDirective::from_str(&format!(":label load-offset UwU -{:#0b}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(24, TOKENS_OFFSET)),
                   "{:#x}",
                   addr);

        assert_eq!(AssemblerDirective::from_str(&format!(":label load-offset UwU {}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(24, TOKENS_OFFSET)),
                   "{:#x}",
                   addr);
        assert_eq!(AssemblerDirective::from_str(&format!(":label load-offset UwU {:#0x}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(24, TOKENS_OFFSET)),
                   "{:#x}",
                   addr);
        assert_eq!(AssemblerDirective::from_str(&format!(":label load-offset UwU {:#0X}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(24, TOKENS_OFFSET)),
                   "{:#x}",
                   addr);
        assert_eq!(AssemblerDirective::from_str(&format!(":label load-offset UwU {:#0o}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(24, TOKENS_OFFSET)),
                   "{:#x}",
                   addr);
        assert_eq!(AssemblerDirective::from_str(&format!(":label load-offset UwU {:#0b}", addr)),
                   Err(ParseInstructionError::UnrecognisedToken(24, TOKENS_OFFSET)),
                   "{:#x}",
                   addr);
    }
}

#[test]
fn literal() {
    static TOKENS_LITERAL: &[&str] = &["\"[string]\""];

    for pad_lleft in 0..5 {
        for pad_left in 0..5 {
            for pad_center in 1..5 {
                for pad_right in 1..5 {
                    for token_len in 1..10 {
                        for (quote_pre, quote_post) in &[(false, false), (false, true), (true, false)] {
                            for _ in 0..5 {
                                let token = Alphanumeric.sample_iter(thread_rng()).take(token_len).collect::<String>();

                                let instr = format!("{e:wll$}:{e:wl$}literal{e:wc$}{}{}{}{e:wr$}",
                                                    token,
                                                    if *quote_pre { "\"" } else { "" },
                                                    if *quote_post { "\"" } else { "" },
                                                    e = "",
                                                    wll = pad_lleft,
                                                    wl = pad_left,
                                                    wc = pad_center,
                                                    wr = pad_right);

                                assert_eq!(AssemblerDirective::from_str(&instr),
                                           Err(ParseInstructionError::UnrecognisedToken(pad_lleft + 1 + pad_left + "literal".len() + pad_center + 1,
                                                                                        TOKENS_LITERAL)),
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
