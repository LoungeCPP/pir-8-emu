use pir_8_emu::isa::instruction::AluOperationShiftOrRotateType;
use std::convert::TryFrom;


#[test]
fn lsf() {
    assert_eq!(AluOperationShiftOrRotateType::try_from(0b00), Ok(AluOperationShiftOrRotateType::Lsf));
}

#[test]
fn asf() {
    assert_eq!(AluOperationShiftOrRotateType::try_from(0b01), Ok(AluOperationShiftOrRotateType::Asf));
}

#[test]
fn rtc() {
    assert_eq!(AluOperationShiftOrRotateType::try_from(0b10), Ok(AluOperationShiftOrRotateType::Rtc));
}

#[test]
fn rtw() {
    assert_eq!(AluOperationShiftOrRotateType::try_from(0b11), Ok(AluOperationShiftOrRotateType::Rtw));
}
