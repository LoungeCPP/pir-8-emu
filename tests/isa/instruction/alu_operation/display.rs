use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, AluOperation};


#[test]
fn add() {
    assert_eq!(AluOperation::Add.to_string(), "ADD");
}

#[test]
fn sub() {
    assert_eq!(AluOperation::Sub.to_string(), "SUB");
}

#[test]
fn not() {
    assert_eq!(AluOperation::Not.to_string(), "NOT");
}

#[test]
fn or() {
    assert_eq!(AluOperation::Or.to_string(), "OR");
}

#[test]
fn xor() {
    assert_eq!(AluOperation::Xor.to_string(), "XOR");
}

#[test]
fn and() {
    assert_eq!(AluOperation::And.to_string(), "AND");
}


#[test]
fn shift_or_rotate_lsf() {
    assert_eq!(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Right,
                   tt: AluOperationShiftOrRotateType::Lsf,
               }.to_string(), "SOR RIGHT LSF");

    assert_eq!(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Left,
                   tt: AluOperationShiftOrRotateType::Lsf,
               }.to_string(), "SOR LEFT LSF");
}

#[test]
fn shift_or_rotate_asf() {
    assert_eq!(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Right,
                   tt: AluOperationShiftOrRotateType::Asf,
               }.to_string(), "SOR RIGHT ASF");

    assert_eq!(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Left,
                   tt: AluOperationShiftOrRotateType::Asf,
               }.to_string(), "SOR LEFT ASF");
}

#[test]
fn shift_or_rotate_rtc() {
    assert_eq!(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Right,
                   tt: AluOperationShiftOrRotateType::Rtc,
               }.to_string(), "SOR RIGHT RTC");

    assert_eq!(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Left,
                   tt: AluOperationShiftOrRotateType::Rtc,
               }.to_string(), "SOR LEFT RTC");
}

#[test]
fn shift_or_rotate_rtw() {
    assert_eq!(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Right,
                   tt: AluOperationShiftOrRotateType::Rtw,
               }.to_string(), "SOR RIGHT RTW");

    assert_eq!(AluOperation::ShiftOrRotate {
                   d: AluOperationShiftOrRotateDirection::Left,
                   tt: AluOperationShiftOrRotateType::Rtw,
               }.to_string(), "SOR LEFT RTW");
}


#[test]
fn reserved_block_0() {
    assert_eq!(AluOperation::Reserved(0b0011).to_string(), "0b0011");
}

#[test]
fn reserved_block_1() {
    assert_eq!(AluOperation::Reserved(0b0111).to_string(), "0b0111");
}
