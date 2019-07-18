use pir_8_emu::micro::{MicroOpPerformError, MicroOp};
use self::super::super::universe;

mod check_jump_condition;
mod alu;


#[test]
fn load_instruction() {
    let uni_orig = universe();
    let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = uni_orig.clone();

    let mut stack = vec![];

    assert_eq!(MicroOp::LoadInstruction.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
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

#[test]
fn stack_push() {
    for addr in 0..0x11u16 {
        let addr = addr | ((addr + 1) << 8);

        let mut uni_orig = universe();
        let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = uni_orig.clone();

        let mut stack = vec![];
        *sp = addr;

        assert_eq!(MicroOp::StackPush.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                   Err(MicroOpPerformError::MicrostackUnderflow));

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

#[test]
fn port_in() {
    for i in 1..=0xFF {
        for port in 0..=0xFF {
            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = uni_orig.clone();

            let mut stack = vec![];
            ports[port as usize] = i;

            assert_eq!(MicroOp::PortIn.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                       Err(MicroOpPerformError::MicrostackUnderflow));

            uni_orig.1[port as usize] = i;

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

#[test]
fn port_out() {
    for stack_depth in 0..2 {
        for i in 1..=0xFF {
            let uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = uni_orig.clone();

            let mut stack = match stack_depth {
                0 => vec![],
                1 => vec![i],
                _ => unreachable!(),
            };

            assert_eq!(MicroOp::PortOut.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
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

#[test]
fn compare() {
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

            assert_eq!(MicroOp::Compare.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
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

#[test]
fn fetch_address() {
    for stack_depth in 0..2 {
        for i in 1..=0xFF {
            for addr in 0..0x11u16 {
                let addr = addr | ((addr + 1) << 8);

                let mut uni_orig = universe();
                let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = uni_orig.clone();

                let mut stack = match stack_depth {
                    0 => vec![],
                    1 => vec![addr as u8 + 1],
                    _ => unreachable!(),
                };
                memory[addr] = i;

                assert_eq!(MicroOp::FetchAddress.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                           Err(MicroOpPerformError::MicrostackUnderflow));

                uni_orig.0[addr] = i;

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
}

#[test]
fn write_address() {
    for stack_depth in 0..3 {
        for i in 1..=0xFF {
            for addr in 0..0x11u16 {
                let addr = addr | ((addr + 1) << 8);

                let uni_orig = universe();
                let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = uni_orig.clone();

                let mut stack = match stack_depth {
                    0 => vec![],
                    1 => vec![i],
                    2 => vec![i, addr as u8 + 1],
                    _ => unreachable!(),
                };

                assert_eq!(MicroOp::WriteAddress.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
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
}

#[test]
fn jump() {
    for stack_depth in 0..3 {
        for start_addr in 0x11..0x22u16 {
            let start_addr = start_addr | ((start_addr + 1) << 8);

            for dest_addr in 0..0x11u16 {
                let dest_addr = dest_addr | ((dest_addr + 1) << 8);

                let mut uni_orig = universe();
                let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = uni_orig.clone();

                let mut stack = match stack_depth {
                    0 => vec![],
                    1 => vec![dest_addr as u8 + 1],
                    2 => vec![dest_addr as u8 + 1, dest_addr as u8],
                    _ => unreachable!(),
                };
                *pc = start_addr;

                assert_eq!(MicroOp::Jump.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                           Err(MicroOpPerformError::MicrostackUnderflow));

                *uni_orig.3 = start_addr;

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
}

#[test]
fn write_register() {
    for aaa in 0..0b111 {
        let uni_orig = universe();
        let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = uni_orig.clone();

        let mut stack = vec![];

        assert_eq!(MicroOp::WriteRegister(aaa).perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
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
