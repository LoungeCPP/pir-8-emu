use std::ops::{DerefMut, Deref};
use std::hash::{self, Hash};
use std::cmp::Ordering;
use std::fmt;


/// "Mostly-transparent wrapper for a heap-allocated 256B `u8` array for I/O ports
#[derive(Clone)]
#[repr(transparent)]
pub struct Ports(Box<[u8; 0xFF + 1]>);

impl Ports {
    pub fn new() -> Ports {
        Ports(Box::new([0; 0xFF + 1]))
    }
}

impl Default for Ports {
    fn default() -> Ports {
        Ports::new()
    }
}

impl Deref for Ports {
    type Target = [u8; 0xFF + 1];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ports {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for Ports {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl Hash for Ports {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        Hash::hash(&self[..], state)
    }
}

impl PartialEq<[u8]> for Ports {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self[..] == other[..]
    }

    #[inline]
    fn ne(&self, other: &[u8]) -> bool {
        self[..] != other[..]
    }
}

impl PartialEq<Ports> for Ports {
    #[inline]
    fn eq(&self, other: &Ports) -> bool {
        self[..] == other[..]
    }

    #[inline]
    fn ne(&self, other: &Ports) -> bool {
        self[..] != other[..]
    }
}

impl Eq for Ports {}

impl PartialOrd for Ports {
    #[inline]
    fn partial_cmp(&self, other: &Ports) -> Option<Ordering> {
        PartialOrd::partial_cmp(&&self[..], &&other[..])
    }

    #[inline]
    fn lt(&self, other: &Ports) -> bool {
        PartialOrd::lt(&&self[..], &&other[..])
    }

    #[inline]
    fn le(&self, other: &Ports) -> bool {
        PartialOrd::le(&&self[..], &&other[..])
    }

    #[inline]
    fn ge(&self, other: &Ports) -> bool {
        PartialOrd::ge(&&self[..], &&other[..])
    }

    #[inline]
    fn gt(&self, other: &Ports) -> bool {
        PartialOrd::gt(&&self[..], &&other[..])
    }
}

impl Ord for Ports {
    #[inline]
    fn cmp(&self, other: &Ports) -> Ordering {
        Ord::cmp(&&self[..], &&other[..])
    }
}
