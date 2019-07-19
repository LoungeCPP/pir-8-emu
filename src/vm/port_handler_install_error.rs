use std::error::Error;
use std::fmt;


/// An error that could've occurred when performing a [μOp](enum.MicroOp.html).
///
/// # Examples
///
/// ```
/// # use pir_8_emu::vm::{PortHandlerInstallError, PortHandler, Ports};
/// struct NopHandler;
/// impl PortHandler for NopHandler {
///     fn port_count(&self) -> u8 { 3 }
/// #   fn init(&mut self, _: &[u8]) { }
/// #   fn clone(&self) -> Box<PortHandler> { Box::new(NopHandler) }
/// }
///
/// let mut ports = Ports::new();
/// assert_eq!(ports.install_handler(NopHandler, &[0, 1]).map_err(|(_, e)| e),
///            Err(PortHandlerInstallError::WrongPortCount(2, 3)));
///
/// ports.install_handler(NopHandler, &[0, 1, 2]).map_err(|(_, e)| e).unwrap();
///
/// assert_eq!(ports.install_handler(NopHandler, &[1, 2, 3]).map_err(|(_, e)| e),
///            Err(PortHandlerInstallError::PortsTaken(vec![1, 2])));
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PortHandlerInstallError {
    /// The specified ports were already taken by some other handler
    PortsTaken(Vec<u8>),

    /// The specified port count was specified, but the handler takes only the specified amount of ports
    WrongPortCount(usize, u8),
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
