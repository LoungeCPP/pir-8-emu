mod instruction;
mod display;

pub use self::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionStckRegisterPair, InstructionStckDirection,
                            AluOperation, Instruction};
pub use self::display::DisplayInstruction;
