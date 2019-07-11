use pir_8_emu::isa::SpecialPurposeRegister;


#[test]
fn new_data_zero() {
    assert_eq!(SpecialPurposeRegister::<u8>::new("SpecialPurposeRegister::<u8>", "SU8").data, 0);
    assert_eq!(SpecialPurposeRegister::<u16>::new("SpecialPurposeRegister::<u16>", "SU16").data, 0);
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
        reg.data = i as u8;
        assert_eq!(format!("{:?}", reg), format!("SU8({:02X})", i));
    }

    let mut reg = SpecialPurposeRegister::<u16>::new("SpecialPurposeRegister::<u16>", "SU16");
    for i in 0..2u32.pow(16) {
        reg.data = i as u16;
        assert_eq!(format!("{:?}", reg), format!("SU16({:04X})", i));
    }
}
