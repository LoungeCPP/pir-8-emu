use std::error::Error;
use std::fmt;


/// An error that could've occurred when performing a [μOp](enum.MicroOp.html).
///
/// # Examples
///
/// ```
/// # use pir_8_emu::isa::{GeneralPurposeRegister, SpecialPurposeRegister};
/// # use pir_8_emu::microcode::{MicrocodeExecutionError, MicroOp};
/// # use pir_8_emu::{Memory, Ports};
/// # let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) =
/// #     (Memory::new(), Ports::new(), GeneralPurposeRegister::defaults(),
/// #      SpecialPurposeRegister::new("Program Counter", "PC"), SpecialPurposeRegister::new("Stack Pointer", "SP"),
/// #      SpecialPurposeRegister::new("Memory Address", "ADR"));
/// assert_eq!(MicroOp::StackPush.execute(&mut vec![], &mut memory, &mut ports, &mut registers,
///                                       &mut pc, &mut sp, &mut adr),
///            Err(MicrocodeExecutionError::MicrostackUnderflow));
/// ```
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MicrocodeExecutionError {
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

impl Error for MicrocodeExecutionError {}

impl fmt::Display for MicrocodeExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MicrocodeExecutionError::MicrostackUnderflow => f.write_str("μStack underflow"),
            MicrocodeExecutionError::InvalidMicrostackTop(actual, valid) => {
                write!(f, "Invalid top of the μstack: {:#04x}, expected any of: ", actual)?;
                write_expected(valid, f)
            }

            MicrocodeExecutionError::StackOverflow => f.write_str("Stack overflow"),
            MicrocodeExecutionError::StackUnderflow => f.write_str("Stack underflow"),

            MicrocodeExecutionError::ProgramOverflow => f.write_str("Program overflow"),
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
