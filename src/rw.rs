use std::marker::PhantomData;
use std::fmt;


/// Marker for wrapper types that need to track when they were read from/written to.
///
/// # Examples
///
/// ```
/// # use pir_8_emu::ReadWriteMarker;
/// let marker = ReadWriteMarker::new();
///
/// assert_eq!(marker.was_read(), false);
/// assert_eq!(marker.was_written(), false);
///
/// marker.read();
/// assert_eq!(marker.was_read(), true);
///
/// let mut marker = marker;
/// marker.written();
/// assert_eq!(marker.was_written(), true);
///
/// marker.reset();
/// assert_eq!(marker.was_read(), false);
/// assert_eq!(marker.was_written(), false);
/// ```
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReadWriteMarker {
    mask: u8,
    _not_send_nor_sync: PhantomData<*const ()>,
}

impl ReadWriteMarker {
    /// Create a new unread and unwritten marker.
    pub fn new() -> ReadWriteMarker {
        ReadWriteMarker {
            mask: 0b00,
            _not_send_nor_sync: PhantomData,
        }
    }

    /// Mark this marker read.
    pub fn read(&self) {
        unsafe {
            *(&self.mask as *const u8 as *mut u8) |= 0b01;
        }
    }

    /// Mark this marker written.
    pub fn written(&mut self) {
        self.mask |= 0b10;
    }


    /// Check if this marker was marked read.
    pub fn was_read(&self) -> bool {
        return (self.mask & 0b01) != 0;
    }

    /// Check if this marker was marked written.
    pub fn was_written(&self) -> bool {
        return (self.mask & 0b10) != 0;
    }

    /// Reset the marker to its original unread and unwritten state.
    pub fn reset(&mut self) {
        self.mask = 0b00;
    }
}

impl fmt::Debug for ReadWriteMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReadWriteMarker")
            .field("read", &((self.mask & 0b01) != 0))
            .field("written", &((self.mask & 0b10) != 0))
            .finish()
    }
}
