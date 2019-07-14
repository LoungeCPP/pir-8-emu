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
fn defaults() {
    assert_eq!(GeneralPurposeRegister::defaults(),
               [GeneralPurposeRegister::new(0b000, 'F').unwrap(),
                GeneralPurposeRegister::new(0b001, 'S').unwrap(),
                GeneralPurposeRegister::new(0b010, 'X').unwrap(),
                GeneralPurposeRegister::new(0b011, 'Y').unwrap(),
                GeneralPurposeRegister::new(0b100, 'A').unwrap(),
                GeneralPurposeRegister::new(0b101, 'B').unwrap(),
                GeneralPurposeRegister::new(0b110, 'C').unwrap(),
                GeneralPurposeRegister::new(0b111, 'D').unwrap()]);
}

#[test]
fn display() {
    let mut reg = GeneralPurposeRegister::new(0b010, 'X').unwrap();
    for i in 0..2u32.pow(8) {
        *reg = i as u8;
        assert_eq!(reg.to_string(), format!("X({:02X})", i));
    }
}
