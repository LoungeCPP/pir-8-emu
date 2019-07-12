use pir_8_emu::isa::instruction::AluOperationShiftOrRotateType;


#[test]
fn lsf() {
    assert_eq!(AluOperationShiftOrRotateType::Lsf as u8, 0b00);
}

#[test]
fn asf() {
    assert_eq!(AluOperationShiftOrRotateType::Asf as u8, 0b01);
}

#[test]
fn rtc() {
    assert_eq!(AluOperationShiftOrRotateType::Rtc as u8, 0b10);
}

#[test]
fn rtw() {
    assert_eq!(AluOperationShiftOrRotateType::Rtw as u8, 0b11);
}
