//! An instruction is a single byte, and can include some following imediate values purely for data.


use self::super::super::util::limit_to_width;
use std::convert::{TryFrom, From};


/// Instructions will increase the PC by one, unless otherwise stated.
///
/// The PC is incremented as the instruction is loaded from RAM.
/// An instruction is a single byte, and can include some following imediate values purely for data.
///
/// The 'Bit Mask' shows a pattern which denotes an instruction or group of instructions, the letters donoting where any value
/// can be used and still be consdiered part of the same instruction.
/// The 'name' is for either a group or single instruction.
/// 'Count' is how many of the 256 possible instructions are used by that bit pattern; HALT for example is exactly one
/// instruction, whilst MOVE is effectively 64 possible combinations [this was added to help me keep track of how many
/// operations I've defined, it should add up to 256].
///
/// Bit Mask  | Name | Count | Description
/// ----------|------|-------|------------
/// 0000 XXXX |      |    16 | Reserved
/// 0001 0XXX | JUMP |     8 | Jump, see section below
/// 0001 1AAA | LOAD |     8 | Load the the next byte into register `AAA`
///                       |||| (PC will be incremented a second time)
/// 0010 0AAA | LOAD |     8 | Load value in address indicated by the next two bytes into register `AAA`
///                       |||| (PC will be incremented two more times)
/// 0010 1AAA | SAVE |     8 | Store value in register `AAA` in address indicated by the next two bytes
///                       |||| (PC will be incremented two more times)
/// 0011 XXXX | ALU  |    16 | ALU based operations, see section below
/// 01AA ABBB | MOVE |    64 | Move a value from register `AAA` to register `BBB`
/// 10XX XXXX |      |    64 | Reserved
/// 110X XXXX |      |    32 | Reserved
/// 1110 XXXX |      |    16 | Reserved
/// 1111 0AAA | COMP |     8 | Compare register S with register `AAA`, see section below
/// 1111 10XX | STCK |     4 | Stack manipulation, see section below
/// 1111 110X |      |     2 | Reserved
/// 1111 1110 | CLRF |     1 | Clear the 'F' register, by setting it to `0000 0000`
/// 1111 1111 | HALT |     1 | Stop the CPU from doing any more execution
///
/// ## COMP - Compare
///
/// The compare instruction will compare the S register with a selected register. It will set the Zero and Parity flag based on
/// the value of the S register; the Zero flag if all the bits are zero, Parity if the number of one bits is even. Compare will
/// set the Equal flag if the two registers have the same bit pattern. The Greater than flag is set if S is greater than the
/// second register. Note that when when doing a compare signed/unsigned is not taken into account, the two registers are
/// treated as if they contain two unsigned values.
///
/// NB: This might change to instead compare just the X and Y register.
///
/// ## Stack Manipulation
///
/// When dealing with the stack, a pair of registers will be moved to or from 'the stack' and the SP updated to reflect the
/// changed address.
///
/// The registers A and B are paired, as are the registers C and D. Effectively, the stack works on 16 bit
/// values, but due to the 8 bit data bus it requires two transfers, though this is handled via the hardware/microcode.
/// Although still two distinct bytes, the B and D registers should be considered the more significant byte whilst A and C
/// registers the lesser; the more significant byte will be stored at the lower address in the stack, the pair of registers are
/// big-endian.
///
/// The Stack manipulation operations are of pattern `1111 10DR`.
/// The D bit indicates the direction; 0 for PUSH and 1 for POP.
/// The R bit indicates the register pair; 0 for A & B and 1 for C & D.
///
/// When PUSHing B or D will go to the address of the SP, whilst A or C will go to address one less than the SP.
/// After PUSHing, the SP will have been decremented by two.
///
/// When POPing, the same respective pairs of memory locations will be read from the same pair of registers, and the SP
/// increased by two.
///
/// **NB:** I Think I might update this to allow pushing/poping the PC, this would make it very easy (hardware wise) to handle
/// calling and returning functions
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    /// Reserved instruction, contains the entire byte
    Reserved(u8),
    /// Jump, see section above
    Jump { xxx: u8, },
    /// Load the the next byte into register `AAA` (PC will be incremented a second time)
    LoadImmediate { aaa: u8, },
    /// Load value in address indicated by the next two bytes into register `AAA` (PC will be incremented two more times)
    LoadIndirect { aaa: u8, },
    /// Store value in register `AAA` in address indicated by the next two bytes (PC will be incremented two more times)
    Save { aaa: u8, },
    /// ALU based operations, see member doc
    Alu(AluOperation),
    /// Move a value from register `AAA` to register `BBB`
    Move { aaa: u8, bbb: u8, },
    /// Compare register S with register `AAA`, see section above
    Comp { aaa: u8, },
    /// Stack manipulation, see member doc
    Stck {
        d: InstructionStckDirection,
        r: InstructionStckRegisterPair,
    },
    /// Clear the 'F' register, by setting it to `0000 0000`
    Clrf,
    /// Stop the CPU from doing any more execution
    Halt,
}

impl Instruction {
    /// Check if this instruction doesn't use reserved space
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::instruction::{AluOperation, Instruction};
    /// assert!(Instruction::Clrf.is_valid());
    /// assert!(Instruction::Alu(AluOperation::Or).is_valid());
    ///
    /// assert!(!Instruction::Reserved(0).is_valid());
    /// assert!(!Instruction::Alu(AluOperation::Reserved(0b0011)).is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        match self {
            Instruction::Reserved(_) => false,
            Instruction::Alu(op) => op.is_valid(),
            _ => true,
        }
    }
}

impl From<u8> for Instruction {
    fn from(raw: u8) -> Instruction {
        match ((raw & 0b1000_0000) != 0,
               (raw & 0b0100_0000) != 0,
               (raw & 0b0010_0000) != 0,
               (raw & 0b0001_0000) != 0,
               (raw & 0b0000_1000) != 0,
               (raw & 0b0000_0100) != 0,
               (raw & 0b0000_0010) != 0,
               (raw & 0b0000_0001) != 0) {
            (false, false, false, false, _, _, _, _) => Instruction::Reserved(raw),
            (false, false, false, true, false, _, _, _) => Instruction::Jump { xxx: raw & 0b0000_0111 },
            (false, false, false, true, true, _, _, _) => Instruction::LoadImmediate { aaa: raw & 0b0000_0111 },
            (false, false, true, false, false, _, _, _) => Instruction::LoadIndirect { aaa: raw & 0b0000_0111 },
            (false, false, true, false, true, _, _, _) => Instruction::Save { aaa: raw & 0b0000_0111 },
            (false, false, true, true, _, _, _, _) => {
                Instruction::Alu(AluOperation::try_from(raw & 0b0000_1111).expect("Wrong raw instruction slicing for ALU op parse"))
            }
            (false, true, _, _, _, _, _, _) => {
                Instruction::Move {
                    aaa: (raw & 0b0011_1000) >> 3,
                    bbb: raw & 0b0000_0111,
                }
            }
            (true, false, _, _, _, _, _, _) => Instruction::Reserved(raw),
            (true, true, false, _, _, _, _, _) => Instruction::Reserved(raw),
            (true, true, true, false, _, _, _, _) => Instruction::Reserved(raw),
            (true, true, true, true, false, _, _, _) => Instruction::Comp { aaa: raw & 0b0000_0111 },
            (true, true, true, true, true, false, d, r) => {
                Instruction::Stck {
                    d: d.into(),
                    r: r.into(),
                }
            }
            (true, true, true, true, true, true, false, _) => Instruction::Reserved(raw),
            (true, true, true, true, true, true, true, false) => Instruction::Clrf,
            (true, true, true, true, true, true, true, true) => Instruction::Halt,
        }
    }
}


/// The D bit indicates the direction; 0 for PUSH and 1 for POP.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionStckDirection {
    Push = 0,
    Pop = 1,
}

impl From<bool> for InstructionStckDirection {
    fn from(raw: bool) -> InstructionStckDirection {
        match raw {
            false => InstructionStckDirection::Push,
            true => InstructionStckDirection::Pop,
        }
    }
}


/// The R bit indicates the register pair; 0 for A & B and 1 for C & D.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionStckRegisterPair {
    Ab = 0,
    Cd = 1,
}

impl From<bool> for InstructionStckRegisterPair {
    fn from(raw: bool) -> InstructionStckRegisterPair {
        match raw {
            false => InstructionStckRegisterPair::Ab,
            true => InstructionStckRegisterPair::Cd,
        }
    }
}


/// Any CPU instruction of the pattern `0011 FFFF` will invoke some function of the ALU.
///
/// The four bits `FFFF` are the actual operation being performed by the ALU. The registers X and Y are used as inputs to the
/// ALU (only X for unary operations), and the S register is used to store the result.
///
/// ALU operations will also update the F register as noted. All will set the ZERO flag if the output (register S) is `0000
/// 0000`. All will set the Parity flag if the number of high bits are even.
///
/// FFFF | Name | Count | Description
/// -----|------|-------|------------
/// 0000 | ADD  |   1   | Addition of register X and register Y
/// 0001 | SUB  |   1   | Subtraction of register Y from register X (X-Y)
/// 0010 | NOT  |   1   | Bitwise NOT
/// 0011 |      |   1   | Reserved
/// 0100 |  OR  |   1   | Bitwise OR
/// 0101 | XOR  |   1   | Bitwise XOR
/// 0110 | AND  |   1   | Bitwise AND
/// 0111 |      |   1   | Reserved
/// 1DTT |      |   8   | Shift or Rotate, see section below
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AluOperation {
    /// Reserved operation, contains the entire nibble
    Reserved(u8),
    /// Addition of register X and register Y
    Add,
    /// Subtraction of register Y from register X (X-Y)
    Sub,
    /// Bitwise NOT
    Not,
    /// Bitwise OR
    Or,
    /// Bitwise XOR
    Xor,
    /// Bitwise AND
    And,
    /// Shift or Rotate, see member doc
    ShiftOrRotate {
        d: AluOperationShiftOrRotateDirection,
        tt: AluOperationShiftOrRotateType,
    },
}

impl AluOperation {
    /// Check if this operation doesn't use reserved space
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::instruction::AluOperation;
    /// assert!(AluOperation::Or.is_valid());
    ///
    /// assert!(!AluOperation::Reserved(0b0011).is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        match self {
            AluOperation::Reserved(_) => false,
            _ => true,
        }
    }
}

impl TryFrom<u8> for AluOperation {
    type Error = ();

    fn try_from(raw: u8) -> Result<AluOperation, ()> {
        let nib = limit_to_width(raw, 4).ok_or(())?;
        Ok(match ((nib & 0b1000) != 0, (nib & 0b0100) != 0, (nib & 0b0010) != 0, (nib & 0b0001) != 0) {
            (false, false, false, false) => AluOperation::Add,
            (false, false, false, true) => AluOperation::Sub,
            (false, false, true, false) => AluOperation::Not,
            (false, false, true, true) => AluOperation::Reserved(nib),
            (false, true, false, false) => AluOperation::Or,
            (false, true, false, true) => AluOperation::Xor,
            (false, true, true, false) => AluOperation::And,
            (false, true, true, true) => AluOperation::Reserved(nib),
            (true, d, _, _) => {
                AluOperation::ShiftOrRotate {
                    d: d.into(),
                    tt: AluOperationShiftOrRotateType::try_from(nib & 0b0011).expect("Wrong raw instruction slicing for ALU Shift or Rotate Type parse"),
                }
            }
        })
    }
}


/// All shifts can be performed left or right, as designated by the D bit of the instruction.
///
/// If D is a `1`, the shift is to the left, all bits will move to a higher value, if D is `0`, it's a right shift, moving bits
/// to lower values. There are then four types of shift that can be performed designated by the final two bits of the ALU
/// instruction. The name should be appended with an L or R for the direction of the shift, left or right respectively. For all
/// shift operations, the bit shifted out is set into the Carry flag.
///
/// TT | Name | Description
/// ---|------|------------
/// 00 | LSF  | Logical shift - a zero is inserted
/// 01 | ASF  | Arithmetic shift - a zero is inserted for left shift, bit-8 is inserted for right shift
/// 10 | RTC  | Rotate with carry - the Carry flag is inserted (Carry flag value before it is updated is used)
/// 11 | RTW  | Rotate without carry - the bit shifted out is is inserted
///
/// **NB:** An 'Arithmetic shift left' is the same as performing a 'Logcal shift left', they _can_ be used interchagably, but
/// 'Arithmtic shift left' should be avoided.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AluOperationShiftOrRotateType {
    /// Logical shift - a zero is inserted
    Lsf,
    /// Arithmetic shift - a zero is inserted for left shift, bit-8 is inserted for right shift
    Asf,
    /// Rotate with carry - the Carry flag is inserted (Carry flag value before it is updated is used)
    Rtc,
    /// Rotate without carry - the bit shifted out is is inserted
    Rtw,
}

impl TryFrom<u8> for AluOperationShiftOrRotateType {
    type Error = ();

    fn try_from(raw: u8) -> Result<AluOperationShiftOrRotateType, ()> {
        let nib = limit_to_width(raw, 2).ok_or(())?;
        Ok(match ((nib & 0b10) != 0, (nib & 0b01) != 0) {
            (false, false) => AluOperationShiftOrRotateType::Lsf,
            (false, true) => AluOperationShiftOrRotateType::Asf,
            (true, false) => AluOperationShiftOrRotateType::Rtc,
            (true, true) => AluOperationShiftOrRotateType::Rtw,
        })
    }
}


/// If D is a `1`, the shift is to the left, all bits will move to a higher value, if D is `0`, it's a right shift, moving bits
/// to lower values.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AluOperationShiftOrRotateDirection {
    Left = 1,
    Right = 0,
}

impl From<bool> for AluOperationShiftOrRotateDirection {
    fn from(raw: bool) -> AluOperationShiftOrRotateDirection {
        match raw {
            true => AluOperationShiftOrRotateDirection::Left,
            false => AluOperationShiftOrRotateDirection::Right,
        }
    }
}
