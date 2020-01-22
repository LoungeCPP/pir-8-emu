use pir_8_emu::binutils::pir_8_as::LabelFragment;


#[test]
fn len() {
    assert_eq!(LabelFragment::Full.len(), 2);

    assert_eq!(LabelFragment::High.len(), 1);
    assert_eq!(LabelFragment::Low.len(), 1);
}
