use pir_8_emu::isa::instruction::AluOperation;
use pir_8_emu::microcode::MicroOp;
use self::super::universe;

mod sor_katt;


#[test]
fn not() {
    binary_nocarry(AluOperation::Not, |lhs, _| !lhs);
}

#[test]
fn or() {
    binary_nocarry(AluOperation::Or, |lhs, rhs| lhs | rhs);
}

#[test]
fn xor() {
    binary_nocarry(AluOperation::Xor, |lhs, rhs| lhs ^ rhs);
}

#[test]
fn and() {
    binary_nocarry(AluOperation::And, |lhs, rhs| lhs & rhs);
}


fn binary_nocarry(op: AluOperation, exp: fn(u8, u8) -> u8) {
    for flags_start in 0..=0b11111 {
        for lhs in 0..=0xFFu8 {
            let rhs = lhs.wrapping_mul(3);

            let uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = uni_orig.clone();

            let mut stack = vec![lhs, rhs, flags_start];

            assert_eq!(MicroOp::Alu(op).execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                       Ok(true));

            assert_eq!(memory, uni_orig.0);
            assert_eq!(ports, uni_orig.1);
            assert_eq!(registers, uni_orig.2);
            assert_eq!(pc, uni_orig.3);
            assert_eq!(sp, uni_orig.4);
            assert_eq!(adr, uni_orig.5);
            assert_eq!(ins, uni_orig.6);

            let val = exp(lhs, rhs);

            assert_eq!(stack,
                       vec![val,
                            (flags_start & 0b11010) | (if val == 0 { 0b00001 } else { 0b00000 }) |
                            (if val.count_ones() % 2 == 0 {
                                0b00100
                            } else {
                                0b00000
                            })]);
        }
    }
}
