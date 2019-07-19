use std::error::Error;
use std::fmt;


/// An error that could've occurred when performing a [μOp](enum.MicroOp.html).
///
/// # Examples
///
/// ```
/// # use pir_8_emu::isa::{GeneralPurposeRegister, SpecialPurposeRegister};
/// # use pir_8_emu::micro::{MicroOpPerformError, MicroOp};
/// # use pir_8_emu::vm::{Memory, Ports};
/// # let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) =
/// #     (Memory::new(), Ports::new(), GeneralPurposeRegister::defaults(),
/// #      SpecialPurposeRegister::new("Program Counter", "PC"), SpecialPurposeRegister::new("Stack Pointer", "SP"),
/// #      SpecialPurposeRegister::new("Memory Address", "ADR"), SpecialPurposeRegister::new("Instruction", "INS"));
/// assert_eq!(MicroOp::StackPush.perform(&mut vec![], &mut memory, &mut ports, &mut registers,
///                                       &mut pc, &mut sp, &mut adr, &mut ins),
///            Err(MicroOpPerformError::MicrostackUnderflow));
/// ```
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MicroOpPerformError {
    /// The microstack had too few elements for the μOp being executed
    MicrostackUnderflow,
    /// The top of the microstack had a value outside the domain of the μOp being executed
    InvalidMicrostackTop(u8, &'static [u8]),

    /// Stack Pointer would overflow
    StackOverflow,
    /// Stack Pointer would underflow
    StackUnderflow,

    /// Program Counter would overflow
    ProgramOverflow,
}

impl Error for MicroOpPerformError {}

impl fmt::Display for MicroOpPerformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MicroOpPerformError::MicrostackUnderflow => f.write_str("μstack underflow"),
            MicroOpPerformError::InvalidMicrostackTop(actual, valid) => {
                write!(f, "Invalid top of the μstack: {:#04x}, expected any of: ", actual)?;
                write_expected(valid, f)
            }

            MicroOpPerformError::StackOverflow => f.write_str("Stack overflow"),
            MicroOpPerformError::StackUnderflow => f.write_str("Stack underflow"),

            MicroOpPerformError::ProgramOverflow => f.write_str("Program overflow"),
        }
    }
}

fn write_expected(expected: &[u8], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, x) in expected.iter().enumerate() {
        if i != 0 {
            f.write_str(", ")?;
        }
        write!(f, "{:#04x}", x)?;
    }
    Ok(())
}
