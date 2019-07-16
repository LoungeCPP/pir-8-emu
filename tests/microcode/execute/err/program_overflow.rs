use pir_8_emu::microcode::{MicrocodeExecutionError, MicroOp};
use self::super::super::universe;


#[test]
fn load_immediate() {
    let addr = 0xFFFF;

    let mut uni_orig = universe();
    let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

    let mut stack = vec![];
    *pc = addr;

    assert_eq!(MicroOp::LoadImmediate.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
               Err(MicrocodeExecutionError::ProgramOverflow));

    *uni_orig.3 = addr;

    assert_eq!(memory, uni_orig.0);
    assert_eq!(ports, uni_orig.1);
    assert_eq!(registers, uni_orig.2);
    assert_eq!(pc, uni_orig.3);
    assert_eq!(sp, uni_orig.4);
    assert_eq!(adr, uni_orig.5);

    assert_eq!(stack, vec![]);
}

#[test]
fn jump_not_ok() {
    for &start_addr in &[0xFFFF, 0xFFFF - 1] {
        for dest_addr in 0..0x11u16 {
            let dest_addr = dest_addr | ((dest_addr + 1) << 8);

            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

            let mut stack = vec![dest_addr as u8 + 1, dest_addr as u8, 0];
            *pc = start_addr;

            assert_eq!(MicroOp::Jump.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                       Err(MicrocodeExecutionError::ProgramOverflow));

            *uni_orig.3 = start_addr;

            assert_eq!(memory, uni_orig.0);
            assert_eq!(ports, uni_orig.1);
            assert_eq!(registers, uni_orig.2);
            assert_eq!(pc, uni_orig.3);
            assert_eq!(sp, uni_orig.4);
            assert_eq!(adr, uni_orig.5);

            assert_eq!(stack, vec![]);
        }
    }
}
