use num_traits::{Unsigned, NumCast, PrimInt, Num};
use std::iter::Iterator;


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryPortsReadWriteIterator<'m, IdxT: Num + Unsigned + PrimInt + NumCast> {
    pub(super) data: &'m [u8],
    pub(super) read: &'m [u64],
    pub(super) written: &'m [u64],

    pub(super) next_idx: IdxT,
    pub(super) finished: bool,
}

impl<IdxT: Num + Unsigned + PrimInt + NumCast> Iterator for MemoryPortsReadWriteIterator<'_, IdxT> {
    type Item = (IdxT, u8, bool, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let mut next_idx = self.next_idx.to_usize().unwrap();

        let (was_read, was_written) = loop {
            if next_idx >= self.data.len() {
                self.finished = true;
                return None;
            }

            let idx = next_idx / 64;
            let bit = next_idx % 64;

            let read = self.read[idx];
            let written = self.written[idx];
            if read == 0 && written == 0 {
                next_idx += 64 - bit;
                continue;
            }

            let mask = 1 << bit;
            let was_read = (read & mask) != 0;
            let was_written = (written & mask) != 0;
            if !was_read && !was_written {
                next_idx += 1;
                continue;
            }

            break (was_read, was_written);
        };

        let idx = IdxT::from(next_idx).unwrap();
        self.next_idx = idx + IdxT::one();

        Some((idx, self.data[next_idx], was_read, was_written))
    }
}
