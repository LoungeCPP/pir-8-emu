use pir_8_emu::isa::instruction::InstructionJumpCondition;


#[test]
fn jmpz() {
    assert_eq!(InstructionJumpCondition::Jmpz as u8, 0b000);
}

#[test]
fn jmpp() {
    assert_eq!(InstructionJumpCondition::Jmpp as u8, 0b001);
}

#[test]
fn jmpg() {
    assert_eq!(InstructionJumpCondition::Jmpg as u8, 0b010);
}

#[test]
fn jmpc() {
    assert_eq!(InstructionJumpCondition::Jmpc as u8, 0b011);
}

#[test]
fn jmzg() {
    assert_eq!(InstructionJumpCondition::Jmzg as u8, 0b100);
}

#[test]
fn jmzl() {
    assert_eq!(InstructionJumpCondition::Jmzl as u8, 0b101);
}

#[test]
fn jmpl() {
    assert_eq!(InstructionJumpCondition::Jmpl as u8, 0b110);
}

#[test]
fn jump() {
    assert_eq!(InstructionJumpCondition::Jump as u8, 0b111);
}
