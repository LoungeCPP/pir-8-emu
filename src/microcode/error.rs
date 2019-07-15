use std::error::Error;
use std::fmt;


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MicrocodeExecutionError {
    MicrostackUnderflow,
    InvalidMicrostackTop(u8, &'static [u8]),

    StackOverflow,
    StackUnderflow,

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
            },

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
