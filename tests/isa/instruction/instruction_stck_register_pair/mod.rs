use pir_8_emu::isa::instruction::InstructionStckRegisterPair;


#[test]
fn parse() {
    assert_eq!(InstructionStckRegisterPair::from(false), InstructionStckRegisterPair::Ab);
    assert_eq!(InstructionStckRegisterPair::from(true), InstructionStckRegisterPair::Cd);
}

#[test]
fn serialise() {
    assert_eq!(InstructionStckRegisterPair::Ab as u8, 0b0);
    assert_eq!(InstructionStckRegisterPair::Cd as u8, 0b1);
}
