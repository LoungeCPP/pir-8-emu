use self::super::super::isa::instruction::{InstructionJumpCondition, AluOperation};
use self::super::super::isa::GeneralPurposeRegisterBank;
use self::super::DisplayMicroOp;


/// Actual μOps executable by the CPU
///
/// The approach is stack-based (think ComputerCraft or FORTH):
/// bytes are individually pushed and popped onto the μstack (separate from the actual program stack),
/// and there is no other storage.
///
/// Each high-level instruction deconstructs losslessly into up to six μOps,
/// with the exception of reserved instructions, which are converted into 6 NOPs.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MicroOp {
    /// Do nothing
    ///
    /// Also to pad out the returned instruction
    Nop,
    /// Halt
    Halt,
    /// Load the top of the stack into INS
    LoadInstruction,

    /// Write the address specified by the top two bytes of the μstack into ADR
    AdrWrite,
    /// Read both bytes of ADR onto the μstack
    AdrRead,

    /// Push a byte from the top of the μstack to the stack
    StackPush,
    /// Pop a byte from the stack to the μstack
    StackPop,

    /// Perform an ALU operation
    Alu(AluOperation),

    /// Read a byte from the port specified at the top of the μstack
    PortIn,
    /// Write to the port specified at the top of the μstack a byte from the next byte on the μstack
    PortOut,

    /// Execute the compare instruction with S at the top and the specified register as the next byte on the μstack
    Compare,

    /// Create an immediate value at the top of the μstack
    MakeImmediate(u8),
    /// Read a 1-byte immediate from memory at PC to the top of the μstack, incrementing PC
    LoadImmediate,

    /// Read the value from memory at ADR
    FetchAddress,
    /// Write to memory at ADR the byte on top of the μstack
    WriteAddress,

    /// Check if the specified jump condition is satisfied by the top of the μstack
    CheckJumpCondition(InstructionJumpCondition),
    /// If the top of the μstack is `1`, set PC to ADR.
    /// If it's `0`, do nothing.
    /// Otherwise, error out.
    Jump,

    /// Read the specified register into the top of the μstack.
    ReadRegister(u8),
    /// Write the top of the μstack to the specified register.
    WriteRegister(u8),
}

impl MicroOp {
    /// Get proxy object implementing `Display` for printing μOps in human-readable format
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::instruction::{InstructionJumpCondition, AluOperation};
    /// # use pir_8_emu::isa::GeneralPurposeRegister;
    /// # use pir_8_emu::micro::MicroOp;
    /// # let registers = GeneralPurposeRegister::defaults();
    /// assert_eq!(MicroOp::WriteRegister(registers[1].address()).display(&registers).to_string(),
    ///            "WriteRegister S");
    ///
    /// assert_eq!(MicroOp::Alu(AluOperation::Or).display(&registers).to_string(),
    ///            "Alu OR");
    ///
    /// assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmpz).display(&registers).to_string(),
    ///            "CheckJumpCondition JMPZ");
    /// ```
    pub fn display<'r, 's: 'r>(&'s self, registers: &'r GeneralPurposeRegisterBank) -> DisplayMicroOp<'r> {
        DisplayMicroOp {
            op: self,
            registers: registers,
        }
    }
}
