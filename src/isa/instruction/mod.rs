mod instruction;
mod display;

pub use self::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionStckRegisterPair, InstructionJumpCondition,
                            InstructionStckDirection, AluOperation, Instruction};
pub use self::display::DisplayInstruction;
