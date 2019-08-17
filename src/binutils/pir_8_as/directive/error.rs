use std::error::Error;
use std::fmt;


/// An error that could've occurred when [obeying](enum.AssemblyDirective.html#method.obey) an
/// [asembly directive](enum.AssemblyDirective.html).
///
/// # Examples
///
/// ```
/// # use pir_8_emu::binutils::pir_8_as::{AssemblerDirectiveObeyError, AssemblerDirective};
/// let mut next_output_address = Some(0x0110);
/// let mut labels = vec![("uwu".to_string(), 0x0069)].into_iter().collect();
/// assert_eq!(AssemblerDirective::SetOrigin(0x0420).obey(&mut next_output_address, &mut labels),
///            Err(AssemblerDirectiveObeyError::OutputAddressAlreadySet(0x0110, 0x0420)));
///
/// assert_eq!(AssemblerDirective::SaveLabel("uwu").obey(&mut next_output_address, &mut labels),
///            Err(AssemblerDirectiveObeyError::LabelNameTaken("uwu")));
/// ```
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssemblerDirectiveObeyError<'s> {
    /// The label wit hthe specified name already exists
    LabelNameTaken(&'s str),

    /// An origin was attempted to be specified with the output address already existing
    ///
    /// First argument is the current output address, the second – the requested
    OutputAddressAlreadySet(u16, u16),
}

impl Error for AssemblerDirectiveObeyError<'_> {}

impl fmt::Display for AssemblerDirectiveObeyError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblerDirectiveObeyError::LabelNameTaken(lbl) => {
                f.write_str("Label name \"")?;
                f.write_str(lbl)?;
                f.write_str("\" already used")
            }

            AssemblerDirectiveObeyError::OutputAddressAlreadySet(current, requested) => {
                write!(f,
                       "Couldn't set origin to {:#06x}, as it was set previously or instructions were processed, and the next output address is {:#06x}",
                       requested,
                       current)
            }
        }
    }
}
