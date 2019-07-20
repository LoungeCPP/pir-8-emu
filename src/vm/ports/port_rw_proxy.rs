use std::borrow::{BorrowMut, Borrow};
use std::ops::{DerefMut, Deref};
use self::super::Ports;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PortReadWriteProxy {
    Read(u8),
    Write(u8, u8, *mut Ports),
}

impl Drop for PortReadWriteProxy {
    fn drop(&mut self) {
        match self {
            PortReadWriteProxy::Read(_) => {}
            PortReadWriteProxy::Write(port_idx, value, ports) => unsafe { (**ports).handle_write(*port_idx, *value) },
        }
    }
}

impl Deref for PortReadWriteProxy {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        match self {
            PortReadWriteProxy::Read(val) => val,
            PortReadWriteProxy::Write(_, value, _) => &value,
        }
    }
}

impl DerefMut for PortReadWriteProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            PortReadWriteProxy::Read(_) => unreachable!(),
            PortReadWriteProxy::Write(_, ref mut value, _) => value,
        }
    }
}

impl Borrow<u8> for PortReadWriteProxy {
    fn borrow(&self) -> &u8 {
        self.deref()
    }
}

impl BorrowMut<u8> for PortReadWriteProxy {
    fn borrow_mut(&mut self) -> &mut u8 {
        self.deref_mut()
    }
}
