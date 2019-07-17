use pir_8_emu::isa::instruction::InstructionJumpCondition;
use self::super::satisfy;


#[test]
fn jmpz() {
    satisfy(InstructionJumpCondition::Jmpz, 0b00001, 0b11110, true);
}

#[test]
fn jmpp() {
    satisfy(InstructionJumpCondition::Jmpp, 0b00100, 0b11011, true);
}

#[test]
fn jmpg() {
    satisfy(InstructionJumpCondition::Jmpg, 0b10000, 0b01110, true);
}

#[test]
fn jmpc() {
    satisfy(InstructionJumpCondition::Jmpc, 0b00010, 0b11101, true);
}

#[test]
fn jmzg() {
    satisfy(InstructionJumpCondition::Jmzg, 0b10001, 0b01110, true);
    satisfy(InstructionJumpCondition::Jmzg, 0b10000, 0b01110, true);
    satisfy(InstructionJumpCondition::Jmzg, 0b00001, 0b01110, true);
}

#[test]
fn jmzl() {
    satisfy(InstructionJumpCondition::Jmzl, 0b10001, 0b01110, true);
    satisfy(InstructionJumpCondition::Jmzl, 0b00001, 0b01110, true);
    satisfy(InstructionJumpCondition::Jmzl, 0b00000, 0b01110, true);
}

#[test]
fn jmpl() {
    satisfy(InstructionJumpCondition::Jmpl, 0b00000, 0b01110, true);
}

#[test]
fn jump() {
    satisfy(InstructionJumpCondition::Jump, 0b00000, 0b11111, true);
}
