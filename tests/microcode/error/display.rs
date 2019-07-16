use pir_8_emu::microcode::MicrocodeExecutionError;


static EXPECTED: &[u8] = &[0, 1, 2, 3, 4];


#[test]
fn microstack_underflow() {
    assert_eq!(MicrocodeExecutionError::MicrostackUnderflow.to_string(), "μStack underflow");
}

#[test]
fn invalid_microstack_top() {
    for top in 0..=0xFF {
        assert_eq!(MicrocodeExecutionError::InvalidMicrostackTop(top, EXPECTED).to_string(),
                   format!("Invalid top of the μstack: {:#04x}, expected any of: 0x00, 0x01, 0x02, 0x03, 0x04", top));
    }
}

#[test]
fn stack_overflow() {
    assert_eq!(MicrocodeExecutionError::StackOverflow.to_string(), "Stack overflow");
}

#[test]
fn stack_underflow() {
    assert_eq!(MicrocodeExecutionError::StackUnderflow.to_string(), "Stack underflow");
}

#[test]
fn program_overflow() {
    assert_eq!(MicrocodeExecutionError::ProgramOverflow.to_string(), "Program overflow");
}
