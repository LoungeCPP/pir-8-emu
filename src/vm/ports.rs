use std::ops::{RangeToInclusive, RangeInclusive, RangeFull, RangeFrom, RangeTo, IndexMut, Index, Range};
use self::super::PortsReadWriteIterator;
use std::marker::PhantomData;
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
    /// Create fresh zero-initialised unread and unwritten ports
    pub fn new() -> Ports {
        Ports {
            data: Box::new([0; PORTS_LEN]),
            read: Box::new([0; PORTS_LEN / 64]),
            written: Box::new([0; PORTS_LEN / 64]),
        }
    }

    /// Get an iterator over the read and written port cells
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::vm::Ports;
    /// let mut ports = Ports::new();
    /// ports[0x4B] = ports[0xA1];
    /// println!("{}", ports[0x4B]);
    /// ports[0xEB] = 0x12;
    ///
    /// // (address, value, was_read, was_written)
    /// assert_eq!(ports.iter_rw().collect::<Vec<_>>(),
    ///            &[(0x4B, 0x00, true, true),
    ///              (0xA1, 0x00, true, false),
    ///              (0xEB, 0x12, false, true)]);
    /// ```
    pub fn iter_rw(&self) -> PortsReadWriteIterator {
        PortsReadWriteIterator {
            data: &self.data[..],
            read: &self.read[..],
            written: &self.written[..],
            next_idx: 0,
            finished: false,
            idx: PhantomData,
        }
    }

    /// Mark all ports as unread and unwritten
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::vm::Ports;
    /// let mut ports = Ports::new();
    /// ports[0x4B] = ports[0xA1];
    /// println!("{}", ports[0x4B]);
    /// ports[0xEB] = 0x12;
    ///
    /// ports.reset_rw();
    /// assert_eq!(ports.iter_rw().collect::<Vec<_>>(), &[]);
    /// ```
    pub fn reset_rw(&mut self) {
        for r in &mut self.read[..] {
            *r = 0;
        }

        for w in &mut self.written[..] {
            *w = 0;
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
