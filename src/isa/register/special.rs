use num_traits::{Unsigned, PrimInt, Num};
use std::ops::{DerefMut, Deref};
use std::mem::size_of;
use std::fmt;


/// There are some special purpose registers that you cannot directly read/write from, these are used by the CPU for its
/// internal state.
///
/// There are three 16 bit registers for holding significant memory addresses and a single 8 bit register.
///
/// Name            | Short | Bits | Description
/// ----------------|-------|------|------------
/// Program Counter | PC    |  16  | Address of the next instruction to be fetched
/// Stack Pointer   | SP    |  16  | Current address of the stack (detailed later)
/// Memory Address  | ADR   |  16  | Current address of RAM being accessed
/// Instruction     | INS   |   8  | Instruction currently being executed
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecialPurposeRegister<T: Num + Unsigned + PrimInt> {
    data: T,
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
    /// assert_eq!(*pc, 0);
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

impl<T: Num + Unsigned + PrimInt> Deref for SpecialPurposeRegister<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Num + Unsigned + PrimInt> DerefMut for SpecialPurposeRegister<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Num + Unsigned + PrimInt + fmt::Display + fmt::UpperHex> fmt::Debug for SpecialPurposeRegister<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({:0w$X})", self.short, self.data, w = size_of::<T>() * 2)
    }
}
