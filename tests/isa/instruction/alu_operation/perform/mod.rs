use pir_8_emu::isa::instruction::AluOperation;

mod sor_katt;


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

#[test]
fn not() {
    binary_nocarry(AluOperation::Not, |lhs, _| !lhs);
}


fn binary_nocarry(op: AluOperation, exp: fn(u8, u8) -> u8) {
    for &carry_start in &[false, true] {
        for lhs in 0..=0xFF {
            for rhs in 0..=0xFF {
                let mut carry = carry_start;

                assert_eq!(op.perform(lhs, rhs, &mut carry), exp(lhs, rhs));
                assert_eq!(carry, carry_start);
            }
        }
    }
}
