use pir_8_emu::micro::{MicroOpPerformError, MicroOp};
use pir_8_emu::isa::instruction::AluOperation;
use self::super::universe;

mod sor_katt;


#[test]
fn not() {
    binary_nocarry(AluOperation::Not);
}

#[test]
fn or() {
    binary_nocarry(AluOperation::Or);
}

#[test]
fn xor() {
    binary_nocarry(AluOperation::Xor);
}

#[test]
fn and() {
    binary_nocarry(AluOperation::And);
}


fn binary_nocarry(op: AluOperation) {
    for stack_depth in 0..3 {
        for lhs in 0..=0xFFu8 {
            let rhs = lhs.wrapping_mul(3);

            let uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = uni_orig.clone();

            let mut stack = match stack_depth {
                0 => vec![],
                1 => vec![lhs],
                2 => vec![lhs, rhs],
                _ => unreachable!(),
            };

            assert_eq!(MicroOp::Alu(op).perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                       Err(MicroOpPerformError::MicrostackUnderflow));

            assert_eq!(memory, uni_orig.0);
            assert_eq!(ports, uni_orig.1);
            assert_eq!(registers, uni_orig.2);
            assert_eq!(pc, uni_orig.3);
            assert_eq!(sp, uni_orig.4);
            assert_eq!(adr, uni_orig.5);
            assert_eq!(ins, uni_orig.6);

            assert_eq!(stack, vec![]);
        }
    }
}
