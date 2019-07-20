extern crate pir_8_emu;

use pir_8_emu::isa::{GeneralPurposeRegister, SpecialPurposeRegister};
use pir_8_emu::micro::{MicroOp, NEXT_INSTRUCTION};
use pir_8_emu::isa::instruction::Instruction;
use pir_8_emu::vm::{Memory, Ports};
use pir_8_emu::ReadWritable;
use std::{env, fs};


fn main() {
    let (mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = (Ports::new(),
                                                                        GeneralPurposeRegister::defaults(),
                                                                        SpecialPurposeRegister::new("Program Counter", "PC"),
                                                                        SpecialPurposeRegister::new("Stack Pointer", "SP"),
                                                                        SpecialPurposeRegister::new("Memory Address", "ADR"),
                                                                        SpecialPurposeRegister::new("Instruction", "INS"));

    let mut memory = Memory::from(&fs::read(env::args().skip(1).next().expect("File argument not passed")).expect("Passed file nonexistant")[..]);

    let mut stack = vec![];
    'ol: loop {
        let ops = NEXT_INSTRUCTION;
        let ops = &ops.0[..ops.1];
        for op in ops {
            // println!("{}", op.display(&registers));
            if !op.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins).unwrap() {
                break 'ol;
            }
        }
        println!();

        let instr = Instruction::from(*ins);
        println!("{}", instr.display(&registers));
        let ops = MicroOp::from_instruction(instr);
        let ops = &ops.0[..ops.1];
        for op in ops {
            // println!("{}", op.display(&registers));
            if !op.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins).unwrap() {
                break 'ol;
            }
        }

        println!();
        // println!("{:?}", ports);
        for reg in &mut registers {
            let r = reg.was_read();
            let w = reg.was_written();
            if r || w {
                println!("{}{} {}", if r { 'R' } else { ' ' }, if w { 'W' } else { ' ' }, reg);
                reg.reset_rw();
            }
        }

        let mut memory_accessed = false;
        for (addr, val, r, w) in memory.iter_rw() {
            if !memory_accessed {
                memory_accessed = true;

                println!("Memory:");
            }

            println!("  {:#06x} {}{} {:#04x}", addr, if r { 'R' } else { ' ' }, if w { 'W' } else { ' ' }, val);
        }
        if memory_accessed {
            memory.reset_rw();
        }

        let mut ports_accessed = false;
        for (addr, val, r, w) in ports.iter_rw() {
            if !ports_accessed {
                ports_accessed = true;

                println!("Ports:");
            }

            println!("  {:#06x} {}{} {:#04x}", addr, if r { 'R' } else { ' ' }, if w { 'W' } else { ' ' }, val);
        }
        if ports_accessed {
            ports.reset_rw();
        }
        // println!("{}", pc);
        // println!("{}", sp);
        // println!("{}", adr);
        // println!("{}", ins);
        println!();
    }

    println!();
    for reg in &registers {
        println!("{}", reg);
    }
}
