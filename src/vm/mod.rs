//! Various parts of the virtual machine implementation


mod cpu;
mod ports;
mod memory;
mod port_handler;
mod mem_ports_rw_iter;
mod port_handler_install_error;

pub use self::cpu::Cpu;
pub use self::ports::Ports;
pub use self::memory::Memory;
pub use self::port_handler::PortHandler;
pub use self::port_handler_install_error::PortHandlerInstallError;


/// Iterator over read-from and written-to parts of `Ports`
///
/// Created by the [`iter_rw()`](struct.Ports.html#method.iter_rw) method on [`Ports`](struct.Ports.html)
///
/// The item type is `(idx, val, was_read, was_written)`
pub type PortsReadWrittenIterator<'p> = self::mem_ports_rw_iter::MemoryPortsReadWrittenIterator<'p, u8>;

/// Iterator over read-from and written-to bytes of `Memory`
///
/// Created by the [`iter_rw()`](struct.Memory.html#method.iter_rw) method on [`Memory`](struct.Memory.html)
///
/// The item type is `(idx, val, was_read, was_written)`
pub type MemoryReadWrittenIterator<'m> = self::mem_ports_rw_iter::MemoryPortsReadWrittenIterator<'m, u16>;
