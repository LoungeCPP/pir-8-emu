use pir_8_emu::isa::instruction::InstructionPortDirection;


#[test]
fn parse() {
    assert_eq!(InstructionPortDirection::from(true), InstructionPortDirection::In);
    assert_eq!(InstructionPortDirection::from(false), InstructionPortDirection::Out);
}

#[test]
fn serialise() {
    assert_eq!(InstructionPortDirection::In as u8, 0b1000);
    assert_eq!(InstructionPortDirection::Out as u8, 0b0000);
}

#[test]
fn display() {
    assert_eq!(InstructionPortDirection::In.to_string(), "IN");
    assert_eq!(InstructionPortDirection::Out.to_string(), "OUT");
}
