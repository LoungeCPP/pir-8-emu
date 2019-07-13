use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, AluOperation};
use std::convert::TryFrom;
use std::str::FromStr;


#[test]
fn raw() {
    for i in 0..0b1111 {
        assert_eq!(AluOperation::from_str(&format!("{}", i)), Ok(AluOperation::try_from(i as u8).unwrap()));
        assert_eq!(AluOperation::from_str(&format!("{:#0x}", i)), Ok(AluOperation::try_from(i as u8).unwrap()));
        assert_eq!(AluOperation::from_str(&format!("{:#0X}", i)), Ok(AluOperation::try_from(i as u8).unwrap()));
        assert_eq!(AluOperation::from_str(&format!("{:#0o}", i)), Ok(AluOperation::try_from(i as u8).unwrap()));
        assert_eq!(AluOperation::from_str(&format!("{:#0b}", i)), Ok(AluOperation::try_from(i as u8).unwrap()));
    }
}

#[test]
fn add() {
    assert_eq!(AluOperation::from_str("ADD"), Ok(AluOperation::Add));
}

#[test]
fn sub() {
    assert_eq!(AluOperation::from_str("SUB"), Ok(AluOperation::Sub));
}

#[test]
fn not() {
    assert_eq!(AluOperation::from_str("NOT"), Ok(AluOperation::Not));
}

#[test]
fn or() {
    assert_eq!(AluOperation::from_str("OR"), Ok(AluOperation::Or));
}

#[test]
fn xor() {
    assert_eq!(AluOperation::from_str("XOR"), Ok(AluOperation::Xor));
}

#[test]
fn and() {
    assert_eq!(AluOperation::from_str("AND"), Ok(AluOperation::And));
}

#[test]
fn sor() {
    assert_eq!(AluOperation::from_str("SOR LEFT LSF"),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Left,
                   tt: AluOperationShiftOrRotateType::Lsf,
               }));
    assert_eq!(AluOperation::from_str("SOR LEFT ASF"),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Left,
                   tt: AluOperationShiftOrRotateType::Asf,
               }));
    assert_eq!(AluOperation::from_str("SOR LEFT RTC"),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Left,
                   tt: AluOperationShiftOrRotateType::Rtc,
               }));
    assert_eq!(AluOperation::from_str("SOR LEFT RTW"),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Left,
                   tt: AluOperationShiftOrRotateType::Rtw,
               }));
    assert_eq!(AluOperation::from_str("SOR RIGHT LSF"),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Right,
                   tt: AluOperationShiftOrRotateType::Lsf,
               }));
    assert_eq!(AluOperation::from_str("SOR RIGHT ASF"),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Right,
                   tt: AluOperationShiftOrRotateType::Asf,
               }));
    assert_eq!(AluOperation::from_str("SOR RIGHT RTC"),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Right,
                   tt: AluOperationShiftOrRotateType::Rtc,
               }));
    assert_eq!(AluOperation::from_str("SOR RIGHT RTW"),
               Ok(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Right,
                   tt: AluOperationShiftOrRotateType::Rtw,
               }));
}
