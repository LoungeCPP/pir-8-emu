use self::super::super::isa::instruction::{InstructionJumpCondition, AluOperation};


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
    /// Halt.
    Halt,

    /// Push a byte from the top of the μstack to the stack
    StackPush,
    /// Pop a byte from the stack
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
    /// Read a 1-byte immediate from memory @ PC to the top of the μstack, incrementing PC
    LoadImmediate,

    /// Read the value from memory at the address specified by the top two bytes of the μstack
    FetchAddress,
    /// Write to memory at the address specified by the top two bytes of the μstack the byte at the next value on the μstack
    WriteAddress,

    /// Check if the specified jump condition is satisfied by the top of the μstack
    CheckJumpCondition(InstructionJumpCondition),
    /// If the top of the μstack is `0`, pop two bytes off the top of the μstack and increment PC by 2.
    /// If it's `1`, pop an two bytes off the top of the μstack and load them into PC.
    /// Otherwise, error out.
    Jump,

    /// Read the specified register into the top of the μstack.
    ReadRegister(u8),
    /// Write the top of the μstack to the specified register.
    WriteRegister(u8),
}
