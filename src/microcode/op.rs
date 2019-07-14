use self::super::super::isa::instruction::{InstructionStckRegisterPair, InstructionJumpCondition, InstructionPortDirection, InstructionStckDirection,
                                           AluOperation, Instruction};


const FLAG_REGISTER_ADDRESS: u8 = 0b000;
const S_REGISTER_ADDRESS: u8 = 0b001;
const X_REGISTER_ADDRESS: u8 = 0b010;
const Y_REGISTER_ADDRESS: u8 = 0b011;
const A_REGISTER_ADDRESS: u8 = 0b100;
const B_REGISTER_ADDRESS: u8 = 0b101;
const C_REGISTER_ADDRESS: u8 = 0b110;
const D_REGISTER_ADDRESS: u8 = 0b111;


/// Actual μOps executable by the CPU
///
/// The approach is stack-based (think ComputerCraft or FORTH):
/// bytes are individually pushed and popped onto the μstack (separate from the actual program stack),
/// and there is no other storage.
///
/// Each high-level instruction deconstructs losslessly into up to five μOps,
/// with the exception of reserved instructions, which are converted into 5 NOPs.
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
    /// If the top of the μstack is 0, increment PC twice and pop two bytes off the top of the μstack.
    /// Otherwise, pop an two bytes off the top of the μstack and load them into PC.
    Jump,

    /// Read the specified register into the top of the μstack.
    ReadRegister(u8),
    /// Write the top of the μstack to the specified register.
    WriteRegister(u8),
}

impl MicroOp {
    /// Get μOps corresponding to the given instruction
    ///
    /// The return type is `(ops, len)`, where `&ops[..len]` are the actual μOps and the rest is padding.
    /// This was done to reduce allocations.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::instruction::Instruction;
    /// # use pir_8_emu::microcode::MicroOp;
    /// let ops = MicroOp::from_instruction(Instruction::Move { aaa: 0b100, bbb: 0b101 });
    /// let ops = &ops.0[..ops.1];
    ///
    /// assert_eq!(ops, &[MicroOp::ReadRegister(0b100), MicroOp::WriteRegister(0b101)]);
    /// ```
    pub fn from_instruction(instr: Instruction) -> ([MicroOp; 5], usize) {
        match instr {
            Instruction::Reserved(_) |
            Instruction::Alu(AluOperation::Reserved(_)) => {
                ([// forcebreak
                  MicroOp::Nop,
                  MicroOp::Nop,
                  MicroOp::Nop,
                  MicroOp::Nop,
                  MicroOp::Nop],
                 5)
            }

            Instruction::Jump(cond) => {
                ([// forcebreak
                  MicroOp::LoadImmediate,
                  MicroOp::LoadImmediate,
                  MicroOp::ReadRegister(FLAG_REGISTER_ADDRESS),
                  MicroOp::CheckJumpCondition(cond),
                  MicroOp::Jump],
                 5)
            }

            Instruction::LoadImmediate { aaa } => {
                ([// forcebreak
                  MicroOp::LoadImmediate,
                  MicroOp::WriteRegister(aaa),
                  MicroOp::Nop,
                  MicroOp::Nop,
                  MicroOp::Nop],
                 2)
            }

            Instruction::LoadIndirect { aaa } => {
                ([// forcebreak
                  MicroOp::LoadImmediate,
                  MicroOp::LoadImmediate,
                  MicroOp::FetchAddress,
                  MicroOp::WriteRegister(aaa),
                  MicroOp::Nop],
                 4)
            }

            Instruction::Save { aaa } => {
                ([// forcebreak
                  MicroOp::ReadRegister(aaa),
                  MicroOp::LoadImmediate,
                  MicroOp::LoadImmediate,
                  MicroOp::WriteAddress,
                  MicroOp::Nop],
                 4)
            }

            Instruction::Alu(op) => {
                ([// forcebreak
                  MicroOp::ReadRegister(X_REGISTER_ADDRESS),
                  MicroOp::ReadRegister(Y_REGISTER_ADDRESS),
                  MicroOp::Alu(op),
                  MicroOp::WriteRegister(S_REGISTER_ADDRESS),
                  MicroOp::Nop],
                 4)
            }

            Instruction::Move { aaa, bbb } => {
                ([// forcebreak
                  MicroOp::ReadRegister(aaa),
                  MicroOp::WriteRegister(bbb),
                  MicroOp::Nop,
                  MicroOp::Nop,
                  MicroOp::Nop],
                 2)
            }

            Instruction::Port { d: InstructionPortDirection::In, aaa } => {
                ([// forcebreak
                  MicroOp::ReadRegister(A_REGISTER_ADDRESS),
                  MicroOp::PortIn,
                  MicroOp::WriteRegister(aaa),
                  MicroOp::Nop,
                  MicroOp::Nop],
                 3)
            }
            Instruction::Port { d: InstructionPortDirection::Out, aaa } => {
                ([// forcebreak
                  MicroOp::ReadRegister(aaa),
                  MicroOp::ReadRegister(A_REGISTER_ADDRESS),
                  MicroOp::PortOut,
                  MicroOp::Nop,
                  MicroOp::Nop],
                 3)
            }

            Instruction::Comp { aaa } => {
                ([// forcebreak
                  MicroOp::ReadRegister(S_REGISTER_ADDRESS),
                  MicroOp::ReadRegister(aaa),
                  MicroOp::Compare,
                  MicroOp::WriteRegister(FLAG_REGISTER_ADDRESS),
                  MicroOp::Nop],
                 4)
            }

            Instruction::Stck { d: InstructionStckDirection::Push, r } => {
                let [f, s] = stck_address_pair(r);

                ([// forcebreak
                  MicroOp::ReadRegister(f),
                  MicroOp::StackPush,
                  MicroOp::ReadRegister(s),
                  MicroOp::StackPush,
                  MicroOp::Nop],
                 4)
            }
            Instruction::Stck { d: InstructionStckDirection::Pop, r } => {
                let [f, s] = stck_address_pair(r);

                ([// forcebreak
                  MicroOp::StackPop,
                  MicroOp::WriteRegister(s),
                  MicroOp::StackPop,
                  MicroOp::WriteRegister(f),
                  MicroOp::Nop],
                 4)
            }

            Instruction::Clrf => {
                ([// forcebreak
                  MicroOp::MakeImmediate(0),
                  MicroOp::WriteRegister(FLAG_REGISTER_ADDRESS),
                  MicroOp::Nop,
                  MicroOp::Nop,
                  MicroOp::Nop],
                 2)
            }

            Instruction::Halt => {
                ([// forcebreak
                  MicroOp::Halt,
                  MicroOp::Nop,
                  MicroOp::Nop,
                  MicroOp::Nop,
                  MicroOp::Nop],
                 1)
            }
        }
    }
}

fn stck_address_pair(r: InstructionStckRegisterPair) -> [u8; 2] {
    match r {
        InstructionStckRegisterPair::Ab => [A_REGISTER_ADDRESS, B_REGISTER_ADDRESS],
        InstructionStckRegisterPair::Cd => [C_REGISTER_ADDRESS, D_REGISTER_ADDRESS],
    }
}
