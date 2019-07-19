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
    pub(super) op: &'a MicroOp,
    pub(super) registers: &'a GeneralPurposeRegisterBank,
}

impl<'a> fmt::Display for DisplayMicroOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.op {
            MicroOp::Alu(op) => write!(f, "Alu {}", op),

            MicroOp::MakeImmediate(imm) => write!(f, "MakeImmediate {:#04x}", imm),

            MicroOp::CheckJumpCondition(cond) => write!(f, "CheckJumpCondition {}", cond),

            MicroOp::ReadRegister(aaa) => write!(f, "ReadRegister {}", self.registers[*aaa as usize].letter()),
            MicroOp::WriteRegister(aaa) => write!(f, "WriteRegister {}", self.registers[*aaa as usize].letter()),

            op => (op as &fmt::Debug).fmt(f),
        }
    }
}
