use pir_8_emu::isa::GeneralPurposeRegister;


#[test]
fn new_data_zero() {
    assert_eq!(*GeneralPurposeRegister::new(0b010, 'X').unwrap(), 0);
}

#[test]
fn new_address_preserved() {
    for i in 0..=0b111u8 {
        assert_eq!(GeneralPurposeRegister::new(i, 'Q').unwrap().address(), i);
    }
}

#[test]
fn new_invalid_address_rejected() {
    for i in (0b111 + 1)..2u32.pow(8) {
        assert_eq!(GeneralPurposeRegister::new(i as u8, 'Q'), None);
    }
}

#[test]
fn new_letter_preserved() {
    assert_eq!(GeneralPurposeRegister::new(0b010, 'X').unwrap().letter(), 'X');
}

#[test]
fn new_invalid_letter_rejected() {
    assert_eq!(GeneralPurposeRegister::new(0b010, 'Ą'), None);
    assert_eq!(GeneralPurposeRegister::new(0b010, 'Ж'), None);
}

#[test]
fn display() {
    let mut reg = GeneralPurposeRegister::new(0b010, 'X').unwrap();
    for i in 0..2u32.pow(8) {
        *reg = i as u8;
        assert_eq!(reg.to_string(), format!("X({:02X})", i));
    }
}
