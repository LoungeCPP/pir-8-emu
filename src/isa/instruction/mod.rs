mod instruction;
mod display;

pub use self::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionStckRegisterPair, InstructionJumpCondition,
                            InstructionPortDirection, InstructionStckDirection, AluOperation, Instruction};
pub use self::display::DisplayInstruction;
