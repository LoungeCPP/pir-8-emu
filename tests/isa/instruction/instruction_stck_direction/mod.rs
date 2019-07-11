use pir_8_emu::isa::instruction::InstructionStckDirection;


#[test]
fn parse() {
    assert_eq!(InstructionStckDirection::from(false), InstructionStckDirection::Push);
    assert_eq!(InstructionStckDirection::from(true), InstructionStckDirection::Pop);
}
