use pir_8_emu::isa::instruction::AluOperationShiftOrRotateDirection;


#[test]
fn parse() {
    assert_eq!(AluOperationShiftOrRotateDirection::from(false), AluOperationShiftOrRotateDirection::Right);
    assert_eq!(AluOperationShiftOrRotateDirection::from(true), AluOperationShiftOrRotateDirection::Left);
}

#[test]
fn serialise() {
    assert_eq!(AluOperationShiftOrRotateDirection::Right as u8, 0b000);
    assert_eq!(AluOperationShiftOrRotateDirection::Left as u8, 0b100);
}
