use self::super::Ports;
use std::iter::Iterator;


/// Iterator over read-from and written-to bytes of `Ports`
///
/// Created by the [`iter_rw()`](struct.Ports.html#method.iter_rw) method on [`Ports`](struct.Ports.html)
///
/// The item type is `(idx, val, was_read, was_written)`
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PortsReadWriteIterator<'m> {
    pub(super) ports: &'m Ports,
    pub(super) next_idx: u8,
    pub(super) finished: bool,
}

impl Iterator for PortsReadWriteIterator<'_> {
    type Item = (u8, u8, bool, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let mut next_idx = self.next_idx as usize;

        let (was_read, was_written) = loop {
            if next_idx >= self.ports.data.len() {
                self.finished = true;
                return None;
            }

            let idx = next_idx / 64;
            let bit = next_idx % 64;

            let read = self.ports.read[idx];
            let written = self.ports.written[idx];
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

        let idx = next_idx as u8;
        self.next_idx = idx + 1;

        Some((idx, self.ports.data[idx as usize], was_read, was_written))
    }
}
