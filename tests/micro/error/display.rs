use pir_8_emu::micro::MicroOpPerformError;


static EXPECTED: &[u8] = &[0, 1, 2, 3, 4];


#[test]
fn microstack_underflow() {
    assert_eq!(MicroOpPerformError::MicrostackUnderflow.to_string(), "μStack underflow");
}

#[test]
fn invalid_microstack_top() {
    for top in 0..=0xFF {
        assert_eq!(MicroOpPerformError::InvalidMicrostackTop(top, EXPECTED).to_string(),
                   format!("Invalid top of the μstack: {:#04x}, expected any of: 0x00, 0x01, 0x02, 0x03, 0x04", top));
    }
}

#[test]
fn stack_overflow() {
    assert_eq!(MicroOpPerformError::StackOverflow.to_string(), "Stack overflow");
}

#[test]
fn stack_underflow() {
    assert_eq!(MicroOpPerformError::StackUnderflow.to_string(), "Stack underflow");
}

#[test]
fn program_overflow() {
    assert_eq!(MicroOpPerformError::ProgramOverflow.to_string(), "Program overflow");
}
