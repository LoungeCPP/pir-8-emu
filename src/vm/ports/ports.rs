use std::ops::{RangeToInclusive, RangeInclusive, RangeFull, RangeFrom, RangeTo, IndexMut, Index, Range};
use std::hash::{self, Hash};
use std::cmp::Ordering;
use std::fmt;


const PORTS_LEN: usize = 0xFF + 1;


/// Mostly-transparent wrapper for a heap-allocated 256B `u8` array for I/O ports
#[derive(Clone)]
pub struct Ports {
    pub(super) data: Box<[u8; PORTS_LEN]>,
    pub(super) read: Box<[u64; PORTS_LEN / 64]>,
    pub(super) written: Box<[u64; PORTS_LEN / 64]>,
}

impl Ports {
    pub fn new() -> Ports {
        Ports {
            data: Box::new([0; PORTS_LEN]),
            read: Box::new([0; PORTS_LEN / 64]),
            written: Box::new([0; PORTS_LEN / 64]),
        }
    }
}

impl Default for Ports {
    fn default() -> Ports {
        Ports::new()
    }
}

impl Index<u8> for Ports {
    type Output = u8;

    #[inline]
    fn index(&self, index: u8) -> &Self::Output {
        let index = index as usize;

        let idx = index / 64;
        let bit = index % 64;
        unsafe {
            *(&self.read[idx] as *const u64 as *mut u64) |= 1 << bit;
        }

        &self.data[index]
    }
}

impl IndexMut<u8> for Ports {
    #[inline]
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        let index = index as usize;

        let idx = index / 64;
        let bit = index % 64;
        self.written[idx] |= 1 << bit;

        &mut self.data[index]
    }
}

impl Index<Range<u8>> for Ports {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: Range<u8>) -> &Self::Output {
        self.data.index(Range {
            start: index.start as usize,
            end: index.end as usize,
        })
    }
}

impl IndexMut<Range<u8>> for Ports {
    #[inline(always)]
    fn index_mut(&mut self, index: Range<u8>) -> &mut Self::Output {
        self.data.index_mut(Range {
            start: index.start as usize,
            end: index.end as usize,
        })
    }
}

impl Index<RangeFrom<u8>> for Ports {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeFrom<u8>) -> &Self::Output {
        self.data.index(RangeFrom { start: index.start as usize })
    }
}

impl IndexMut<RangeFrom<u8>> for Ports {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFrom<u8>) -> &mut Self::Output {
        self.data.index_mut(RangeFrom { start: index.start as usize })
    }
}

impl Index<RangeFull> for Ports {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeFull) -> &Self::Output {
        self.data.index(index)
    }
}

impl IndexMut<RangeFull> for Ports {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl Index<RangeInclusive<u8>> for Ports {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeInclusive<u8>) -> &Self::Output {
        let (start, end) = index.into_inner();
        self.data.index(RangeInclusive::new(start as usize, end as usize))
    }
}

impl IndexMut<RangeInclusive<u8>> for Ports {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeInclusive<u8>) -> &mut Self::Output {
        let (start, end) = index.into_inner();
        self.data.index_mut(RangeInclusive::new(start as usize, end as usize))
    }
}

impl Index<RangeTo<u8>> for Ports {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeTo<u8>) -> &Self::Output {
        self.data.index(RangeTo { end: index.end as usize })
    }
}

impl IndexMut<RangeTo<u8>> for Ports {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeTo<u8>) -> &mut Self::Output {
        self.data.index_mut(RangeTo { end: index.end as usize })
    }
}

impl Index<RangeToInclusive<u8>> for Ports {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeToInclusive<u8>) -> &Self::Output {
        self.data.index(RangeToInclusive { end: index.end as usize })
    }
}

impl IndexMut<RangeToInclusive<u8>> for Ports {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeToInclusive<u8>) -> &mut Self::Output {
        self.data.index_mut(RangeToInclusive { end: index.end as usize })
    }
}

impl fmt::Debug for Ports {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ports")
            .field("data", &&self.data[..])
            .field("read", &&self.read[..])
            .field("written", &&self.written[..])
            .finish()
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
        &self.data[..] == other
    }
}

impl PartialEq<Ports> for Ports {
    #[inline]
    fn eq(&self, other: &Ports) -> bool {
        self.data[..] == other.data[..] && self.read[..] == other.read[..] && self.written[..] == other.written[..]
    }
}

impl Eq for Ports {}

impl PartialOrd for Ports {
    #[inline]
    fn partial_cmp(&self, other: &Ports) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ports {
    #[inline]
    fn cmp(&self, other: &Ports) -> Ordering {
        Ord::cmp(&self.data[..], &other.data[..])
            .then(Ord::cmp(&self.read[..], &other.read[..]))
            .then(Ord::cmp(&self.written[..], &other.written[..]))
    }
}
