use pir_8_emu::isa::instruction::InstructionMadrDirection;


#[test]
fn parse() {
    assert_eq!(InstructionMadrDirection::from(false), InstructionMadrDirection::Write);
    assert_eq!(InstructionMadrDirection::from(true), InstructionMadrDirection::Read);
}

#[test]
fn serialise() {
    assert_eq!(InstructionMadrDirection::Write as u8, 0b00);
    assert_eq!(InstructionMadrDirection::Read as u8, 0b10);
}

#[test]
fn display() {
    assert_eq!(InstructionMadrDirection::Write.to_string(), "WRITE");
    assert_eq!(InstructionMadrDirection::Read.to_string(), "READ");
}
