use pir_8_emu::isa::instruction::InstructionRegisterPair;


#[test]
fn parse() {
    assert_eq!(InstructionRegisterPair::from(false), InstructionRegisterPair::Ab);
    assert_eq!(InstructionRegisterPair::from(true), InstructionRegisterPair::Cd);
}

#[test]
fn serialise() {
    assert_eq!(InstructionRegisterPair::Ab as u8, 0b0);
    assert_eq!(InstructionRegisterPair::Cd as u8, 0b1);
}

#[test]
fn display() {
    assert_eq!(InstructionRegisterPair::Ab.to_string(), "A&B");
    assert_eq!(InstructionRegisterPair::Cd.to_string(), "C&D");
}
