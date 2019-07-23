use self::super::super::universe;
use pir_8_emu::micro::MicroOp;


#[test]
fn stack_push() {
    for i in 0..=0xFF {
        let addr = 0xFFFF;

        let mut uni_orig = universe();
        let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

        let mut stack = vec![i];
        *sp = addr;

        assert_eq!(MicroOp::StackPush.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                   Ok(true));

        uni_orig.0[0x0000] = i;
        let _read_sp = *uni_orig.4;
        *uni_orig.4 = 0x0000;
        let _read_adr = *uni_orig.5;
        *uni_orig.5 = 0x0000;

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

#[test]
fn stack_pop() {
    for i in 0..=0xFF {
        let addr = 0x0000;

        let mut uni_orig = universe();
        let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

        let mut stack = vec![];
        memory[addr] = i;
        *sp = addr;

        assert_eq!(MicroOp::StackPop.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                   Ok(true));

        let _read_mem = uni_orig.0[addr];
        uni_orig.0[addr] = i;
        let _read_sp = *uni_orig.4;
        *uni_orig.4 = 0xFFFF;
        *uni_orig.5 = *uni_orig.5;

        assert_eq!(memory, uni_orig.0);
        assert_eq!(ports, uni_orig.1);
        assert_eq!(registers, uni_orig.2);
        assert_eq!(pc, uni_orig.3);
        assert_eq!(sp, uni_orig.4);
        assert_eq!(adr, uni_orig.5);
        assert_eq!(ins, uni_orig.6);

        assert_eq!(stack, vec![i]);
    }
}

#[test]
fn load_immediate() {
    for i in 0..=0xFF {
        let addr = 0xFFFF;

        let mut uni_orig = universe();
        let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

        let mut stack = vec![];
        *pc = addr;
        memory[addr] = i;

        assert_eq!(MicroOp::LoadImmediate.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                   Ok(true));

        let _read_mem = uni_orig.0[0xFFFF];
        uni_orig.0[addr] = i;
        let _read_pc = *uni_orig.3;
        *uni_orig.3 = 0x0000;
        let _read_adr = *uni_orig.5;
        *uni_orig.5 = addr;

        assert_eq!(memory, uni_orig.0);
        assert_eq!(ports, uni_orig.1);
        assert_eq!(registers, uni_orig.2);
        assert_eq!(pc, uni_orig.3);
        assert_eq!(sp, uni_orig.4);
        assert_eq!(adr, uni_orig.5);
        assert_eq!(ins, uni_orig.6);

        assert_eq!(stack, vec![i]);
    }
}
