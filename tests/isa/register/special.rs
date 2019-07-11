use pir_8_emu::isa::SpecialPurposeRegister;


#[test]
fn new_data_zero() {
    assert_eq!(*SpecialPurposeRegister::<u8>::new("SpecialPurposeRegister::<u8>", "SU8"), 0);
    assert_eq!(*SpecialPurposeRegister::<u16>::new("SpecialPurposeRegister::<u16>", "SU16"), 0);
}

#[test]
fn new_name_preserved() {
    assert_eq!(SpecialPurposeRegister::<u8>::new("SpecialPurposeRegister::<u8>", "SU8").name(),
               "SpecialPurposeRegister::<u8>");
    assert_eq!(SpecialPurposeRegister::<u16>::new("SpecialPurposeRegister::<u16>", "SU16").name(),
               "SpecialPurposeRegister::<u16>");
}

#[test]
fn new_shortname_preserved() {
    assert_eq!(SpecialPurposeRegister::<u8>::new("SpecialPurposeRegister::<u8>", "SU8").short_name(), "SU8");
    assert_eq!(SpecialPurposeRegister::<u16>::new("SpecialPurposeRegister::<u16>", "SU16").short_name(), "SU16");
}

#[test]
fn debug() {
    let mut reg = SpecialPurposeRegister::<u8>::new("SpecialPurposeRegister::<u8>", "SU8");
    for i in 0..2u32.pow(8) {
        *reg = i as u8;
        assert_eq!(reg.to_string(), format!("SU8({:02X})", i));
    }

    let mut reg = SpecialPurposeRegister::<u16>::new("SpecialPurposeRegister::<u16>", "SU16");
    for i in 0..2u32.pow(16) {
        *reg = i as u16;
        assert_eq!(reg.to_string(), format!("SU16({:04X})", i));
    }
}
