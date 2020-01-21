use self::super::super::super::{ReadWriteMarker, ReadWritable};
use num_traits::{Unsigned, PrimInt, Num};
use std::ops::{DerefMut, Deref};
use std::mem::size_of;
use std::fmt;


/// There are some special purpose registers that you cannot directly read/write from, these are used by the CPU for its
/// internal state.
///
/// All the registers will start with an initial value of `0`.
///
/// There are three 16-bit registers for holding significant memory addresses and a single 8-bit register.
///
/// Name            | Short | Bits | Description
/// ----------------|-------|------|------------
/// Program Counter | PC    |  16  | Address of the next instruction to be fetched
/// Stack Pointer   | SP    |  16  | Current address of the stack (detailed later)
/// Memory Address  | ADR   |  16  | Address saved for use during certain instructions
/// Instruction     | INS   |   8  | Instruction currently being executed
///
/// The address bus is controlled by either `PC`, `SP` or `ADR`.
///
/// As the CPU is reading an instruction from RAM, the value of the `PC` will be used, for some instructions though,
/// such as `JUMP` or `STACK`, it is value of `ADR` or `SP` respectively that is used.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecialPurposeRegister<T: Num + Unsigned + PrimInt> {
    data: T,
    name: &'static str,
    short: &'static str,
    rw: ReadWriteMarker,
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
            rw: ReadWriteMarker::new(),
        }
    }

    /// The full name of this register
    ///
    /// E.g. "Program Counter"
    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// The short/mnemonical name of this register
    ///
    /// E.g. "PC"
    #[inline]
    pub fn short_name(&self) -> &'static str {
        self.short
    }
}

impl<T: Num + Unsigned + PrimInt> ReadWritable for SpecialPurposeRegister<T> {
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

impl<T: Num + Unsigned + PrimInt> Deref for SpecialPurposeRegister<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.rw.read();
        &self.data
    }
}

impl<T: Num + Unsigned + PrimInt> DerefMut for SpecialPurposeRegister<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.rw.written();
        &mut self.data
    }
}

impl<T: Num + Unsigned + PrimInt + fmt::Display + fmt::UpperHex> fmt::Display for SpecialPurposeRegister<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({:0w$X})", self.short, self.data, w = size_of::<T>() * 2)
    }
}
