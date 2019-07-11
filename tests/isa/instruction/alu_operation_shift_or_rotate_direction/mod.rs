use pir_8_emu::isa::instruction::AluOperationShiftOrRotateDirection;


#[test]
fn parse() {
    assert_eq!(AluOperationShiftOrRotateDirection::from(false), AluOperationShiftOrRotateDirection::Right);
    assert_eq!(AluOperationShiftOrRotateDirection::from(true), AluOperationShiftOrRotateDirection::Left);
}
