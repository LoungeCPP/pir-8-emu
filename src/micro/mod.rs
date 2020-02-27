//! Microcode implementation
//!
//! The μOp is the smallest executable atom into sets of which instructions degenerate,
//! implementing an 8-bit stack-based language.
//!
//! # Examples
//!
//! Ignoring error handling, a simple emulator without I/O port support can be implemented as such
//! (as in the `emulate_dumb` example):
//!
//! ```
//! # use pir_8_emu::isa::instruction::{InstructionLoadImmediateWideRegisterPair, InstructionMadrDirection, InstructionRegisterPair, Instruction};
//! # use pir_8_emu::isa::{GeneralPurposeRegister, SpecialPurposeRegister};
//! # use pir_8_emu::micro::{MicroOp, NEXT_INSTRUCTION};
//! # use pir_8_emu::vm::{Memory, Ports};
//! let mut ports     = Ports::new();
//! let mut registers = GeneralPurposeRegister::defaults();
//! let mut pc        = SpecialPurposeRegister::new("Program Counter", "PC");
//! let mut sp        = SpecialPurposeRegister::new("Stack Pointer", "SP");
//! let mut adr       = SpecialPurposeRegister::new("Memory Address", "ADR");
//! let mut ins       = SpecialPurposeRegister::new("Instruction", "INS");
//!
//! let mut memory = Memory::from(&[Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Adr }.into(),
//!                                 0x01,
//!                                 0x10,
//!                                 Instruction::LoadIndirect { rrr: 0b100 }.into(),
//!                                 Instruction::LoadImmediateByte { rrr: 0b101 }.into(),
//!                                 0x69,
//!                                 Instruction::Move { qqq: 0b100, rrr: 0b110 }.into(),
//!                                 Instruction::Move { qqq: 0b101, rrr: 0b100 }.into(),
//!                                 Instruction::Move { qqq: 0b110, rrr: 0b101 }.into(),
//!                                 Instruction::Halt.into()][..]);
//! memory[0x0110] = 0xA1;
//!
//! let mut stack = vec![];
//! 'outside: loop {
//!     let ops = NEXT_INSTRUCTION;
//!     let ops = &ops.0[..ops.1];
//!     for op in ops {
//!         if !op.perform(&mut stack, &mut memory, &mut ports, &mut registers,
//!                        &mut pc, &mut sp, &mut adr, &mut ins).unwrap() {
//!             break 'outside;
//!         }
//!     }
//!
//!     let instr = Instruction::from(*ins);
//!     let ops = MicroOp::from_instruction(instr);
//!     let ops = &ops.0[..ops.1];
//!     for op in ops {
//!         if !op.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc,
//!                        &mut sp, &mut adr, &mut ins).unwrap() {
//!             break 'outside;
//!         }
//!     }
//! }
//!
//! assert_eq!(*registers[0b100], 0x69);
//! assert_eq!(*registers[0b101], 0xA1);
//! ```


mod from_instruction;
mod display;
mod perform;
mod error;
mod op;

pub use self::from_instruction::MicroOpBlock;
pub use self::error::MicroOpPerformError;
pub use self::display::DisplayMicroOp;
pub use self::op::MicroOp;


/// μOps to perform to get to the next instruction.
pub static NEXT_INSTRUCTION: (MicroOpBlock, usize) = ([// forcebreak
                                                       MicroOp::LoadImmediate,
                                                       MicroOp::LoadInstruction,
                                                       MicroOp::Nop,
                                                       MicroOp::Nop,
                                                       MicroOp::Nop,
                                                       MicroOp::Nop],
                                                      2);
