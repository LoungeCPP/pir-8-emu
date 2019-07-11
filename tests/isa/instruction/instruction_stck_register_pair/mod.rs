use pir_8_emu::isa::instruction::InstructionStckRegisterPair;


#[test]
fn parse() {
    assert_eq!(InstructionStckRegisterPair::from(false), InstructionStckRegisterPair::Ab);
    assert_eq!(InstructionStckRegisterPair::from(true), InstructionStckRegisterPair::Cd);
}
