use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, AluOperation};


#[test]
fn add() {
    let raw: u8 = AluOperation::Add.into();
    assert_eq!(raw, 0b0000);
}

#[test]
fn sub() {
    let raw: u8 = AluOperation::Sub.into();
    assert_eq!(raw, 0b0001);
}

#[test]
fn not() {
    let raw: u8 = AluOperation::Not.into();
    assert_eq!(raw, 0b0010);
}

#[test]
fn or() {
    let raw: u8 = AluOperation::Or.into();
    assert_eq!(raw, 0b0100);
}

#[test]
fn xor() {
    let raw: u8 = AluOperation::Xor.into();
    assert_eq!(raw, 0b0101);
}

#[test]
fn and() {
    let raw: u8 = AluOperation::And.into();
    assert_eq!(raw, 0b0110);
}


#[test]
fn shift_or_rotate_lsf() {
    let raw: u8 = AluOperation::ShiftOrRotate {
            d: AluOperationShiftOrRotateDirection::Right,
            tt: AluOperationShiftOrRotateType::Lsf,
        }
        .into();
    assert_eq!(raw, 0b1000);

    let raw: u8 = AluOperation::ShiftOrRotate {
            d: AluOperationShiftOrRotateDirection::Left,
            tt: AluOperationShiftOrRotateType::Lsf,
        }
        .into();
    assert_eq!(raw, 0b1100);
}

#[test]
fn shift_or_rotate_asf() {
    let raw: u8 = AluOperation::ShiftOrRotate {
            d: AluOperationShiftOrRotateDirection::Right,
            tt: AluOperationShiftOrRotateType::Asf,
        }
        .into();
    assert_eq!(raw, 0b1001);

    let raw: u8 = AluOperation::ShiftOrRotate {
            d: AluOperationShiftOrRotateDirection::Left,
            tt: AluOperationShiftOrRotateType::Asf,
        }
        .into();
    assert_eq!(raw, 0b1101);
}

#[test]
fn shift_or_rotate_rtc() {
    let raw: u8 = AluOperation::ShiftOrRotate {
            d: AluOperationShiftOrRotateDirection::Right,
            tt: AluOperationShiftOrRotateType::Rtc,
        }
        .into();
    assert_eq!(raw, 0b1010);

    let raw: u8 = AluOperation::ShiftOrRotate {
            d: AluOperationShiftOrRotateDirection::Left,
            tt: AluOperationShiftOrRotateType::Rtc,
        }
        .into();
    assert_eq!(raw, 0b1110);
}

#[test]
fn shift_or_rotate_rtw() {
    let raw: u8 = AluOperation::ShiftOrRotate {
            d: AluOperationShiftOrRotateDirection::Right,
            tt: AluOperationShiftOrRotateType::Rtw,
        }
        .into();
    assert_eq!(raw, 0b1011);

    let raw: u8 = AluOperation::ShiftOrRotate {
            d: AluOperationShiftOrRotateDirection::Left,
            tt: AluOperationShiftOrRotateType::Rtw,
        }
        .into();
    assert_eq!(raw, 0b1111);
}


#[test]
fn reserved_block_0() {
    let raw: u8 = AluOperation::Reserved(0b0011).into();
    assert_eq!(raw, 0b0011);
}

#[test]
fn reserved_block_1() {
    let raw: u8 = AluOperation::Reserved(0b0111).into();
    assert_eq!(raw, 0b0111);
}
