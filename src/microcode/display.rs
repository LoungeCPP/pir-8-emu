use self::super::super::isa::GeneralPurposeRegisterBank;
use self::super::MicroOp;
use std::fmt;


/// Helper struct for human-printing μOps with `format!()` and `{}`.
///
/// A μOp might need the register set to be assebly-formatted. This struct provides that.
///
/// It is created by the [`display()`](enum.MicroOp.html#method.display) method on [`MicroOp`](enum.MicroOp.html).
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayMicroOp<'a> {
    pub(crate) op: &'a MicroOp,
    pub(crate) registers: &'a GeneralPurposeRegisterBank,
}

impl<'a> fmt::Display for DisplayMicroOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.op {
            MicroOp::Nop => f.write_str("Nop"),
            MicroOp::Halt => f.write_str("Halt"),

            MicroOp::StackPush => f.write_str("StackPush"),
            MicroOp::StackPop => f.write_str("StackPop"),

            MicroOp::Alu(op) => write!(f, "Alu {}", op),

            MicroOp::PortIn => f.write_str("PortIn"),
            MicroOp::PortOut => f.write_str("PortOut"),

            MicroOp::Compare => f.write_str("Compare"),

            MicroOp::MakeImmediate(imm) => write!(f, "MakeImmediate {:#04x}", imm),
            MicroOp::LoadImmediate => f.write_str("LoadImmediate"),

            MicroOp::FetchAddress => f.write_str("FetchAddress"),
            MicroOp::WriteAddress => f.write_str("WriteAddress"),

            MicroOp::CheckJumpCondition(cond) => write!(f, "CheckJumpCondition {}", cond),
            MicroOp::Jump => f.write_str("Jump"),

            MicroOp::ReadRegister(aaa) => write!(f, "ReadRegister {}", self.registers[*aaa as usize].letter()),
            MicroOp::WriteRegister(aaa) => write!(f, "WriteRegister {}", self.registers[*aaa as usize].letter()),
        }
    }
}
