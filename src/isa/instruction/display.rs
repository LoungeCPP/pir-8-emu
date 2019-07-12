use self::super::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionStckRegisterPair, InstructionStckDirection, AluOperation,
                  Instruction};
use self::super::super::GeneralPurposeRegister;
use std::fmt;


/// Helper struct for assembly-printing instructions with `format!()` and `{}`.
///
/// An instruction might need the register set to be assebly-formatted. This struct provides that.
///
/// It is created by the [`display()`](enum.Instruction.html#method.display) method on [`Instruction`](enum.Instruction.html).
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayInstruction<'a> {
    pub(crate) instr: &'a Instruction,
    pub(crate) registers: &'a [GeneralPurposeRegister; 8],
}

impl<'a> fmt::Display for DisplayInstruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.instr {
            Instruction::Reserved(raw) => write!(f, "{:#06b}_{:04b}", (raw & 0b1111_0000) >> 4, raw & 0b0000_1111),
            Instruction::Jump { xxx } => write!(f, "JUMP {}", self.registers[*xxx as usize].letter()),
            Instruction::LoadImmediate { aaa } => write!(f, "LOAD IMM {}", self.registers[*aaa as usize].letter()),
            Instruction::LoadIndirect { aaa } => write!(f, "LOAD IND {}", self.registers[*aaa as usize].letter()),
            Instruction::Save { aaa } => write!(f, "SAVE {}", self.registers[*aaa as usize].letter()),
            Instruction::Alu(op) => write!(f, "ALU {}", op),
            Instruction::Move { aaa, bbb } => write!(f, "MOVE {} {}", self.registers[*aaa as usize].letter(), self.registers[*bbb as usize].letter()),
            Instruction::Comp { aaa } => write!(f, "COMP {}", self.registers[*aaa as usize].letter()),
            Instruction::Stck { d, r } => write!(f, "STCK {} {}", d, r),
            Instruction::Clrf => f.write_str("CLRF"),
            Instruction::Halt => f.write_str("HALT"),
        }
    }
}


impl fmt::Display for InstructionStckDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionStckDirection::Push => f.write_str("PUSH"),
            InstructionStckDirection::Pop => f.write_str("POP"),
        }
    }
}


impl fmt::Display for InstructionStckRegisterPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionStckRegisterPair::Ab => f.write_str("A&B"),
            InstructionStckRegisterPair::Cd => f.write_str("C&D"),
        }
    }
}


impl fmt::Display for AluOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AluOperation::Reserved(raw) => write!(f, "{:#06b}", raw),
            AluOperation::Add => f.write_str("ADD"),
            AluOperation::Sub => f.write_str("SUB"),
            AluOperation::Not => f.write_str("NOT"),
            AluOperation::Or => f.write_str("OR"),
            AluOperation::Xor => f.write_str("XOR"),
            AluOperation::And => f.write_str("AND"),
            AluOperation::ShiftOrRotate { d, tt } => write!(f, "SOR {} {}", d, tt),
        }
    }
}


impl fmt::Display for AluOperationShiftOrRotateDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AluOperationShiftOrRotateDirection::Left => f.write_str("LEFT"),
            AluOperationShiftOrRotateDirection::Right => f.write_str("RIGHT"),
        }
    }
}


impl fmt::Display for AluOperationShiftOrRotateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AluOperationShiftOrRotateType::Lsf => f.write_str("LSF"),
            AluOperationShiftOrRotateType::Asf => f.write_str("ASF"),
            AluOperationShiftOrRotateType::Rtc => f.write_str("RTC"),
            AluOperationShiftOrRotateType::Rtw => f.write_str("RTW"),
        }
    }
}
