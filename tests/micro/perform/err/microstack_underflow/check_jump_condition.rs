use pir_8_emu::micro::{MicroOpPerformError, MicroOp};
use pir_8_emu::isa::instruction::InstructionJumpCondition;
use self::super::super::super::universe;


#[test]
fn jmpz() {
    satisfy(InstructionJumpCondition::Jmpz);
}

#[test]
fn jmpp() {
    satisfy(InstructionJumpCondition::Jmpp);
}

#[test]
fn jmpg() {
    satisfy(InstructionJumpCondition::Jmpg);
}

#[test]
fn jmpc() {
    satisfy(InstructionJumpCondition::Jmpc);
}

#[test]
fn jmzg() {
    satisfy(InstructionJumpCondition::Jmzg);
}

#[test]
fn jmzl() {
    satisfy(InstructionJumpCondition::Jmzl);
}

#[test]
fn jmpl() {
    satisfy(InstructionJumpCondition::Jmpl);
}

#[test]
fn jump() {
    satisfy(InstructionJumpCondition::Jump);
}


fn satisfy(cond: InstructionJumpCondition) {
    let uni_orig = universe();
    let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = uni_orig.clone();
    let mut stack = vec![];

    assert_eq!(MicroOp::CheckJumpCondition(cond).perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
               Err(MicroOpPerformError::MicrostackUnderflow));

    assert_eq!((memory, ports, registers, pc, sp, adr, ins), uni_orig);

    assert_eq!(stack, vec![]);
}
