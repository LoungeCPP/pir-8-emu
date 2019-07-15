mod from_instruction;
mod execute;
mod error;
mod op;

pub use self::error::MicrocodeExecutionError;
pub use self::from_instruction::MicroOpBlock;
pub use self::op::MicroOp;
