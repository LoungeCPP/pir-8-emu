use pir_8_emu::isa::instruction::InstructionJumpCondition;
use std::convert::TryFrom;


#[test]
fn jmpz() {
    assert_eq!(InstructionJumpCondition::try_from(0b000), Ok(InstructionJumpCondition::Jmpz));
}

#[test]
fn jmpp() {
    assert_eq!(InstructionJumpCondition::try_from(0b001), Ok(InstructionJumpCondition::Jmpp));
}

#[test]
fn jmpg() {
    assert_eq!(InstructionJumpCondition::try_from(0b010), Ok(InstructionJumpCondition::Jmpg));
}

#[test]
fn jmpc() {
    assert_eq!(InstructionJumpCondition::try_from(0b011), Ok(InstructionJumpCondition::Jmpc));
}

#[test]
fn jmzg() {
    assert_eq!(InstructionJumpCondition::try_from(0b100), Ok(InstructionJumpCondition::Jmzg));
}

#[test]
fn jmzl() {
    assert_eq!(InstructionJumpCondition::try_from(0b101), Ok(InstructionJumpCondition::Jmzl));
}

#[test]
fn jmpl() {
    assert_eq!(InstructionJumpCondition::try_from(0b110), Ok(InstructionJumpCondition::Jmpl));
}

#[test]
fn jump() {
    assert_eq!(InstructionJumpCondition::try_from(0b111), Ok(InstructionJumpCondition::Jump));
}
