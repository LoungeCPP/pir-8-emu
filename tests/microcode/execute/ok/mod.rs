use pir_8_emu::microcode::MicroOp;
use self::super::universe;

mod check_jump_condition;
mod jump;
mod alu;


#[test]
fn nop() {
    let uni_orig = universe();
    let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();
    let mut stack = vec![];

    assert_eq!(MicroOp::Nop.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
               Ok(true));

    assert_eq!((memory, ports, registers, pc, sp, adr), uni_orig);

    assert_eq!(stack, vec![]);
}

#[test]
fn halt() {
    let uni_orig = universe();
    let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();
    let mut stack = vec![];

    assert_eq!(MicroOp::Halt.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
               Ok(false));

    assert_eq!((memory, ports, registers, pc, sp, adr), uni_orig);

    assert_eq!(stack, vec![]);
}

#[test]
fn stack_push() {
    for i in 1..=0xFF {
        for addr in 0..0x11u16 {
            let addr = addr | ((addr + 1) << 8);

            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

            let mut stack = vec![i];
            *sp = addr;

            assert_eq!(MicroOp::StackPush.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                       Ok(true));

            uni_orig.0[addr as usize + 1] = i;
            *uni_orig.4 = addr + 1;

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

#[test]
fn stack_pop() {
    for i in 1..=0xFF {
        for addr in 1..0x12u16 {
            let addr = addr | ((addr + 1) << 8);

            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

            let mut stack = vec![];
            memory[addr as usize] = i;
            *sp = addr;

            assert_eq!(MicroOp::StackPop.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                       Ok(true));

            uni_orig.0[addr as usize] = i;
            *uni_orig.4 = addr - 1;

            assert_eq!(memory, uni_orig.0);
            assert_eq!(ports, uni_orig.1);
            assert_eq!(registers, uni_orig.2);
            assert_eq!(pc, uni_orig.3);
            assert_eq!(sp, uni_orig.4);
            assert_eq!(adr, uni_orig.5);

            assert_eq!(stack, vec![i]);
        }
    }
}

#[test]
fn port_in() {
    for i in 1..=0xFF {
        for port in 0..=0xFF {
            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

            let mut stack = vec![port];
            ports[port as usize] = i;

            assert_eq!(MicroOp::PortIn.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                       Ok(true));

            uni_orig.1[port as usize] = i;

            assert_eq!(memory, uni_orig.0);
            assert_eq!(ports, uni_orig.1);
            assert_eq!(registers, uni_orig.2);
            assert_eq!(pc, uni_orig.3);
            assert_eq!(sp, uni_orig.4);
            assert_eq!(adr, uni_orig.5);

            assert_eq!(stack, vec![i]);
        }
    }
}

#[test]
fn port_out() {
    for i in 1..=0xFF {
        for port in 0..=0xFF {
            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

            let mut stack = vec![i, port];

            assert_eq!(MicroOp::PortOut.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                       Ok(true));

            uni_orig.1[port as usize] = i;

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

#[test]
fn compare() {
    for flags_start in 0..=0b11111 {
        for lhs in 0..=0xFFu8 {
            let rhs = lhs.wrapping_mul(3);

            let uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

            let mut stack = vec![lhs, rhs, flags_start];

            assert_eq!(MicroOp::Compare.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                       Ok(true));

            assert_eq!(memory, uni_orig.0);
            assert_eq!(ports, uni_orig.1);
            assert_eq!(registers, uni_orig.2);
            assert_eq!(pc, uni_orig.3);
            assert_eq!(sp, uni_orig.4);
            assert_eq!(adr, uni_orig.5);

            assert_eq!(stack,
                       vec![(flags_start & 0b00010) | (if lhs == 0 { 0b00001 } else { 0b00000 }) |
                            (if lhs.count_ones() % 2 == 0 {
                                0b00100
                            } else {
                                0b00000
                            }) | (if lhs == rhs { 0b01000 } else { 0b00000 }) | (if lhs > rhs { 0b10000 } else { 0b00000 })]);
        }
    }
}

#[test]
fn make_immediate() {
    for i in 0..=0xFF {
        let uni_orig = universe();
        let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

        let mut stack = vec![];

        assert_eq!(MicroOp::MakeImmediate(i).execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                   Ok(true));

        assert_eq!(memory, uni_orig.0);
        assert_eq!(ports, uni_orig.1);
        assert_eq!(registers, uni_orig.2);
        assert_eq!(pc, uni_orig.3);
        assert_eq!(sp, uni_orig.4);
        assert_eq!(adr, uni_orig.5);

        assert_eq!(stack, vec![i]);
    }
}

#[test]
fn load_immediate() {
    for i in 1..=0xFF {
        for addr in 0..0x11u16 {
            let addr = addr | ((addr + 1) << 8);

            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

            let mut stack = vec![];
            *pc = addr;
            memory[addr as usize + 1] = i;

            assert_eq!(MicroOp::LoadImmediate.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                       Ok(true));

            uni_orig.0[addr as usize + 1] = i;
            *uni_orig.3 = addr + 1;

            assert_eq!(memory, uni_orig.0);
            assert_eq!(ports, uni_orig.1);
            assert_eq!(registers, uni_orig.2);
            assert_eq!(pc, uni_orig.3);
            assert_eq!(sp, uni_orig.4);
            assert_eq!(adr, uni_orig.5);

            assert_eq!(stack, vec![i]);
        }
    }
}

#[test]
fn fetch_address() {
    for i in 1..=0xFF {
        for addr in 0..0x11u16 {
            let addr = addr | ((addr + 1) << 8);

            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

            let mut stack = vec![addr as u8 + 1, addr as u8];
            memory[addr as usize] = i;

            assert_eq!(MicroOp::FetchAddress.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                       Ok(true));

            uni_orig.0[addr as usize] = i;

            assert_eq!(memory, uni_orig.0);
            assert_eq!(ports, uni_orig.1);
            assert_eq!(registers, uni_orig.2);
            assert_eq!(pc, uni_orig.3);
            assert_eq!(sp, uni_orig.4);
            assert_eq!(adr, uni_orig.5);

            assert_eq!(stack, vec![i]);
        }
    }
}

#[test]
fn write_address() {
    for i in 1..=0xFF {
        for addr in 0..0x11u16 {
            let addr = addr | ((addr + 1) << 8);

            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

            let mut stack = vec![i, addr as u8 + 1, addr as u8];

            assert_eq!(MicroOp::WriteAddress.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                       Ok(true));

            uni_orig.0[addr as usize] = i;

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

#[test]
fn read_register() {
    for i in 1..=0xFF {
        for aaa in 0..0b111 {
            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

            let mut stack = vec![];
            *registers[aaa as usize] = i;

            assert_eq!(MicroOp::ReadRegister(aaa).execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                       Ok(true));

            *uni_orig.2[aaa as usize] = i;

            assert_eq!(memory, uni_orig.0);
            assert_eq!(ports, uni_orig.1);
            assert_eq!(registers, uni_orig.2);
            assert_eq!(pc, uni_orig.3);
            assert_eq!(sp, uni_orig.4);
            assert_eq!(adr, uni_orig.5);

            assert_eq!(stack, vec![i]);
        }
    }
}

#[test]
fn write_register() {
    for i in 1..=0xFF {
        for aaa in 0..0b111 {
            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

            let mut stack = vec![i];

            assert_eq!(MicroOp::WriteRegister(aaa).execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                       Ok(true));

            *uni_orig.2[aaa as usize] = i;

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
