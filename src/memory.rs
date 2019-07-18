use std::ops::{RangeToInclusive, RangeInclusive, RangeFull, RangeFrom, RangeTo, IndexMut, Index, Range};
use self::super::ReadWriteMarker;
use std::hash::{self, Hash};
use std::cmp::Ordering;
use std::fmt;


macro_rules! index_passthrough {
    ($idx_tp:ty) => {
        impl Index<$idx_tp> for Memory {
            type Output = [MemoryCell];

            fn index(&self, index: $idx_tp) -> &Self::Output {
                self.0.index(index)
            }
        }

        impl IndexMut<$idx_tp> for Memory {
            fn index_mut(&mut self, index: $idx_tp) -> &mut Self::Output {
                self.0.index_mut(index)
            }
        }
    };
}


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryCell {
    inner: u8,
    rw: ReadWriteMarker,
}

impl MemoryCell {
    pub fn new() -> MemoryCell {
        MemoryCell {
            inner: 0,
            rw: ReadWriteMarker::new(),
        }
    }
}


/// Mostly-transparent wrapper for a heap-allocated 64KiB `u8` array
///
/// TODO: optimise this to not use 2x the memory it needs, because that's a huge performance hit
#[derive(Clone)]
#[repr(transparent)]
pub struct Memory(Box<[MemoryCell; 0xFFFF + 1]>);

impl Memory {
    pub fn new() -> Memory {
        Memory(Box::new([MemoryCell::new(); 0xFFFF + 1]))
    }
}

impl Default for Memory {
    fn default() -> Memory {
        Memory::new()
    }
}

impl From<&[u8]> for Memory {
    fn from(data: &[u8]) -> Self {
        let mut ret = Memory::new();

        for (c, d) in ret[..].iter_mut().zip(data.iter()) {
            c.inner = *d;
        }

        ret
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let cell = &self.0[index];
        cell.rw.read();
        &cell.inner
    }
}

impl IndexMut<usize> for Memory {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let cell = &mut self.0[index];
        cell.rw.written();
        &mut cell.inner
    }
}

index_passthrough!(Range<usize>);
index_passthrough!(RangeFrom<usize>);
index_passthrough!(RangeFull);
index_passthrough!(RangeInclusive<usize>);
index_passthrough!(RangeTo<usize>);
index_passthrough!(RangeToInclusive<usize>);

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
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
        self.0.iter().zip(other.iter()).all(|(c, o)| c.inner == *o)
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
