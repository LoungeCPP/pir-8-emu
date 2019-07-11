use num_traits::{Unsigned, PrimInt, Num};
use std::mem::size_of;
use std::fmt;


#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecialPurposeRegister<T: Num + Unsigned + PrimInt> {
    pub data: T,

    name: &'static str,
    short: &'static str,
}

impl<T: Num + Unsigned + PrimInt> SpecialPurposeRegister<T> {
    /// Create a new, empty register named as specified.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::SpecialPurposeRegister;
    /// let pc = SpecialPurposeRegister::<u16>::new("Program Counter", "PC");
    /// assert_eq!(pc.data, 0);
    /// assert_eq!(pc.name(), "Program Counter");
    /// assert_eq!(pc.short_name(), "PC");
    /// ```
    pub fn new(name: &'static str, short: &'static str) -> SpecialPurposeRegister<T> {
        SpecialPurposeRegister {
            data: T::zero(),
            name: name,
            short: short,
        }
    }
}

impl<T: Num + Unsigned + PrimInt> SpecialPurposeRegister<T> {
    /// The full name of this register
    ///
    /// E.g. "Program Counter"
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// The short/mnemonical name of this register
    ///
    /// E.g. "PC"
    pub fn short_name(&self) -> &'static str {
        self.short
    }
}

impl<T: Num + Unsigned + PrimInt + fmt::Display + fmt::UpperHex> fmt::Debug for SpecialPurposeRegister<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({:0w$X})", self.short, self.data, w = size_of::<T>() * 2)
    }
}
