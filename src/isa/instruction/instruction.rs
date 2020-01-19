use self::super::{ParseInstructionError, DisplayInstruction};
use self::super::super::super::util::limit_to_width;
use self::super::super::GeneralPurposeRegisterBank;
use std::convert::{TryFrom, From};


/// Instructions will increase the PC by one, unless otherwise stated.
///
/// The PC is incremented as the instruction is loaded from RAM.
///
/// An instruction is a single byte, and can include some following immediate values purely for data.
///
///
/// It is possible that the PC will overflow and wrap around as you load an instruction,
/// there is no hardware level protection or detection if this happens.
///
/// An example of how this can happen is if you perform a jump to `0xFFFF`,
/// as the instruction at `0xFFFF` is loaded, the PC would be incremented to `0x10000`,
/// but as it's only 16 bits wide, it becomes just `0x0000`.
///
///
/// The 'Bit Mask' shows a pattern which denotes an instruction or group of instructions, the letters denoting where any value
/// can be used and still be considered part of the same instruction.
///
/// The 'name' is for either a group or single instruction.
///
/// 'Count' is how many of the 256 possible instructions are used by that bit pattern; HALT for example is exactly one
/// instruction, whilst MOVE is effectively 64 possible combinations [this was added to help me keep track of how many
/// operations I've defined, it should add up to 256].
///
/// Bit Mask  | Name | Count | Description
/// ----------|------|-------|------------
/// 0000 0XXX |      |     8 | Reserved
/// 0000 10XX |      |     4 | Reserved
/// 0000 11XX | MADR |     4 | Move a value to/from the ADR register, see section below
/// 0001 0XXX | JUMP |     8 | Jump, see section below
/// 0001 1AAA | LOAD |     8 | Load the the next byte into register `AAA`
///                       |||| (PC will be incremented a second time)
/// 0010 0AAA | LOAD |     8 | Load value in address indicated by `ADR` into register `AAA`
/// 0010 1AAA | SAVE |     8 | Store value in register `AAA` in address indicated by `ADR`
/// 0011 XXXX | ALU  |    16 | ALU based operations, see section below
/// 01AA ABBB | MOVE |    64 | Move a value from register `AAA` to register `BBB`
/// 10XX XXXX |      |    64 | Reserved
/// 110X XXXX |      |    32 | Reserved
/// 1110 XXXX | PORT |    16 | Perform I/O, see section below
/// 1111 0AAA | COMP |     8 | Compare register S with register `AAA`, see section below
/// 1111 10XX | STCK |     4 | Stack manipulation, see section below
/// 1111 110X |      |     2 | Reserved
/// 1111 1110 | CLRF |     1 | Clear the 'F' register, by setting it to `0000 0000`
/// 1111 1111 | HALT |     1 | Stop the CPU from doing any more execution
///
/// ## PORT - I/O
///
/// The PORT instruction in the form `1110 DAAA` will perform I/O on the port specified in register A.
///
/// The `D` bit specifies the direction; `1` for reading in from the port (`PORT IN`) and `0` for writing out to the port
/// (`PORT OUT`).
/// The `AAA` bits specify the register to write to (D=1) or read from (D=0).
///
/// ## COMP - Compare
///
/// The compare instruction will compare the S register with a selected register.
///
/// It will set the Zero and Parity flag based on the value of the S register; the Zero flag if all the bits are zero,
/// Parity if the number of set bits is even.
///
/// Compare will set the Equal flag if the two registers have the same bit pattern.
///
/// The Greater than flag is set if S is greater than the second register.
///
/// Note that when doing a compare signed/unsigned is not taken into account,
/// the two registers are treated as if they contain two unsigned values.
///
/// **NB:** This might change to instead compare just the X and Y register.
///
/// ## Stack Manipulation
///
/// When dealing with the stack, a pair of registers will be moved to or from 'the stack' and the SP updated to reflect the
/// changed address.
///
/// The registers A and B are paired, as are the registers C and D.
///
/// Effectively, the stack works on 16-bit values, but due to the 8-bit data bus it requires two transfers,
/// though this is handled via the hardware/microcode.
///
/// Although still two distinct bytes, the B and D registers should be considered the more significant byte whilst A and C
/// registers the lesser; the more significant byte will be stored at the lower address in the stack,
/// the pair of registers are big-endian.
///
///
/// The Stack manipulation operations are of pattern `1111 10DR`.
///
/// The D bit indicates the direction; 0 for PUSH and 1 for POP.
///
/// The R bit indicates the register pair; 0 for A & B and 1 for C & D.
///
///
/// When PUSHing B/D will go to the address one less than the current SP, whilst A/C will go to address two less than the SP.
///
/// After PUSHing, the SP will have been decremented by two, with the SP containing the address of A/C (now in memory).
///
/// When POPing, the same respective pairs of memory locations will be read to the same pair of registers, and the SP increased
/// by two.
///
/// Care must be taken, especially when POPing the stack, as there is no under/overflow protection or detection,
/// just like with the PC incrementing during instruction execution.
///
/// In fact, by design, POPing the final value from the stack will result in an overflow bringing the SP back to `0x0000`.
///
/// In terms of pseudo-code, a PUSH followed by a POP can view as the following microcode, where SP is a pointer to the memory
/// address:
///
/// ```cpp
/// // PUSH
/// SP -= 1
/// *SP = B
/// SP -= 1
/// *SP = A
///
/// // POP
/// A = *SP
/// SP += 1
/// B = *SP
/// SP += 1
/// ```
///
/// **NB:** I Think I might update this to allow pushing/popping the PC, this would make it very easy (hardware wise) to handle
/// calling and returning functions
///
/// ## MADR - ADR Register Manipulation
///
/// The ADR register is a 16-bit register, it's value can be set/read to the general purpose registers.
///
/// The registers A and B are paired, as are the registers C and D.
///
/// Although still two distinct bytes, the B and D registers should be considered the more significant byte whilst A and C
/// registers the lesser;
/// the more significant byte will be stored at the lower address in the stack, the pair of registers are big-endian.
///
///
/// These ADR manipulation operations are of pattern `0000 10DR`.
///
/// The D bit indicates the direction; 0 for write-to and 1 for read-from the ADR register.
///
/// The R bit indicates the register pair; 0 for A & B and 1 for C & D.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    /// Reserved instruction, contains the entire byte
    Reserved(u8),
    /// Manipulate ADR, see section above
    Madr {
        d: InstructionMadrDirection,
        r: InstructionRegisterPair,
    },
    /// Jump, see member doc
    Jump(InstructionJumpCondition),
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
    /// Perform I/O, see section above
    Port {
        d: InstructionPortDirection,
        aaa: u8,
    },
    /// Compare register S with register `AAA`, see section above
    Comp { aaa: u8, },
    /// Stack manipulation, see member doc
    Stck {
        d: InstructionStckDirection,
        r: InstructionRegisterPair,
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
    pub fn is_valid(self) -> bool {
        match self {
            Instruction::Reserved(_) => false,
            Instruction::Alu(op) => op.is_valid(),
            _ => true,
        }
    }

    /// Get the amount of data bytes following this instruction
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::instruction::{AluOperation, Instruction};
    /// assert_eq!(Instruction::Clrf.data_length(), 0);
    /// assert_eq!(Instruction::Alu(AluOperation::Or).data_length(), 0);
    ///
    /// assert_eq!(Instruction::LoadImmediate{ aaa: 0 }.data_length(), 1);
    /// ```
    pub fn data_length(self) -> usize {
        match self {
            Instruction::LoadImmediate { .. } => 1,
            _ => 0,
        }
    }

    /// Get proxy object implementing `Display` for printing instructions in assembly format
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::instruction::{InstructionStckDirection, InstructionRegisterPair, AluOperation, Instruction};
    /// # use pir_8_emu::isa::GeneralPurposeRegister;
    /// # let registers = GeneralPurposeRegister::defaults();
    /// assert_eq!(Instruction::Clrf.display(&registers).to_string(),
    ///            "CLRF");
    /// assert_eq!(Instruction::Alu(AluOperation::Or).display(&registers).to_string(),
    ///            "ALU OR");
    /// assert_eq!(Instruction::Stck {
    ///                d: InstructionStckDirection::Push,
    ///                r: InstructionRegisterPair::Cd,
    ///            }.display(&registers).to_string(),
    ///            "STCK PUSH C&D");
    ///
    /// assert_eq!(Instruction::Reserved(0b1111_0000).display(&registers).to_string(),
    ///            "0b1111_0000");
    /// ```
    pub fn display<'r, 's: 'r>(&'s self, registers: &'r GeneralPurposeRegisterBank) -> DisplayInstruction<'r> {
        DisplayInstruction {
            instr: self,
            registers: registers,
        }
    }

    /// Parse assembly instruction format
    ///
    /// The input string must be ASCII and contain no vertical whitespace
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType,
    /// #                                   InstructionJumpCondition, AluOperation, Instruction};
    /// # use pir_8_emu::isa::GeneralPurposeRegister;
    /// # let registers = GeneralPurposeRegister::defaults();
    /// assert_eq!(Instruction::from_str("JMPL", &registers),
    ///            Ok(Instruction::Jump(InstructionJumpCondition::Jmpl)));
    ///
    /// assert_eq!(Instruction::from_str("LOAD IND B", &registers),
    ///            Ok(Instruction::LoadIndirect { aaa: 0b101 }));
    ///
    /// assert_eq!(Instruction::from_str("ALU SOR RIGHT ASF", &registers),
    ///            Ok(Instruction::Alu(AluOperation::ShiftOrRotate {
    ///                d: AluOperationShiftOrRotateDirection::Right,
    ///                tt: AluOperationShiftOrRotateType::Asf,
    ///            })));
    /// ```
    #[inline]
    pub fn from_str(s: &str, registers: &GeneralPurposeRegisterBank) -> Result<Instruction, ParseInstructionError> {
        Instruction::from_str_impl(s, registers)
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
            (false, false, false, false, false, _, _, _) => Instruction::Reserved(raw),
            (false, false, false, false, true, false, _, _) => Instruction::Reserved(raw),
            (false, false, false, false, true, true, d, r) => {
                Instruction::Madr {
                    d: d.into(),
                    r: r.into(),
                }
            }
            (false, false, false, true, false, _, _, _) => {
                Instruction::Jump(InstructionJumpCondition::try_from(raw & 0b0000_1111).expect("Wrong raw instruction slicing for JUMP condition parse"))
            }
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
            (true, true, true, false, d, _, _, _) => {
                Instruction::Port {
                    d: d.into(),
                    aaa: raw & 0b0000_0111,
                }
            }
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

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        match self {
            Instruction::Reserved(raw) => raw,
            Instruction::Madr { d, r } => 0b0000_1100 | (d as u8) | (r as u8),
            Instruction::Jump(cond) => 0b0001_0000 | (cond as u8),
            Instruction::LoadImmediate { aaa } => 0b0001_1000 | aaa,
            Instruction::LoadIndirect { aaa } => 0b0010_0000 | aaa,
            Instruction::Save { aaa } => 0b0010_1000 | aaa,
            Instruction::Alu(op) => {
                let op_b: u8 = op.into();
                0b0011_0000u8 | op_b
            }
            Instruction::Move { aaa, bbb } => 0b0100_0000 | (aaa << 3) | bbb,
            Instruction::Port { d, aaa } => 0b1110_0000 | (d as u8) | aaa,
            Instruction::Comp { aaa } => 0b1111_0000 | aaa,
            Instruction::Stck { d, r } => 0b1111_1000 | (d as u8) | (r as u8),
            Instruction::Clrf => 0b1111_1110,
            Instruction::Halt => 0b1111_1111,
        }
    }
}


/// The `D` bit indicates the direction – `0` for write-to (`MADR WRITE`) and `1` for read-from (`MADR READ`) the `ADR`
/// register.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionMadrDirection {
    Write = 0b00,
    Read = 0b10,
}

impl From<bool> for InstructionMadrDirection {
    fn from(raw: bool) -> InstructionMadrDirection {
        match raw {
            false => InstructionMadrDirection::Write,
            true => InstructionMadrDirection::Read,
        }
    }
}


/// This Instruction takes a three bit operand indicating under what condition the jump should be performed.
///
/// If the condition is met, the value of ADR is loaded into the PC.
///
/// If the condition is not met, no further special action is taken; the PC would have already been incremented as part of
/// loading the instruction.
///
/// **NB:** The value of ADR must have been set with the desired target location prior to the JUMP instruction being performed.
///
///
/// This table shows what combination of bits to the JUMP instruction check what flags and in what combination
///
///
/// FFF | Name | Description
/// ----|------|-------------
/// 000 | JMPZ | Zero flag
/// 001 | JMPP | Parity flag
/// 010 | JMPG | NOT Zero AND Greater than flag (i.e. greater than)
/// 011 | JMPC | Carry flag
/// 100 | JMZG | Zero OR Greater than flags
/// 101 | JMZL | Zero OR NOT Greater than flag
/// 110 | JMPL | NOT Zero AND NOT Greater than flag (i.e. less than)
/// 111 | JUMP | Unconditional Jump (always jumps)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionJumpCondition {
    Jmpz = 0b000,
    Jmpp = 0b001,
    Jmpg = 0b010,
    Jmpc = 0b011,
    Jmzg = 0b100,
    Jmzl = 0b101,
    Jmpl = 0b110,
    Jump = 0b111,
}

impl TryFrom<u8> for InstructionJumpCondition {
    type Error = ();

    fn try_from(raw: u8) -> Result<InstructionJumpCondition, ()> {
        let nib = limit_to_width(raw, 3).ok_or(())?;
        Ok(match ((nib & 0b0100) != 0, (nib & 0b0010) != 0, (nib & 0b0001) != 0) {
            (false, false, false) => InstructionJumpCondition::Jmpz,
            (false, false, true) => InstructionJumpCondition::Jmpp,
            (false, true, false) => InstructionJumpCondition::Jmpg,
            (false, true, true) => InstructionJumpCondition::Jmpc,
            (true, false, false) => InstructionJumpCondition::Jmzg,
            (true, false, true) => InstructionJumpCondition::Jmzl,
            (true, true, false) => InstructionJumpCondition::Jmpl,
            (true, true, true) => InstructionJumpCondition::Jump,
        })
    }
}


/// The `D` bit specifies the direction - `1` for reading in from the port (`PORT IN`) and `0` for writing out to the port
/// (`PORT OUT`).
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionPortDirection {
    In = 0b1000,
    Out = 0b0000,
}

impl From<bool> for InstructionPortDirection {
    fn from(raw: bool) -> InstructionPortDirection {
        match raw {
            true => InstructionPortDirection::In,
            false => InstructionPortDirection::Out,
        }
    }
}


/// The D bit indicates the direction; 0 for PUSH and 1 for POP.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionStckDirection {
    Push = 0b00,
    Pop = 0b10,
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
pub enum InstructionRegisterPair {
    Ab = 0b0,
    Cd = 0b1,
}

impl From<bool> for InstructionRegisterPair {
    fn from(raw: bool) -> InstructionRegisterPair {
        match raw {
            false => InstructionRegisterPair::Ab,
            true => InstructionRegisterPair::Cd,
        }
    }
}


/// Any CPU instruction of the pattern `0011 FFFF` will invoke some function of the ALU.
///
/// The four bits `FFFF` are the actual operation being performed by the ALU.
///
/// The registers X and Y are used as inputs to the ALU (only X for unary operations),
/// and the S register is used to store the result.
///
///
/// ALU operations will also update the F register as noted.
///
/// All will set the ZERO flag if the output (register S) is `0000 0000`.
///
/// All will set the Parity flag if the number of high bits are even.
///
/// FFFF | Name | Count | Description
/// -----|------|-------|------------
/// 0000 | ADD  |   1   | Addition of register X and register Y
/// 0001 | SUB  |   1   | Subtraction of register Y from register X (X-Y)
/// 0010 | NOT  |   1   | Bitwise NOT (unary operation)
/// 0011 |      |   1   | Reserved
/// 0100 |  OR  |   1   | Bitwise OR
/// 0101 | XOR  |   1   | Bitwise XOR
/// 0110 | AND  |   1   | Bitwise AND
/// 0111 |      |   1   | Reserved
/// 1DTT |      |   8   | Shift or Rotate, see section below (unary operation)
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
    pub fn is_valid(self) -> bool {
        match self {
            AluOperation::Reserved(_) => false,
            _ => true,
        }
    }

    /// Perform the ALU operation on the specified operands
    ///
    /// Returns `0` and sets carry for reserved ops.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, AluOperation};
    /// assert_eq!(AluOperation::Or.perform(0b0001, 0b0100, &mut false), 0b0101);
    ///
    /// let mut carry = true;
    /// assert_eq!(AluOperation::ShiftOrRotate {
    ///                d: AluOperationShiftOrRotateDirection::Left,
    ///                tt: AluOperationShiftOrRotateType::Rtc,
    ///            }.perform(0b0101_0000, 0, &mut carry),
    ///            0b1010_0001);
    /// assert_eq!(carry, false);
    ///
    /// let mut carry = false;
    /// assert_eq!(AluOperation::Add.perform(0b1001_0000, 0b1010_0101, &mut carry),
    ///            0b0011_0101);
    /// assert_eq!(carry, true);
    /// ```
    #[inline]
    pub fn perform(self, lhs: u8, rhs: u8, carry: &mut bool) -> u8 {
        self.perform_impl(lhs, rhs, carry)
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

impl Into<u8> for AluOperation {
    fn into(self) -> u8 {
        match self {
            AluOperation::Reserved(raw) => raw,
            AluOperation::Add => 0b0000,
            AluOperation::Sub => 0b0001,
            AluOperation::Not => 0b0010,
            AluOperation::Or => 0b0100,
            AluOperation::Xor => 0b0101,
            AluOperation::And => 0b0110,
            AluOperation::ShiftOrRotate { d, tt } => {
                let tt_b: u8 = tt.into();
                0b1000 | (d as u8) | tt_b
            }
        }
    }
}


/// If D is a `1`, the shift is to the left, all bits will move to a higher value, if D is `0`, it's a right shift, moving bits
/// to lower values.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AluOperationShiftOrRotateDirection {
    Left = 0b100,
    Right = 0b000,
}

impl From<bool> for AluOperationShiftOrRotateDirection {
    fn from(raw: bool) -> AluOperationShiftOrRotateDirection {
        match raw {
            true => AluOperationShiftOrRotateDirection::Left,
            false => AluOperationShiftOrRotateDirection::Right,
        }
    }
}


/// All shifts can be performed left or right, as designated by the D bit of the instruction.
///
/// If D is a `1`, the shift is to the left, all bits will move to a higher value, if D is `0`, it's a right shift,
/// moving bits to lower values.
///
/// There are then four types of shift that can be performed designated by the final two bits of the ALU
/// instruction.
///
/// The name should be appended with an L or R for the direction of the shift, left or right respectively.
///
/// For all shift operations, the bit shifted out is set into the Carry flag.
///
/// TT | Name | Description
/// ---|------|------------
/// 00 | LSF  | Logical shift - a zero is inserted
/// 01 | ASF  | Arithmetic shift - a zero is inserted for left shift, bit-7 (MSB) is inserted for right shift
/// 10 | RTC  | Rotate with carry - the Carry flag is inserted (Carry flag value before it is updated is used)
/// 11 | RTW  | Rotate without carry - the bit shifted out is inserted
///
/// An example of a Arithmetic shift right; `AXXX XXXB` would become `AAXX XXXX`, with `B` copied to the Carry bit.
///
/// **NB:** An 'Arithmetic shift left' is the same as performing a 'Logical shift left', they _can_ be used interchangeably, but
/// 'Arithmetic shift left' should be avoided.
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

impl Into<u8> for AluOperationShiftOrRotateType {
    fn into(self) -> u8 {
        match self {
            AluOperationShiftOrRotateType::Lsf => 0b00,
            AluOperationShiftOrRotateType::Asf => 0b01,
            AluOperationShiftOrRotateType::Rtc => 0b10,
            AluOperationShiftOrRotateType::Rtw => 0b11,
        }
    }
}
