extern crate pir_8_emu;

use pir_8_emu::isa::{GeneralPurposeRegister, SpecialPurposeRegister};
use pir_8_emu::micro::{MicroOp, NEXT_INSTRUCTION};
use pir_8_emu::isa::instruction::Instruction;
use pir_8_emu::vm::{Memory, Ports};
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
            println!("{}", op.display(&registers));
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
            println!("{}", op.display(&registers));
            if !op.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins).unwrap() {
                break 'ol;
            }
        }

        println!();
        // println!("{:?}", ports);
        for reg in &registers {
            println!("{}", reg);
        }
        println!("{}", pc);
        println!("{}", sp);
        println!("{}", adr);
        println!("{}", ins);
        println!();
    }

    println!();
    println!("{:?}", memory);
}
