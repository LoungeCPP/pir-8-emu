use self::super::super::super::{ReadWriteMarker, ReadWritable};
use self::super::super::super::util::limit_to_width;
use self::super::GeneralPurposeRegisterBank;
use std::ops::{DerefMut, Deref};
use std::fmt;


/// There are eight 8-bit General Purpose registers, each has an internal address for use within the CPU, instructions like
/// 'MOVE' and 'LOAD' can use these addresses.
///
/// All the registers will start with an initial value of `0`.
///
/// There are eight 8-bit General Purpose registers, each has an internal address for use within the CPU, instructions like
/// 'MOVE' and 'LOAD' can use these addresses. The first five registers have some special functionality, as described, the last
/// three have no special functionality.
///
/// The last four registers can also be used with the stack.
///
/// Address | Letter | Description
/// --------|--------|------------
/// 000     | F      | Flag register (can also be used to get a zero value)
/// 001     | S      | Output of the ALU - ALU operations will overwrite any value stored
/// 010     | X      | Input to ALU (Only input for unary operations)
/// 011     | Y      | Second input for ALU
/// 100     | A      | Port number for PORT instruction
/// 101     | B      |
/// 110     | C      |
/// 111     | D      |
///
/// ## Flag register
///
/// The flag register can be be read and written to as a general purpose register, though keep in mind that ALU and Compare
/// instructions can effect the value of the flags.
///
/// Not all of the bits have a specified role (yet), though the CLRF operation will still clear them.
///
/// A value of `1` denotes the flag as 'set', whilst a value of `0` denotes  the flag is 'unset'.
///
/// Below is a description of what each bit in the flag register denotes.
///
/// Bit | Letter | Description
/// ----|--------|------------
/// 0   | Z      | Zero flag
/// 1   | C      | Carry flag
/// 2   | P      | Parity (even number of set bits)
/// 3   | E      | Equals flag
/// 4   | G      | Greater than
/// 5   |        |
/// 6   |        |
/// 7   |        |
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeneralPurposeRegister {
    data: u8,
    /// Verified to 3 bits at construction time
    address: u8,
    letter: char,
    rw: ReadWriteMarker,
}

impl GeneralPurposeRegister {
    /// Create a new, empty register named as specified.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::GeneralPurposeRegister;
    /// let x = GeneralPurposeRegister::new(0b010, 'X').unwrap();
    /// assert_eq!(*x, 0);
    /// assert_eq!(x.address(), 0b010);
    /// assert_eq!(x.letter(), 'X');
    ///
    /// assert_eq!(GeneralPurposeRegister::new(0b1000, 'Q'), None);
    /// ```
    pub fn new(address: u8, letter: char) -> Option<GeneralPurposeRegister> {
        Some(GeneralPurposeRegister {
            data: 0,
            address: limit_to_width(address, 3)?,
            letter: if letter.is_ascii() {
                Some(letter)
            } else {
                None
            }?,
            rw: ReadWriteMarker::new(),
        })
    }

    /// The address for this register
    ///
    /// Limited to 3 bits' width
    #[inline]
    pub fn address(self) -> u8 {
        self.address
    }

    /// The letter/mnemonic for this register
    ///
    /// E.g. "X"
    #[inline]
    pub fn letter(self) -> char {
        self.letter
    }

    /// Change this register's letter to the specified one
    ///
    /// Will return `Err(())` if the letter isn't ASCII
    pub fn relabel(&mut self, to_letter: char) -> Result<(), ()> {
        if to_letter.is_ascii() {
            self.letter = to_letter;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl GeneralPurposeRegister {
    /// Get the default 8 GP registers specified in the ISA
    pub fn defaults() -> GeneralPurposeRegisterBank {
        [GeneralPurposeRegister::new(0b000, 'F').expect("F register"), // Flag register (can also be used to get a zero value)
         GeneralPurposeRegister::new(0b001, 'S').expect("S register"), // Output of the ALU - ALU operations will overwrite any value stored
         GeneralPurposeRegister::new(0b010, 'X').expect("X register"), // Input to ALU (Only input for unary operations)
         GeneralPurposeRegister::new(0b011, 'Y').expect("Y register"), // Second input for ALU
         GeneralPurposeRegister::new(0b100, 'A').expect("A register"),
         GeneralPurposeRegister::new(0b101, 'B').expect("B register"),
         GeneralPurposeRegister::new(0b110, 'C').expect("C register"),
         GeneralPurposeRegister::new(0b111, 'D').expect("D register")]
    }

    /// Create a GP register bank from letters contained within the specified string
    ///
    /// The string must be 8-ASCII-characters-long.
    ///
    /// If it's shorter, `Err(-1)` is returned,
    /// if it's longer, `Err(8)` is returned,
    /// if character for register at address `AAA` is non-ASCII, `Err(AAA)` is returned,
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::GeneralPurposeRegister;
    /// assert_eq!(GeneralPurposeRegister::from_letters("FSXYABCD"), Ok(GeneralPurposeRegister::defaults()));
    /// ```
    pub fn from_letters(s: &str) -> Result<GeneralPurposeRegisterBank, i8> {
        let mut cc = s.chars();

        let ret = [GeneralPurposeRegister::new(0b000, cc.next().ok_or(-1)?).ok_or(0b000)?,
                   GeneralPurposeRegister::new(0b001, cc.next().ok_or(-1)?).ok_or(0b001)?,
                   GeneralPurposeRegister::new(0b010, cc.next().ok_or(-1)?).ok_or(0b010)?,
                   GeneralPurposeRegister::new(0b011, cc.next().ok_or(-1)?).ok_or(0b011)?,
                   GeneralPurposeRegister::new(0b100, cc.next().ok_or(-1)?).ok_or(0b100)?,
                   GeneralPurposeRegister::new(0b101, cc.next().ok_or(-1)?).ok_or(0b101)?,
                   GeneralPurposeRegister::new(0b110, cc.next().ok_or(-1)?).ok_or(0b110)?,
                   GeneralPurposeRegister::new(0b111, cc.next().ok_or(-1)?).ok_or(0b111)?];

        if cc.next().is_some() { Err(8) } else { Ok(ret) }
    }
}

impl ReadWritable for GeneralPurposeRegister {
    #[inline]
    fn was_read(&self) -> bool {
        self.rw.was_read()
    }

    #[inline]
    fn was_written(&self) -> bool {
        self.rw.was_written()
    }

    #[inline]
    fn reset_rw(&mut self) {
        self.rw.reset()
    }
}

impl Deref for GeneralPurposeRegister {
    type Target = u8;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.rw.read();
        &self.data
    }
}

impl DerefMut for GeneralPurposeRegister {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.rw.written();
        &mut self.data
    }
}

impl fmt::Display for GeneralPurposeRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({:02X})", self.letter, self.data)
    }
}
