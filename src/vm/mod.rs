//! Various parts of the virtual machine implementation


mod cpu;
mod ports;
mod memory;
mod mem_ports_rw_iter;

pub use self::cpu::Cpu;
pub use self::ports::Ports;
pub use self::memory::Memory;


/// Iterator over read-from and written-to parts of `Ports`
///
/// Created by the [`iter_rw()`](struct.Ports.html#method.iter_rw) method on [`Ports`](struct.Ports.html)
///
/// The item type is `(idx, val, was_read, was_written)`
pub type PortsReadWriteIterator<'p> = self::mem_ports_rw_iter::MemoryPortsReadWriteIterator<'p, u8>;

/// Iterator over read-from and written-to bytes of `Memory`
///
/// Created by the [`iter_rw()`](struct.Memory.html#method.iter_rw) method on [`Memory`](struct.Memory.html)
///
/// The item type is `(idx, val, was_read, was_written)`
pub type MemoryReadWriteIterator<'m> = self::mem_ports_rw_iter::MemoryPortsReadWriteIterator<'m, u16>;
