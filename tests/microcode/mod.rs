use pir_8_emu::microcode::{MicroOp, NEXT_INSTRUCTION};

mod from_instruction;
mod display;
mod execute;
mod error;


#[test]
fn next_instruction() {
    let ops = NEXT_INSTRUCTION;
    let ops = &ops.0[..ops.1];

    assert_eq!(ops, &[MicroOp::LoadImmediate, MicroOp::LoadInstruction]);
}
