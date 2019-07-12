use pir_8_emu::isa::instruction::InstructionJumpCondition;


#[test]
fn jmpz() {
    assert_eq!(InstructionJumpCondition::Jmpz.to_string(), "JMPZ");
}

#[test]
fn jmpp() {
    assert_eq!(InstructionJumpCondition::Jmpp.to_string(), "JMPP");
}

#[test]
fn jmpg() {
    assert_eq!(InstructionJumpCondition::Jmpg.to_string(), "JMPG");
}

#[test]
fn jmpc() {
    assert_eq!(InstructionJumpCondition::Jmpc.to_string(), "JMPC");
}

#[test]
fn jmzg() {
    assert_eq!(InstructionJumpCondition::Jmzg.to_string(), "JMZG");
}

#[test]
fn jmzl() {
    assert_eq!(InstructionJumpCondition::Jmzl.to_string(), "JMZL");
}

#[test]
fn jmpl() {
    assert_eq!(InstructionJumpCondition::Jmpl.to_string(), "JMPL");
}

#[test]
fn jump() {
    assert_eq!(InstructionJumpCondition::Jump.to_string(), "JUMP");
}
