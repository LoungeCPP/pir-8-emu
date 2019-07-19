use num_traits::{Unsigned, NumCast, PrimInt, Num};
use std::marker::PhantomData;
use std::iter::Iterator;


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryPortsReadWrittenIterator<'m, IdxT: Num + Unsigned + PrimInt + NumCast> {
    data: &'m [u8],
    read: &'m [u64],
    written: &'m [u64],

    next_idx: usize,
    finished: bool,

    idx: PhantomData<IdxT>,
}

impl<'m, IdxT: Num + Unsigned + PrimInt + NumCast> MemoryPortsReadWrittenIterator<'m, IdxT> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub(super) fn new(data: &'m [u8], read: &'m [u64], written: &'m [u64]) -> MemoryPortsReadWrittenIterator<'m, IdxT> {
        MemoryPortsReadWrittenIterator {
            data: data,
            read: read,
            written: written,
            next_idx: 0,
            finished: false,
            idx: PhantomData,
        }
    }
}

impl<IdxT: Num + Unsigned + PrimInt + NumCast> Iterator for MemoryPortsReadWrittenIterator<'_, IdxT> {
    type Item = (IdxT, u8, bool, bool);

    fn next(&mut self) -> Option<Self::Item> {
        next(self.data, self.read, self.written, &mut self.next_idx, &mut self.finished).map(|(idx, dt, wr, ww)| (IdxT::from(idx).unwrap(), dt, wr, ww))
    }
}


fn next(self_data: &[u8], self_read: &[u64], self_written: &[u64], next_idx: &mut usize, self_finished: &mut bool) -> Option<(usize, u8, bool, bool)> {
    if *self_finished {
        return None;
    }

    let (was_read, was_written) = loop {
        if *next_idx >= self_data.len() {
            *self_finished = true;
            return None;
        }

        let idx = *next_idx / 64;
        let bit = *next_idx % 64;

        let read = self_read[idx];
        let written = self_written[idx];
        if read == 0 && written == 0 {
            *next_idx += 64 - bit;
            continue;
        }

        let mask = 1 << bit;
        let was_read = (read & mask) != 0;
        let was_written = (written & mask) != 0;
        if !was_read && !was_written {
            *next_idx += 1;
            continue;
        }

        break (was_read, was_written);
    };

    let idx = *next_idx;
    *next_idx += 1;

    Some((idx, self_data[idx], was_read, was_written))
}
