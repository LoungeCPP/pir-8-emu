use pir_8_emu::micro::MicroOpPerformError;


static EXPECTED: &[u8] = &[0, 1, 2, 3, 4];


#[test]
fn microstack_underflow() {
    assert_eq!(MicroOpPerformError::MicrostackUnderflow.to_string(), "μstack underflow");
}

#[test]
fn invalid_microstack_top() {
    for top in 0..=0xFF {
        assert_eq!(MicroOpPerformError::InvalidMicrostackTop(top, EXPECTED).to_string(),
                   format!("Invalid top of the μstack: {:#04x}, expected any of: 0x00, 0x01, 0x02, 0x03, 0x04", top));
    }
}
