use self::super::{InstructionLoadImmediateWideRegisterPair, AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionJumpCondition,
                  InstructionPortDirection, InstructionMadrDirection, InstructionStckDirection, InstructionRegisterPair, AluOperation, Instruction};
use self::super::super::GeneralPurposeRegisterBank;
use std::fmt;


/// Helper struct for assembly-printing instructions with `format!()` and `{}`.
///
/// An instruction might need the register set to be assebly-formatted. This struct provides that.
///
/// It is created by the [`display()`](enum.Instruction.html#method.display) method on [`Instruction`](enum.Instruction.html).
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayInstruction<'a> {
    pub(super) instr: &'a Instruction,
    pub(super) registers: &'a GeneralPurposeRegisterBank,
}

impl<'a> fmt::Display for DisplayInstruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.instr {
            Instruction::Reserved(raw) => write!(f, "{:#06b}_{:04b}", (raw & 0b1111_0000) >> 4, raw & 0b0000_1111),
            Instruction::LoadImmediateByte { rrr } => write!(f, "LOAD IMM BYTE {}", self.registers[*rrr as usize].letter()),
            Instruction::LoadIndirect { rrr } => write!(f, "LOAD IND {}", self.registers[*rrr as usize].letter()),
            Instruction::LoadImmediateWide { rr } => write!(f, "LOAD IMM WIDE {}", rr),
            Instruction::Jump(cond) => cond.fmt(f),
            Instruction::Save { rrr } => write!(f, "SAVE {}", self.registers[*rrr as usize].letter()),
            Instruction::Alu(op) => write!(f, "ALU {}", op),
            Instruction::Move { qqq, rrr } => write!(f, "MOVE {} {}", self.registers[*qqq as usize].letter(), self.registers[*rrr as usize].letter()),
            Instruction::Madr { d, r } => write!(f, "MADR {} {}", d, r),
            Instruction::Port { d, rrr } => write!(f, "PORT {} {}", d, self.registers[*rrr as usize].letter()),
            Instruction::Comp { rrr } => write!(f, "COMP {}", self.registers[*rrr as usize].letter()),
            Instruction::Stck { d, r } => write!(f, "STCK {} {}", d, r),
            Instruction::Clrf => f.write_str("CLRF"),
            Instruction::Halt => f.write_str("HALT"),
        }
    }
}


impl fmt::Display for InstructionMadrDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionMadrDirection::Write => f.write_str("WRITE"),
            InstructionMadrDirection::Read => f.write_str("READ"),
        }
    }
}


impl fmt::Display for InstructionJumpCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionJumpCondition::Jmpz => f.write_str("JMPZ"),
            InstructionJumpCondition::Jmpp => f.write_str("JMPP"),
            InstructionJumpCondition::Jmpg => f.write_str("JMPG"),
            InstructionJumpCondition::Jmpc => f.write_str("JMPC"),
            InstructionJumpCondition::Jmzg => f.write_str("JMZG"),
            InstructionJumpCondition::Jmzl => f.write_str("JMZL"),
            InstructionJumpCondition::Jmpl => f.write_str("JMPL"),
            InstructionJumpCondition::Jump => f.write_str("JUMP"),
        }
    }
}


impl fmt::Display for InstructionPortDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionPortDirection::In => f.write_str("IN"),
            InstructionPortDirection::Out => f.write_str("OUT"),
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


impl fmt::Display for InstructionLoadImmediateWideRegisterPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionLoadImmediateWideRegisterPair::Ab => f.write_str("A&B"),
            InstructionLoadImmediateWideRegisterPair::Cd => f.write_str("C&D"),
            InstructionLoadImmediateWideRegisterPair::Xy => f.write_str("X&Y"),
            InstructionLoadImmediateWideRegisterPair::Adr => f.write_str("ADR"),
        }
    }
}


impl fmt::Display for InstructionRegisterPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionRegisterPair::Ab => f.write_str("A&B"),
            InstructionRegisterPair::Cd => f.write_str("C&D"),
        }
    }
}


impl fmt::Display for AluOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AluOperation::Add => f.write_str("ADD"),
            AluOperation::Sub => f.write_str("SUB"),
            AluOperation::AddC => f.write_str("ADDC"),
            AluOperation::SubC => f.write_str("SUBC"),
            AluOperation::Or => f.write_str("OR"),
            AluOperation::Xor => f.write_str("XOR"),
            AluOperation::And => f.write_str("AND"),
            AluOperation::Not => f.write_str("NOT"),
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
