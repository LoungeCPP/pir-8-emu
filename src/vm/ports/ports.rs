use std::ops::{RangeToInclusive, RangeInclusive, RangeFull, RangeFrom, RangeTo, IndexMut, Index, Range};
use self::super::{PortHandlerInstallError, PortHandler};
use self::super::super::PortsReadWrittenIterator;
use std::hash::{self, Hash};
use std::num::NonZeroU16;
use std::cmp::Ordering;
use std::fmt;


const PORTS_LEN: usize = 0xFF + 1;


/// 256B of I/O ports with R/W tracking and per-port handler logic
pub struct Ports {
    cache: Box<[u8; PORTS_LEN]>,
    read: Box<[u64; PORTS_LEN / 64]>,
    written: Box<[u64; PORTS_LEN / 64]>,

    handlers: Vec<Option<Box<PortHandler + 'static>>>,
    handler_mappings: Box<[Option<NonZeroU16>; PORTS_LEN]>,
}

impl Ports {
    /// Create fresh zero-initialised unread and unwritten ports with no handlers
    pub fn new() -> Ports {
        Ports {
            cache: Box::new([0; PORTS_LEN]),
            read: Box::new([0; PORTS_LEN / 64]),
            written: Box::new([0; PORTS_LEN / 64]),
            handlers: vec![],
            handler_mappings: Box::new([None; PORTS_LEN]),
        }
    }

    /// Install the specified handler on the specified ports
    ///
    /// On success, calls [`PortHandler::init()`](trait.PortHandler.html#method.init) on the specified handler
    /// and returns a unique ID thereof
    ///
    /// Will return an error if the specified ports are already occupied,
    /// or the handler doesn't take the amount of ports specified,
    /// or too many handlers've been registered
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::vm::{PortHandlerInstallError, PortHandler, Ports};
    /// # use std::num::NonZeroU8;
    /// # #[derive(Eq, PartialEq, Debug)]
    /// struct InitableHandler(Option<u8>);
    /// impl PortHandler for InitableHandler {
    /// #   fn port_count(&self) -> NonZeroU8 { NonZeroU8::new(1).unwrap() }
    ///     fn init(&mut self, ports: &[u8]) { self.0 = Some(ports[0]); }
    /// #   fn uninit(&mut self) {}
    /// #   fn handle_read(&mut self, _: u8) -> u8 { 0 }
    /// #   fn handle_write(&mut self, _: u8, _: u8) {}
    /// }
    ///
    /// let mut ports = Ports::new();
    ///
    /// let handler_id = ports.install_handler(InitableHandler(None), &[0xA1])
    ///                       .map_err(|(_, e)| e).unwrap();
    ///
    /// assert_eq!(ports.get_handler(handler_id).and_then(|h| h.downcast_ref()),
    ///            Some(&InitableHandler(Some(0xA1))));
    /// ```
    pub fn install_handler<H: PortHandler + 'static>(&mut self, handler: H, ports: &[u8]) -> Result<usize, (H, PortHandlerInstallError)> {
        if self.handlers.len() == 0xFFFF {
            return Err((handler, PortHandlerInstallError::TooManyHandlers));
        }

        let port_count = handler.port_count().get();
        if ports.len() != port_count as usize {
            return Err((handler, PortHandlerInstallError::WrongPortCount(ports.len(), port_count)));
        }

        if let Some(taken_ports) = self.verify_ports_free(ports) {
            return Err((handler, PortHandlerInstallError::PortsTaken(taken_ports)));
        }

        Ok(self.install_handler_impl(Box::new(handler), ports))
    }

    fn verify_ports_free(&self, ports: &[u8]) -> Option<Vec<u8>> {
        let mut taken_ports = vec![];

        for &port in ports {
            if self.handler_mappings[port as usize].is_some() {
                taken_ports.push(port);
            }
        }

        if !taken_ports.is_empty() {
            Some(taken_ports)
        } else {
            None
        }
    }

    fn install_handler_impl(&mut self, handler: Box<PortHandler + 'static>, ports: &[u8]) -> usize {
        self.handlers.push(Some(handler));

        let handler_idx = self.handlers.len() - 1;
        self.handlers[handler_idx].as_mut().unwrap().init(ports);

        for &port in ports {
            self.handler_mappings[port as usize] = NonZeroU16::new(handler_idx as u16 + 1);
        }

        handler_idx
    }

    /// Get reference to the handler with the specified ID, if exists
    pub fn get_handler(&self, idx: usize) -> Option<&(PortHandler + 'static)> {
        self.handlers.get(idx).and_then(|h| h.as_ref()).map(|h| h.as_ref())
    }

    /// Get mutable reference to the handler with the specified ID, if exists
    pub fn get_handler_mut(&mut self, idx: usize) -> Option<&mut (PortHandler + 'static)> {
        self.handlers.get_mut(idx).and_then(|h| h.as_mut()).map(|h| h.as_mut())
    }

    /// Remove, unregister, and deinitialise the handler with the specified ID and return it, if exists
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::vm::{PortHandlerInstallError, PortHandler, Ports};
    /// # use std::num::NonZeroU8;
    /// # #[derive(Eq, PartialEq, Debug)]
    /// struct DeInitableHandler(Option<u8>, bool);
    /// impl PortHandler for DeInitableHandler {
    /// #   fn port_count(&self) -> NonZeroU8 { NonZeroU8::new(1).unwrap() }
    ///     fn init(&mut self, ports: &[u8]) { self.0 = Some(ports[0]); }
    ///     fn uninit(&mut self) { self.1 = true; }
    /// #   fn handle_read(&mut self, _: u8) -> u8 { 0 }
    /// #   fn handle_write(&mut self, _: u8, _: u8) {}
    /// }
    ///
    /// let mut ports = Ports::new();
    ///
    /// let handler_id = ports.install_handler(DeInitableHandler(None, false), &[0xA1])
    ///                       .map_err(|(_, e)| e).unwrap();
    /// assert_eq!(ports.uninstall_handler(handler_id).and_then(|h| h.downcast().ok()),
    ///            Some(Box::new(DeInitableHandler(Some(0xA1), true))));
    ///
    /// assert!(ports.get_handler(handler_id).is_none());
    /// ```
    pub fn uninstall_handler(&mut self, idx: usize) -> Option<Box<PortHandler + 'static>> {
        if idx >= self.handlers.len() {
            return None;
        }

        let mut handler = self.handlers[idx].take()?;
        handler.uninit();

        let mut ports_left = handler.port_count().get();
        let mapping = NonZeroU16::new(idx as u16 + 1);
        for m in &mut self.handler_mappings[..] {
            if *m == mapping {
                *m = None;

                ports_left -= 1;
                if ports_left == 0 {
                    break;
                }
            }
        }

        Some(handler)
    }

    /// Read from the specified port
    ///
    /// If a handler is installed there, delegate thereto and cache the result
    ///
    /// Marks the port read
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::vm::{PortHandlerInstallError, PortHandler, Ports};
    /// # use std::num::NonZeroU8;
    /// # #[derive(Eq, PartialEq, Debug)]
    /// struct PassthroughHandler;
    /// impl PortHandler for PassthroughHandler {
    /// #   fn port_count(&self) -> NonZeroU8 { NonZeroU8::new(1).unwrap() }
    /// #   fn init(&mut self, _: &[u8]) {}
    /// #   fn uninit(&mut self) {}
    ///     fn handle_read(&mut self, port: u8) -> u8 { port }
    /// #   fn handle_write(&mut self, _: u8, _: u8) {}
    /// }
    ///
    /// let mut ports = Ports::new();
    ///
    /// ports.install_handler(PassthroughHandler, &[0xA1]).map_err(|(_, e)| e).unwrap();
    ///
    /// assert_eq!(ports.read(0xBE), 0);
    /// assert_eq!(ports.read(0xA1), 0xA1);
    /// ```
    pub fn read(&mut self, port: u8) -> u8 {
        let index = port as usize;

        let idx = index / 64;
        let bit = index % 64;
        self.read[idx] |= 1 << bit;

        if let Some(handler_idx) = self.handler_mappings[index] {
            let new_val = self.handlers[handler_idx.get() as usize - 1].as_mut().unwrap().handle_read(port);

            self.cache[index] = new_val;
        }

        self.cache[index]
    }


    /// Write to the specified port
    ///
    /// If a handler is installed there, delegate thereto
    ///
    /// Caches the specified byte
    ///
    /// Marks the port written
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::vm::{PortHandlerInstallError, PortHandler, Ports};
    /// # use std::num::NonZeroU8;
    /// # #[derive(Eq, PartialEq, Debug)]
    /// struct StorageHandler(u8);
    /// impl PortHandler for StorageHandler {
    /// #   fn port_count(&self) -> NonZeroU8 { NonZeroU8::new(1).unwrap() }
    /// #   fn init(&mut self, _: &[u8]) {}
    /// #   fn uninit(&mut self) {}
    ///     fn handle_read(&mut self, _: u8) -> u8 { self.0 * 2 }
    ///     fn handle_write(&mut self, _: u8, data: u8) { self.0 = data; }
    /// }
    ///
    /// let mut ports = Ports::new();
    ///
    /// let handler_id = ports.install_handler(StorageHandler(3), &[0xA1])
    ///                       .map_err(|(_, e)| e).unwrap();
    ///
    /// assert_eq!(ports.read(0xA1), 6);
    /// ports.write(0xA1, 12);
    /// assert_eq!(ports.read(0xA1), 24);
    ///
    /// assert_eq!(ports.get_handler(handler_id).and_then(|h| h.downcast_ref()),
    ///            Some(&StorageHandler(12)));
    /// ```
    pub fn write(&mut self, port: u8, byte: u8) {
        let index = port as usize;

        let idx = index / 64;
        let bit = index % 64;
        self.written[idx] |= 1 << bit;

        if let Some(handler_idx) = self.handler_mappings[index] {
            self.handlers[handler_idx.get() as usize - 1].as_mut().unwrap().handle_write(port, byte);
        }

        self.cache[index] = byte;
    }

    /// Get an iterator over the read and written port cells
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::vm::Ports;
    /// let mut ports = Ports::new();
    /// let val = ports.read(0xA1);
    /// ports.write(0x4B, val);
    /// println!("{}", ports.read(0x4B));
    /// ports.write(0xEB, 0x12);
    ///
    /// // (address, value, was_read, was_written)
    /// assert_eq!(ports.iter_rw().collect::<Vec<_>>(),
    ///            &[(0x4B, 0x00, true, true),
    ///              (0xA1, 0x00, true, false),
    ///              (0xEB, 0x12, false, true)]);
    /// ```
    pub fn iter_rw(&self) -> PortsReadWrittenIterator {
        PortsReadWrittenIterator::new(&self.cache[..], &self.read[..], &self.written[..])
    }

    /// Mark all ports as unread and unwritten
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::vm::Ports;
    /// let mut ports = Ports::new();
    /// let val = ports.read(0xA1);
    /// ports.write(0x4B, val);
    /// println!("{}", ports.read(0x4B));
    /// ports.write(0xEB, 0x12);
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

impl Drop for Ports {
    fn drop(&mut self) {
        for mut handler in self.handlers.drain(..).flatten() {
            handler.uninit();
        }
    }
}

impl Index<Range<u8>> for Ports {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: Range<u8>) -> &Self::Output {
        self.cache.index(Range {
            start: index.start as usize,
            end: index.end as usize,
        })
    }
}

impl IndexMut<Range<u8>> for Ports {
    #[inline(always)]
    fn index_mut(&mut self, index: Range<u8>) -> &mut Self::Output {
        self.cache.index_mut(Range {
            start: index.start as usize,
            end: index.end as usize,
        })
    }
}

impl Index<RangeFrom<u8>> for Ports {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeFrom<u8>) -> &Self::Output {
        self.cache.index(RangeFrom { start: index.start as usize })
    }
}

impl IndexMut<RangeFrom<u8>> for Ports {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFrom<u8>) -> &mut Self::Output {
        self.cache.index_mut(RangeFrom { start: index.start as usize })
    }
}

impl Index<RangeFull> for Ports {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeFull) -> &Self::Output {
        self.cache.index(index)
    }
}

impl IndexMut<RangeFull> for Ports {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        self.cache.index_mut(index)
    }
}

impl Index<RangeInclusive<u8>> for Ports {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeInclusive<u8>) -> &Self::Output {
        let (start, end) = index.into_inner();
        self.cache.index(RangeInclusive::new(start as usize, end as usize))
    }
}

impl IndexMut<RangeInclusive<u8>> for Ports {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeInclusive<u8>) -> &mut Self::Output {
        let (start, end) = index.into_inner();
        self.cache.index_mut(RangeInclusive::new(start as usize, end as usize))
    }
}

impl Index<RangeTo<u8>> for Ports {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeTo<u8>) -> &Self::Output {
        self.cache.index(RangeTo { end: index.end as usize })
    }
}

impl IndexMut<RangeTo<u8>> for Ports {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeTo<u8>) -> &mut Self::Output {
        self.cache.index_mut(RangeTo { end: index.end as usize })
    }
}

impl Index<RangeToInclusive<u8>> for Ports {
    type Output = [u8];

    #[inline(always)]
    fn index(&self, index: RangeToInclusive<u8>) -> &Self::Output {
        self.cache.index(RangeToInclusive { end: index.end as usize })
    }
}

impl IndexMut<RangeToInclusive<u8>> for Ports {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeToInclusive<u8>) -> &mut Self::Output {
        self.cache.index_mut(RangeToInclusive { end: index.end as usize })
    }
}

impl fmt::Debug for Ports {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ports")
            .field("cache", &&self.cache[..])
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
        &self.cache[..] == other
    }
}

impl PartialEq<Ports> for Ports {
    #[inline]
    fn eq(&self, other: &Ports) -> bool {
        self.cache[..] == other.cache[..] && self.read[..] == other.read[..] && self.written[..] == other.written[..]
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
        Ord::cmp(&self.cache[..], &other.cache[..])
            .then(Ord::cmp(&self.read[..], &other.read[..]))
            .then(Ord::cmp(&self.written[..], &other.written[..]))
    }
}
