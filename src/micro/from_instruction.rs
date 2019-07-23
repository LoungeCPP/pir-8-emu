use self::super::super::isa::instruction::{InstructionStckRegisterPair, InstructionPortDirection, InstructionStckDirection, AluOperation, Instruction};
use self::super::MicroOp;


const FLAG_REGISTER_ADDRESS: u8 = 0b000;
const S_REGISTER_ADDRESS: u8 = 0b001;
const X_REGISTER_ADDRESS: u8 = 0b010;
const Y_REGISTER_ADDRESS: u8 = 0b011;
const A_REGISTER_ADDRESS: u8 = 0b100;
const B_REGISTER_ADDRESS: u8 = 0b101;
const C_REGISTER_ADDRESS: u8 = 0b110;
const D_REGISTER_ADDRESS: u8 = 0b111;


/// `[MicroOp; N]` typedef, where `N` is *not* to be relied upon
///
/// Use this type for storing values returned by [`MicroOp::from_instruction()`](enum.MicroOp.html#fn.from_instruction) instead
/// of any fixed-size array, as the size of this is not part of the stable API and subject to change without notice.
pub type MicroOpBlock = [MicroOp; 6];

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
    /// # use pir_8_emu::micro::MicroOp;
    /// let ops = MicroOp::from_instruction(Instruction::Move { aaa: 0b100, bbb: 0b101 });
    /// let ops = &ops.0[..ops.1];
    ///
    /// assert_eq!(ops, &[MicroOp::ReadRegister(0b100), MicroOp::WriteRegister(0b101)]);
    /// ```
    pub fn from_instruction(instr: Instruction) -> (MicroOpBlock, usize) {
        match instr {
            Instruction::Reserved(_) |
            Instruction::Alu(AluOperation::Reserved(_)) => {
                ([// forcebreak
                  MicroOp::Nop,
                  MicroOp::Nop,
                  MicroOp::Nop,
                  MicroOp::Nop,
                  MicroOp::Nop,
                  MicroOp::Nop],
                 6)
            }

            Instruction::Jump(cond) => {
                ([// forcebreak
                  MicroOp::LoadImmediate,
                  MicroOp::LoadImmediate,
                  MicroOp::ReadRegister(FLAG_REGISTER_ADDRESS),
                  MicroOp::CheckJumpCondition(cond),
                  MicroOp::Jump,
                  MicroOp::Nop],
                 5)
            }

            Instruction::LoadImmediate { aaa } => {
                ([// forcebreak
                  MicroOp::LoadImmediate,
                  MicroOp::WriteRegister(aaa),
                  MicroOp::Nop,
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
                  MicroOp::Nop,
                  MicroOp::Nop],
                 4)
            }

            Instruction::Save { aaa } => {
                ([// forcebreak
                  MicroOp::ReadRegister(aaa),
                  MicroOp::LoadImmediate,
                  MicroOp::LoadImmediate,
                  MicroOp::WriteAddress,
                  MicroOp::Nop,
                  MicroOp::Nop],
                 4)
            }

            Instruction::Alu(op) => {
                ([// forcebreak
                  MicroOp::ReadRegister(X_REGISTER_ADDRESS),
                  MicroOp::ReadRegister(Y_REGISTER_ADDRESS),
                  MicroOp::ReadRegister(FLAG_REGISTER_ADDRESS),
                  MicroOp::Alu(op),
                  MicroOp::WriteRegister(FLAG_REGISTER_ADDRESS),
                  MicroOp::WriteRegister(S_REGISTER_ADDRESS)],
                 6)
            }

            Instruction::Move { aaa, bbb } => {
                ([// forcebreak
                  MicroOp::ReadRegister(aaa),
                  MicroOp::WriteRegister(bbb),
                  MicroOp::Nop,
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
                  MicroOp::Nop,
                  MicroOp::Nop],
                 3)
            }

            Instruction::Comp { aaa } => {
                ([// forcebreak
                  MicroOp::ReadRegister(S_REGISTER_ADDRESS),
                  MicroOp::ReadRegister(aaa),
                  MicroOp::ReadRegister(FLAG_REGISTER_ADDRESS),
                  MicroOp::Compare,
                  MicroOp::WriteRegister(FLAG_REGISTER_ADDRESS),
                  MicroOp::Nop],
                 5)
            }

            Instruction::Stck { d: InstructionStckDirection::Push, r } => {
                let [f, s] = stck_address_pair(r);

                ([// forcebreak
                  MicroOp::ReadRegister(s),
                  MicroOp::StackPush,
                  MicroOp::ReadRegister(f),
                  MicroOp::StackPush,
                  MicroOp::Nop,
                  MicroOp::Nop],
                 4)
            }
            Instruction::Stck { d: InstructionStckDirection::Pop, r } => {
                let [f, s] = stck_address_pair(r);

                ([// forcebreak
                  MicroOp::StackPop,
                  MicroOp::WriteRegister(f),
                  MicroOp::StackPop,
                  MicroOp::WriteRegister(s),
                  MicroOp::Nop,
                  MicroOp::Nop],
                 4)
            }

            Instruction::Clrf => {
                ([// forcebreak
                  MicroOp::MakeImmediate(0),
                  MicroOp::WriteRegister(FLAG_REGISTER_ADDRESS),
                  MicroOp::Nop,
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
