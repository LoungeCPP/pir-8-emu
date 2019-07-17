extern crate pir_8_emu;

use pir_8_emu::isa::{GeneralPurposeRegister, SpecialPurposeRegister};
use pir_8_emu::microcode::{MicroOp, NEXT_INSTRUCTION};
use pir_8_emu::isa::instruction::Instruction;
use pir_8_emu::{Memory, Ports};
use std::{env, io};
use std::fs::File;


fn main() {
    let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = (Memory::new(),
                                                                                    Ports::new(),
                                                                                    GeneralPurposeRegister::defaults(),
                                                                                    SpecialPurposeRegister::new("Program Counter", "PC"),
                                                                                    SpecialPurposeRegister::new("Stack Pointer", "SP"),
                                                                                    SpecialPurposeRegister::new("Memory Address", "ADR"),
                                                                                    SpecialPurposeRegister::new("Instruction", "INS"));

    io::copy(&mut File::open(env::args().skip(1).next().expect("File argument not passed")).expect("Passed file nonexistant"),
             &mut &mut memory[..]).expect("Reading failed");

    let mut stack = vec![];
    'ol: loop {
        let ops = NEXT_INSTRUCTION;
        let ops = &ops.0[..ops.1];
        for op in ops {
            println!("{}", op.display(&registers));
            if !op.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins).unwrap() {
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
            if !op.execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins).unwrap() {
                break 'ol;
            }
        }

        println!();
        // println!("{:?}", ports);
        for reg in &registers {
            println!("{:?}", reg);
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
