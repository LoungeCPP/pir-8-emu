use std::ops::{DerefMut, Deref};
use std::hash::{self, Hash};
use std::cmp::Ordering;
use std::fmt;


/// Mostly-transparent wrapper for a heap-allocated 64KiB `u8` array
#[derive(Clone)]
#[repr(transparent)]
pub struct Memory(Box<[u8; 0xFFFF]>);

impl Memory {
    pub fn new() -> Memory {
        Memory(Box::new([0; 0xFFFF]))
    }
}

impl Deref for Memory {
    type Target = [u8; 0xFFFF];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Memory {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl Hash for Memory {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        Hash::hash(&self[..], state)
    }
}

impl PartialEq<[u8]> for Memory {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self[..] == other[..]
    }

    #[inline]
    fn ne(&self, other: &[u8]) -> bool {
        self[..] != other[..]
    }
}

impl PartialEq<Memory> for Memory {
    #[inline]
    fn eq(&self, other: &Memory) -> bool {
        self[..] == other[..]
    }

    #[inline]
    fn ne(&self, other: &Memory) -> bool {
        self[..] != other[..]
    }
}

impl Eq for Memory {}

impl PartialOrd for Memory {
    #[inline]
    fn partial_cmp(&self, other: &Memory) -> Option<Ordering> {
        PartialOrd::partial_cmp(&&self[..], &&other[..])
    }

    #[inline]
    fn lt(&self, other: &Memory) -> bool {
        PartialOrd::lt(&&self[..], &&other[..])
    }

    #[inline]
    fn le(&self, other: &Memory) -> bool {
        PartialOrd::le(&&self[..], &&other[..])
    }

    #[inline]
    fn ge(&self, other: &Memory) -> bool {
        PartialOrd::ge(&&self[..], &&other[..])
    }

    #[inline]
    fn gt(&self, other: &Memory) -> bool {
        PartialOrd::gt(&&self[..], &&other[..])
    }
}

impl Ord for Memory {
    #[inline]
    fn cmp(&self, other: &Memory) -> Ordering {
        Ord::cmp(&&self[..], &&other[..])
    }
}
