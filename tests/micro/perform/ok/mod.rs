use pir_8_emu::micro::MicroOp;
use pir_8_emu::ReadWritable;
use self::super::universe;

mod check_jump_condition;
mod jump;
mod alu;


#[test]
fn nop() {
    let uni_orig = universe();
    let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();
    let mut stack = vec![];

    assert_eq!(MicroOp::Nop.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
               Ok(true));

    assert_eq!((memory, ports, registers, pc, sp, adr, ins), uni_orig);

    assert_eq!(stack, vec![]);
}

#[test]
fn halt() {
    let uni_orig = universe();
    let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();
    let mut stack = vec![];

    assert_eq!(MicroOp::Halt.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
               Ok(false));

    assert_eq!((memory, ports, registers, pc, sp, adr, ins), uni_orig);

    assert_eq!(stack, vec![]);
}

#[test]
fn load_instruction() {
    for i in 1..=0xFF {
        let mut uni_orig = universe();
        let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

        let mut stack = vec![i];

        assert_eq!(MicroOp::LoadInstruction.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                   Ok(true));

        *uni_orig.6 = i;

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
fn stack_push() {
    for i in 1..=0xFF {
        for addr in 0..0x11u16 {
            let addr = addr | ((addr + 1) << 8);

            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

            let mut stack = vec![i];
            *sp = addr;

            assert_eq!(MicroOp::StackPush.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                       Ok(true));

            uni_orig.0[addr + 1] = i;
            let _read_sp = *uni_orig.4;
            *uni_orig.4 = addr + 1;
            let _read_adr = *uni_orig.5;
            *uni_orig.5 = addr + 1;

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
fn stack_pop() {
    for i in 1..=0xFF {
        for addr in 1..0x12u16 {
            let addr = addr | ((addr + 1) << 8);

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
            *uni_orig.4 = addr - 1;
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
}

#[test]
fn port_in() {
    for i in 1..=0xFF {
        for port in 0..=0xFF {
            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

            let mut stack = vec![port];
            ports.write(port, i);

            assert_eq!(MicroOp::PortIn.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                       Ok(true));

            let _read_port = uni_orig.1.read(port);
            uni_orig.1.write(port, i);

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
}

#[test]
fn port_out() {
    for i in 1..=0xFF {
        for port in 0..=0xFF {
            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

            let mut stack = vec![i, port];

            assert_eq!(MicroOp::PortOut.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                       Ok(true));

            uni_orig.1.write(port, i);

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
    for flags_start in 0..=0b11111 {
        for lhs in 0..=0xFFu8 {
            let rhs = lhs.wrapping_mul(3);

            let uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

            let mut stack = vec![lhs, rhs, flags_start];

            assert_eq!(MicroOp::Compare.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                       Ok(true));

            assert_eq!(memory, uni_orig.0);
            assert_eq!(ports, uni_orig.1);
            assert_eq!(registers, uni_orig.2);
            assert_eq!(pc, uni_orig.3);
            assert_eq!(sp, uni_orig.4);
            assert_eq!(adr, uni_orig.5);
            assert_eq!(ins, uni_orig.6);

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
        let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

        let mut stack = vec![];

        assert_eq!(MicroOp::MakeImmediate(i).perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                   Ok(true));

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
    for i in 1..=0xFF {
        for addr in 0..0x11u16 {
            let addr = addr | ((addr + 1) << 8);

            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

            let mut stack = vec![];
            *pc = addr;
            memory[addr] = i;

            assert_eq!(MicroOp::LoadImmediate.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                       Ok(true));

            let _read_mem = uni_orig.0[addr];
            uni_orig.0[addr] = i;
            let _read_pc = *uni_orig.3;
            *uni_orig.3 = addr + 1;
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
}

#[test]
fn fetch_address() {
    for i in 1..=0xFF {
        for addr in 0..0x11u16 {
            let addr = addr | ((addr + 1) << 8);

            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

            let mut stack = vec![addr as u8 + 1, addr as u8];
            memory[addr] = i;

            assert_eq!(MicroOp::FetchAddress.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                       Ok(true));

            let _read_mem = uni_orig.0[addr];
            uni_orig.0[addr] = i;
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
}

#[test]
fn write_address() {
    for i in 1..=0xFF {
        for addr in 0..0x11u16 {
            let addr = addr | ((addr + 1) << 8);

            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

            let mut stack = vec![i, addr as u8 + 1, addr as u8];

            assert_eq!(MicroOp::WriteAddress.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                       Ok(true));

            uni_orig.0[addr] = i;
            let _read_adr = *uni_orig.5;
            *uni_orig.5 = addr;

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
fn read_register() {
    for i in 1..=0xFF {
        for aaa in 0..=0b111 {
            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

            let mut stack = vec![];
            *registers[aaa as usize] = i;
            registers[aaa as usize].rw_reset();

            assert_eq!(MicroOp::ReadRegister(aaa).perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                       Ok(true));

            *uni_orig.2[aaa as usize] = i;
            uni_orig.2[aaa as usize].rw_reset();
            let _read_aaa = *uni_orig.2[aaa as usize];

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
}

#[test]
fn write_register() {
    for i in 1..=0xFF {
        for aaa in 0..=0b111 {
            let mut uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

            let mut stack = vec![i];

            assert_eq!(MicroOp::WriteRegister(aaa).perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                       Ok(true));

            *uni_orig.2[aaa as usize] = i;

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
