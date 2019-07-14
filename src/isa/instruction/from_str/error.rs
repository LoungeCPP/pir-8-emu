use std::error::Error;
use std::fmt;


/// An error that could've occurred when parsing an [`Instruction`](enum.Instruction.html) or an
/// [`AluOperation`](enum.AluOperation.html).
///
/// # Examples
///
/// ```
/// # use pir_8_emu::isa::instruction::{ParseInstructionError, Instruction};
/// # use pir_8_emu::isa::GeneralPurposeRegister;
/// # let registers = GeneralPurposeRegister::defaults();
/// let res = Instruction::from_str("LOAD IND Q", &registers);
/// assert_eq!(res,
///            Err(ParseInstructionError::UnrecognisedRegisterLetter(
///                10, 'Q', ['F', 'S', 'X', 'Y', 'A', 'B', 'C', 'D'])));
///
/// assert_eq!(format!("\nLOAD IND Q\n{}\n", res.unwrap_err()),
///            "\nLOAD IND Q\n          ^ Register Q not found; expected: F, S, X, Y, A, B, C, D\n");
/// /*
///            "
/// LOAD IND Q
///          ^ Register Q not found; expected: F, S, X, Y, A, B, C, D
/// ");
/// */
///
/// let res = Instruction::from_str("   PORT", &registers);
/// assert_eq!(res, Err(ParseInstructionError::MissingToken(8, &["IN", "OUT"])));
///
/// assert_eq!(format!("\n   PORT\n{}\n", res.unwrap_err()),
///            "\n   PORT\nIN, OUT ^ <- expected: Missing token\n");
/// /*
///            "
///    PORT
/// IN, OUT ^ <- expected: Missing token
/// ");
/// */
/// ```
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseInstructionError {
    /// An invalid (non-ASCII/vertical-WS) character was found in the input string
    InvalidCharacter(usize),
    /// The string to be parsed contained no tokens
    EmptyString,

    /// A token was specified that doesn't fit in the place it was used
    UnrecognisedToken(usize, &'static [&'static str]),
    /// The specified register letter was specified but no register with that letter exists
    UnrecognisedRegisterLetter(usize, char, [char; 8]),

    /// A token was not specified, but one was required in that place
    MissingToken(usize, &'static [&'static str]),
    /// A register name was not specified, but one was required in that place
    MissingRegisterLetter(usize, [char; 8]),

    /// A token was specified after a successful parse
    TooManyTokens(usize),
}

impl Error for ParseInstructionError {}

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseInstructionError::InvalidCharacter(idx) => simple_message("Invalid (non-ASCII/vertical-WS) character", *idx, f),
            ParseInstructionError::EmptyString => f.write_str("No tokens"),

            ParseInstructionError::UnrecognisedToken(idx, expected) => expected_message("Unrecognised token", expected, *idx, f),
            ParseInstructionError::UnrecognisedRegisterLetter(idx, letter, regs) => expected_message(&format!("Register {} not found", letter), regs, *idx, f),

            ParseInstructionError::MissingToken(idx, expected) => expected_message("Missing token", expected, *idx, f),
            ParseInstructionError::MissingRegisterLetter(idx, regs) => expected_message("Missing register letter", regs, *idx, f),

            ParseInstructionError::TooManyTokens(idx) => simple_message("Too many tokens", *idx, f),
        }
    }
}

fn simple_message(msg: &str, idx: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if idx != 0 {
        simple_message_with_len(msg, idx - 1, f)
    } else {
        write!(f, "^ {}", msg)
    }
}

fn expected_message<X: Expectable>(msg: &str, expected: &[X], idx: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if idx != 0 {
        expected_message_with_len(msg, expected, idx - 1, f)
    } else {
        write!(f, "^ {}; expected: ", msg)?;
        write_expected(expected, f)
    }
}

fn simple_message_with_len(msg: &str, len: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if msg.len() + 1 <= len {
        write!(f, "{:w$}{} ^", "", msg, w = len - msg.len() - 1)
    } else {
        write!(f, "{:w$}^ {}", "", msg, w = len + 1)
    }
}

fn expected_message_with_len<X: Expectable>(msg: &str, expected: &[X], len: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let expected_len = expected.iter().fold(0, |acc, cur| acc + cur.len()) + 2 * (expected.len() - 1);

    if msg.len() + "; expected: ".len() + expected_len <= len {
        write!(f, "{:w$}{}; expected: ", "", msg, w = len - msg.len() - "; expected: ".len() - expected_len)?;
        write_expected(expected, f)?;
        f.write_str(" ^")
    } else if msg.len() + "; expected: ".len() <= len {
        write!(f, "{:w$}{}; expected: ^ ", "", msg, w = len - msg.len() - "; expected: ".len())?;
        write_expected(expected, f)
    } else if msg.len() <= len {
        write!(f, "{:w$}{} ^ expected: ", "", msg, w = len - msg.len())?;
        write_expected(expected, f)
    } else if "Expected: ".len() + expected_len <= len {
        write!(f, "{:w$}", "", w = len - "Expected: ".len() - expected_len)?;
        f.write_str("Expected: ")?;
        write_expected(expected, f)?;
        f.write_str(" ^ ")?;
        f.write_str(msg)
    } else if expected_len <= len {
        write!(f, "{:w$}", "", w = len - expected_len)?;
        write_expected(expected, f)?;
        f.write_str(" ^ <- expected: ")?;
        f.write_str(msg)
    } else {
        write!(f, "{:w$}^ {}; expected: ", "", msg, w = len + 1)?;
        write_expected(expected, f)
    }
}

fn write_expected<X: Expectable>(expected: &[X], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, x) in expected.iter().enumerate() {
        if i != 0 {
            f.write_str(", ")?;
        }
        x.write(f)?;
    }
    Ok(())
}


trait Expectable {
    fn len(&self) -> usize;
    fn write(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl Expectable for &str {
    #[inline(always)]
    fn len(&self) -> usize {
        (self as &str).len()
    }

    #[inline(always)]
    fn write(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self)
    }
}

impl Expectable for char {
    #[inline(always)]
    fn len(&self) -> usize {
        1
    }

    #[inline(always)]
    fn write(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
