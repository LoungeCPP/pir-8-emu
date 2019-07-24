use pir_8_emu::isa::instruction::ParseInstructionError;
use pir_8_emu::binutils::pir_8_as::AssemblerDirective;
use self::super::unrecognised_token;


#[test]
fn toplevel() {
    static TOKENS_TOPLEVEL: &[&str] = &["origin", "label"];

    unrecognised_token("", TOKENS_TOPLEVEL, 1..25, |len, _| ParseInstructionError::UnrecognisedToken(len, TOKENS_TOPLEVEL));
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
