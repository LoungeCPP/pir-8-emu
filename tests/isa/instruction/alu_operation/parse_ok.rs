use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, AluOperation};
use std::convert::TryFrom;


#[test]
fn add() {
    assert_eq!(AluOperation::try_from(0b0000), Ok(AluOperation::Add));
}

#[test]
fn sub() {
    assert_eq!(AluOperation::try_from(0b0001), Ok(AluOperation::Sub));
}

#[test]
fn addc() {
    assert_eq!(AluOperation::try_from(0b0010), Ok(AluOperation::AddC));
}

#[test]
fn subc() {
    assert_eq!(AluOperation::try_from(0b0011), Ok(AluOperation::SubC));
}

#[test]
fn or() {
    assert_eq!(AluOperation::try_from(0b0100), Ok(AluOperation::Or));
}

#[test]
fn xor() {
    assert_eq!(AluOperation::try_from(0b0101), Ok(AluOperation::Xor));
}

#[test]
fn and() {
    assert_eq!(AluOperation::try_from(0b0110), Ok(AluOperation::And));
}

#[test]
fn not() {
    assert_eq!(AluOperation::try_from(0b0111), Ok(AluOperation::Not));
}


#[test]
fn shift_or_rotate_lsf() {
    assert_eq!(AluOperation::try_from(0b1000),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Right,
                   tt: AluOperationShiftOrRotateType::Lsf,
               }));

    assert_eq!(AluOperation::try_from(0b1100),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Left,
                   tt: AluOperationShiftOrRotateType::Lsf,
               }));
}

#[test]
fn shift_or_rotate_asf() {
    assert_eq!(AluOperation::try_from(0b1001),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Right,
                   tt: AluOperationShiftOrRotateType::Asf,
               }));

    assert_eq!(AluOperation::try_from(0b1101),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Left,
                   tt: AluOperationShiftOrRotateType::Asf,
               }));
}

#[test]
fn shift_or_rotate_rtc() {
    assert_eq!(AluOperation::try_from(0b1010),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Right,
                   tt: AluOperationShiftOrRotateType::Rtc,
               }));

    assert_eq!(AluOperation::try_from(0b1110),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Left,
                   tt: AluOperationShiftOrRotateType::Rtc,
               }));
}

#[test]
fn shift_or_rotate_rtw() {
    assert_eq!(AluOperation::try_from(0b1011),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Right,
                   tt: AluOperationShiftOrRotateType::Rtw,
               }));

    assert_eq!(AluOperation::try_from(0b1111),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Left,
                   tt: AluOperationShiftOrRotateType::Rtw,
               }));
}
