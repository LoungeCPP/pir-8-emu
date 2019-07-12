use pir_8_emu::isa::instruction::InstructionStckDirection;


#[test]
fn parse() {
    assert_eq!(InstructionStckDirection::from(false), InstructionStckDirection::Push);
    assert_eq!(InstructionStckDirection::from(true), InstructionStckDirection::Pop);
}

#[test]
fn serialise() {
    assert_eq!(InstructionStckDirection::Push as u8, 0b00);
    assert_eq!(InstructionStckDirection::Pop as u8, 0b10);
}

#[test]
fn display() {
    assert_eq!(InstructionStckDirection::Push.to_string(), "PUSH");
    assert_eq!(InstructionStckDirection::Pop.to_string(), "POP");
}
