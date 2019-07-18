use pir_8_emu::micro::{MicroOpPerformError, MicroOp};
use self::super::super::universe;


#[test]
fn stack_push() {
    for i in 0..=0xFF {
        let addr = 0xFFFF;

        let mut uni_orig = universe();
        let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = uni_orig.clone();

        let mut stack = vec![i];
        *sp = addr;

        assert_eq!(MicroOp::StackPush.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                   Err(MicroOpPerformError::StackOverflow));

        let _read_sp = *uni_orig.4;
        *uni_orig.4 = addr;

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
