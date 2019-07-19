use std::ops::{RangeToInclusive, RangeInclusive, RangeFull, RangeFrom, RangeTo, IndexMut, Index, Range};
use self::super::MemoryReadWrittenIterator;
use std::hash::{self, Hash};
use std::cmp::Ordering;
use std::fmt;


const MEMORY_LEN: usize = 0xFFFF + 1;


/// Mostly-transparent wrapper for a heap-allocated 64KiB `u8` array with R/W tracking
#[derive(Clone)]
pub struct Memory {
    pub(super) data: Box<[u8; MEMORY_LEN]>,
    pub(super) read: Box<[u64; MEMORY_LEN / 64]>,
    pub(super) written: Box<[u64; MEMORY_LEN / 64]>,
}

impl Memory {
    /// Create fresh zero-initialised unread and unwritten memory
    pub fn new() -> Memory {
        Memory {
            data: Box::new([0; MEMORY_LEN]),
            read: Box::new([0; MEMORY_LEN / 64]),
            written: Box::new([0; MEMORY_LEN / 64]),
        }
    }

    /// Get an iterator over the read and written memory cells
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::vm::Memory;
    /// let mut memory = Memory::new();
    /// memory[0x4B0B] = memory[0x00A1];
    /// println!("{}", memory[0x4B0B]);
    /// memory[0xEB0B] = 0x12;
    ///
    /// // (address, value, was_read, was_written)
    /// assert_eq!(memory.iter_rw().collect::<Vec<_>>(),
    ///            &[(0x00A1, 0x00, true, false),
    ///              (0x4B0B, 0x00, true, true),
    ///              (0xEB0B, 0x12, false, true)]);
    /// ```
    pub fn iter_rw(&self) -> MemoryReadWrittenIterator {
        MemoryReadWrittenIterator::new(&self.data[..], &self.read[..], &self.written[..])
    }

    /// Mark all of memory as unread and unwritten
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::vm::Memory;
    /// let mut memory = Memory::new();
    /// memory[0x4B0B] = memory[0x00A1];
    /// println!("{}", memory[0x4B0B]);
    /// memory[0xEB0B] = 0x12;
    ///
    /// memory.reset_rw();
    /// assert_eq!(memory.iter_rw().collect::<Vec<_>>(), &[]);
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

impl Default for Memory {
    fn default() -> Memory {
        Memory::new()
    }
}

impl From<&[u8]> for Memory {
    /// Read as much data as possible from the specified buffer into this memory
    fn from(data: &[u8]) -> Memory {
        let mut ret = Memory::new();

        let common_len = data.len().min(MEMORY_LEN);
        ret.data[..common_len].copy_from_slice(&data[..common_len]);

        ret
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    #[inline]
    fn index(&self, index: u16) -> &Self::Output {
        let index = index as usize;

        let idx = index / 64;
        let bit = index % 64;
        unsafe {
            *(&self.read[idx] as *const u64 as *mut u64) |= 1 << bit;
        }

        &self.data[index]
    }
}

impl IndexMut<u16> for Memory {
    #[inline]
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        let index = index as usize;

        let idx = index / 64;
        let bit = index % 64;
        self.written[idx] |= 1 << bit;

        &mut self.data[index]
    }
}

impl Index<Range<u16>> for Memory {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: Range<u16>) -> &Self::Output {
        self.data.index(Range {
            start: index.start as usize,
            end: index.end as usize,
        })
    }
}

impl IndexMut<Range<u16>> for Memory {
    #[inline(always)]
    fn index_mut(&mut self, index: Range<u16>) -> &mut Self::Output {
        self.data.index_mut(Range {
            start: index.start as usize,
            end: index.end as usize,
        })
    }
}

impl Index<RangeFrom<u16>> for Memory {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeFrom<u16>) -> &Self::Output {
        self.data.index(RangeFrom { start: index.start as usize })
    }
}

impl IndexMut<RangeFrom<u16>> for Memory {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFrom<u16>) -> &mut Self::Output {
        self.data.index_mut(RangeFrom { start: index.start as usize })
    }
}

impl Index<RangeFull> for Memory {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeFull) -> &Self::Output {
        self.data.index(index)
    }
}

impl IndexMut<RangeFull> for Memory {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl Index<RangeInclusive<u16>> for Memory {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeInclusive<u16>) -> &Self::Output {
        let (start, end) = index.into_inner();
        self.data.index(RangeInclusive::new(start as usize, end as usize))
    }
}

impl IndexMut<RangeInclusive<u16>> for Memory {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeInclusive<u16>) -> &mut Self::Output {
        let (start, end) = index.into_inner();
        self.data.index_mut(RangeInclusive::new(start as usize, end as usize))
    }
}

impl Index<RangeTo<u16>> for Memory {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeTo<u16>) -> &Self::Output {
        self.data.index(RangeTo { end: index.end as usize })
    }
}

impl IndexMut<RangeTo<u16>> for Memory {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeTo<u16>) -> &mut Self::Output {
        self.data.index_mut(RangeTo { end: index.end as usize })
    }
}

impl Index<RangeToInclusive<u16>> for Memory {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeToInclusive<u16>) -> &Self::Output {
        self.data.index(RangeToInclusive { end: index.end as usize })
    }
}

impl IndexMut<RangeToInclusive<u16>> for Memory {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeToInclusive<u16>) -> &mut Self::Output {
        self.data.index_mut(RangeToInclusive { end: index.end as usize })
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Memory")
            .field("data", &&self.data[..])
            .field("read", &&self.read[..])
            .field("written", &&self.written[..])
            .finish()
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
        &self.data[..] == other
    }
}

impl PartialEq<Memory> for Memory {
    #[inline]
    fn eq(&self, other: &Memory) -> bool {
        self.data[..] == other.data[..] && self.read[..] == other.read[..] && self.written[..] == other.written[..]
    }
}

impl Eq for Memory {}

impl PartialOrd for Memory {
    #[inline]
    fn partial_cmp(&self, other: &Memory) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Memory {
    #[inline]
    fn cmp(&self, other: &Memory) -> Ordering {
        Ord::cmp(&self.data[..], &other.data[..])
            .then(Ord::cmp(&self.read[..], &other.read[..]))
            .then(Ord::cmp(&self.written[..], &other.written[..]))
    }
}
