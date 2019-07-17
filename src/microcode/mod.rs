mod from_instruction;
mod execute;
mod display;
mod error;
mod op;

pub use self::error::MicrocodeExecutionError;
pub use self::from_instruction::MicroOpBlock;
pub use self::display::DisplayMicroOp;
pub use self::op::MicroOp;


/// μOps to execute to get to the next instruction.
pub static NEXT_INSTRUCTION: (MicroOpBlock, usize) = ([// forcebreak
                                                       MicroOp::LoadImmediate,
                                                       MicroOp::LoadInstruction,
                                                       MicroOp::Nop,
                                                       MicroOp::Nop,
                                                       MicroOp::Nop,
                                                       MicroOp::Nop],
                                                      2);
