use pir_8_emu::micro::{MicroOpPerformError, MicroOp};
use self::super::super::universe;


#[test]
fn load_immediate() {
    let addr = 0xFFFF;

    let mut uni_orig = universe();
    let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

    let mut stack = vec![];
    *pc = addr;

    assert_eq!(MicroOp::LoadImmediate.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
               Err(MicroOpPerformError::ProgramOverflow));

    let _read_pc = *uni_orig.3;
    *uni_orig.3 = addr;
    let _read_adr = *uni_orig.5;
    *uni_orig.5 = addr;

    let _read_mem = uni_orig.0[0xFFFF];

    assert_eq!(memory, uni_orig.0);
    assert_eq!(ports, uni_orig.1);
    assert_eq!(registers, uni_orig.2);
    assert_eq!(pc, uni_orig.3);
    assert_eq!(sp, uni_orig.4);
    assert_eq!(adr, uni_orig.5);
    assert_eq!(ins, uni_orig.6);

    assert_eq!(stack, vec![]);
}