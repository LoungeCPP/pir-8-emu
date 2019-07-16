use pir_8_emu::isa::instruction::InstructionJumpCondition;
use self::super::satisfy;


#[test]
fn jmpz() {
    satisfy(InstructionJumpCondition::Jmpz, 0b00000, 0b11110, false);
}

#[test]
fn jmpp() {
    satisfy(InstructionJumpCondition::Jmpp, 0b00000, 0b11011, false);
}

#[test]
fn jmpg() {
    satisfy(InstructionJumpCondition::Jmpg, 0b00001, 0b01110, false);
    satisfy(InstructionJumpCondition::Jmpg, 0b10001, 0b01110, false);
    satisfy(InstructionJumpCondition::Jmpg, 0b00000, 0b01110, false);
}

#[test]
fn jmpc() {
    satisfy(InstructionJumpCondition::Jmpc, 0b00000, 0b11101, false);
}

#[test]
fn jmzg() {
    satisfy(InstructionJumpCondition::Jmzg, 0b00000, 0b01110, false);
}

#[test]
fn jmzl() {
    satisfy(InstructionJumpCondition::Jmzl, 0b10000, 0b01110, false);
}

#[test]
fn jmpl() {
    satisfy(InstructionJumpCondition::Jmpl, 0b10000, 0b01110, false);
    satisfy(InstructionJumpCondition::Jmpl, 0b00001, 0b01110, false);
    satisfy(InstructionJumpCondition::Jmpl, 0b10001, 0b01110, false);
}
