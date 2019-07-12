use self::super::super::super::util::limit_to_width;
use std::ops::{DerefMut, Deref};
use std::fmt;


/// There are eight 8-bit General Purpose registers, each has an internal address for use within the CPU, instructions like
/// 'MOVE' and 'LOAD' can use these addresses.
///
/// The first four registers have some special functionality, as described, the second four have no special functionality but
/// can be used with the stack.
///
/// Address | Letter | Description
/// --------|--------|------------
/// 000     | F      | Flag register (can also be used to get a zero value)
/// 001     | S      | Output of the ALU - ALU operations will overwrite any value stored
/// 010     | X      | Input to ALU (Only input for unary operations)
/// 011     | Y      | Second input for ALU
/// 100     | A      |
/// 101     | B      |
/// 110     | C      |
/// 111     | D      |
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeneralPurposeRegister {
    data: u8,
    /// Verified to 3 bits at construction time
    address: u8,
    letter: char,
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
            letter: letter,
        })
    }

    /// The address for this register
    ///
    /// Limited to 3 bits' width
    pub fn address(&self) -> u8 {
        self.address
    }

    /// The letter/mnemonic for this register
    ///
    /// E.g. "X"
    pub fn letter(&self) -> char {
        self.letter
    }
}

impl Deref for GeneralPurposeRegister {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for GeneralPurposeRegister {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl fmt::Display for GeneralPurposeRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({:02X})", self.letter, self.data)
    }
}
