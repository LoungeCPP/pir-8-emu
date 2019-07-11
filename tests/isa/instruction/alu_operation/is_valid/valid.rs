use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, AluOperation};

#[test]
fn add() {
    assert!(AluOperation::Add.is_valid());
}

#[test]
fn sub() {
    assert!(AluOperation::Sub.is_valid());
}

#[test]
fn not() {
    assert!(AluOperation::Not.is_valid());
}

#[test]
fn or() {
    assert!(AluOperation::Or.is_valid());
}

#[test]
fn xor() {
    assert!(AluOperation::Xor.is_valid());
}

#[test]
fn and() {
    assert!(AluOperation::And.is_valid());
}

#[test]
fn shift_or_rotate() {
    for &d in &[AluOperationShiftOrRotateDirection::Right, AluOperationShiftOrRotateDirection::Left] {
        for &tt in &[AluOperationShiftOrRotateType::Lsf,
                     AluOperationShiftOrRotateType::Asf,
                     AluOperationShiftOrRotateType::Rtc,
                     AluOperationShiftOrRotateType::Rtw] {
            assert!(AluOperation::ShiftOrRotate { d: d, tt: tt }.is_valid());
        }
    }
}
