//! An instruction is a single byte, and can include some following imediate values purely for data.


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
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    /// Reserved instruction, contains the entire byte
    Reserved(u8),
    /// Load the the next byte into register `AAA` (PC will be incremented a second time)
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
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
        tt: AluOperationShiftOrRotate,
    },
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
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AluOperationShiftOrRotate {
    /// Logical shift - a zero is inserted
    Lsf,
    /// Arithmetic shift - a zero is inserted for left shift, bit-8 is inserted for right shift
    Asf,
    /// Rotate with carry - the Carry flag is inserted (Carry flag value before it is updated is used)
    Rtc,
    /// Rotate without carry - the bit shifted out is is inserted
    Rtw,
}

/// If D is a `1`, the shift is to the left, all bits will move to a higher value, if D is `0`, it's a right shift, moving bits
/// to lower values.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AluOperationShiftOrRotateDirection {
    Left = 1,
    Right = 0,
}

/// The D bit indicates the direction; 0 for PUSH and 1 for POP.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionStckDirection {
    Push = 0,
    Pop = 1,
}

/// The R bit indicates the register pair; 0 for A & B and 1 for C & D.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionStckRegisterPair {
    Ab = 0,
    Cd = 1,
}
