use pir_8_emu::microcode::{MicrocodeExecutionError, MicroOp};
use self::super::super::universe;


#[test]
fn stack_push() {
    for i in 0..=0xFF {
        let addr = 0xFFFF;

        let mut uni_orig = universe();
        let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

        let mut stack = vec![i];
        *sp = addr;

        assert_eq!(MicroOp::StackPush.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                   Err(MicrocodeExecutionError::StackOverflow));

        *uni_orig.4 = addr;

        assert_eq!(memory, uni_orig.0);
        assert_eq!(ports, uni_orig.1);
        assert_eq!(registers, uni_orig.2);
        assert_eq!(pc, uni_orig.3);
        assert_eq!(sp, uni_orig.4);
        assert_eq!(adr, uni_orig.5);

        assert_eq!(stack, vec![]);
    }
}
