use std::error::Error;
use std::fmt;


/// An error that could've occurred when [installing a port handler](struct.Ports.html#method.install_handler).
///
/// # Examples
///
/// ```
/// # use pir_8_emu::vm::{PortHandlerInstallError, PortHandler, Ports};
/// # use std::num::NonZeroU8;
/// # #[derive(Debug, PartialEq, Eq)]
/// struct NopHandler;
/// impl PortHandler for NopHandler {
///     fn port_count(&self) -> NonZeroU8 { NonZeroU8::new(3).unwrap() }
/// #   fn init(&mut self, _: &[u8]) {}
/// #   fn uninit(&mut self) {}
/// #   fn handle_read(&mut self, _: u8) -> u8 { 0 }
/// #   fn handle_write(&mut self, _: u8, _: u8) {}
/// }
///
/// let mut ports = Ports::new();
/// assert_eq!(ports.install_handler(NopHandler, &[0, 1]),
///            Err((NopHandler, PortHandlerInstallError::WrongPortCount(2, 3))));
///
/// ports.install_handler(NopHandler, &[0, 1, 2]).unwrap();
///
/// assert_eq!(ports.install_handler(NopHandler, &[1, 2, 3]),
///            Err((NopHandler, PortHandlerInstallError::PortsTaken(vec![1, 2]))));
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PortHandlerInstallError {
    /// The specified ports were already taken by some other handler
    PortsTaken(Vec<u8>),

    /// The specified port count was specified, but the handler takes only the specified amount of ports
    WrongPortCount(usize, u8),

    /// Installing this handler would overflow
    TooManyHandlers,
}

impl Error for PortHandlerInstallError {}

impl fmt::Display for PortHandlerInstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortHandlerInstallError::PortsTaken(taken) => {
                f.write_str("The following ports were taken: ")?;
                write_taken(&taken, f)
            }

            PortHandlerInstallError::WrongPortCount(provided, supported) => write!(f, "Provided {} ports, handler supports {}", provided, supported),

            PortHandlerInstallError::TooManyHandlers => f.write_str("Too many handlers"),
        }
    }
}

fn write_taken(expected: &[u8], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, x) in expected.iter().enumerate() {
        if i != 0 {
            f.write_str(", ")?;
        }
        write!(f, "{:#04x}", x)?;
    }
    Ok(())
}
