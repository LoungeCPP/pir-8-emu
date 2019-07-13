mod instruction;
mod from_str;
mod display;

pub use self::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionStckRegisterPair, InstructionJumpCondition,
                            InstructionPortDirection, InstructionStckDirection, AluOperation, Instruction};
pub use self::from_str::ParseInstructionError;
pub use self::display::DisplayInstruction;
