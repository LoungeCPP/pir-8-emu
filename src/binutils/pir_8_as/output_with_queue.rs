use std::collections::{BTreeMap, BTreeSet, VecDeque};
use self::super::LabelFragment;
use std::io::{self, Write};


/// Output sink which transparently waits for labels to be saved
///
/// # Examples
///
/// ```
/// # use pir_8_emu::binutils::pir_8_as::{OutputWithQueue, LabelFragment};
/// # use std::collections::BTreeMap;
/// let mut dest = vec![];
/// # let mut output = OutputWithQueue::new(unsafe { &mut *(&mut dest as *mut _) });
/// # /*
/// let mut output = OutputWithQueue::new(&mut dest);
/// # */
/// let mut labels = BTreeMap::new();
///
/// output.write_all(&[0xFE], &labels).unwrap();
/// assert_eq!(&dest, &[0xFEu8]);
///
/// output.wait_for_label("OwO".to_string(), 0, LabelFragment::Full);
/// output.write_all(&[0xFF], &labels).unwrap();
/// assert_eq!(&dest, &[0xFEu8]);
///
/// output.wait_for_label("OwO".to_string(), 0x0F, LabelFragment::Low);
/// output.write_all(&[0xFF], &labels).unwrap();
/// assert_eq!(&dest, &[0xFEu8]);
///
/// output.wait_for_label("eWe".to_string(), 0, LabelFragment::Full);
/// output.write_all(&[0x4C], &labels).unwrap();
/// assert_eq!(&dest, &[0xFEu8]);
///
/// output.wait_for_label("ЦшЦ".to_string(), 0, LabelFragment::Full);
/// output.write_all(&[0xEC], &labels).unwrap();
/// assert_eq!(&dest, &[0xFEu8]);
///
/// labels.insert("OwO".to_string(), 0x0110);
/// labels.insert("ЦшЦ".to_string(), 0x0420);
/// output.write_all(&[0xFA], &labels).unwrap();
/// assert_eq!(&dest, &[0xFEu8, 0x01, 0x10, 0xFF, 0x1F, 0xFF]);
///
/// assert_eq!(output.unfound_labels(&labels), Some(vec!["eWe".to_string()].into_iter().collect()));
/// ```
pub struct OutputWithQueue {
    phys_out: Box<dyn Write>,
    buffer: VecDeque<BufferedData>,
}

impl OutputWithQueue {
    /// Create an unqueued output, writing to the specified destination
    pub fn new<W: Write + 'static>(output: W) -> OutputWithQueue {
        OutputWithQueue::new_impl(Box::new(output))
    }

    fn new_impl(output: Box<dyn Write>) -> OutputWithQueue {
        OutputWithQueue {
            phys_out: output,
            buffer: VecDeque::new(),
        }
    }

    /// Queue all output going forward until a label with the specified name shows up, and offset it by the specified amount
    pub fn wait_for_label(&mut self, label: String, offset: i16, fragment: LabelFragment) {
        self.buffer.push_back(BufferedData::new(label, offset, fragment))
    }

    /// Write the specified bytes to the output or queue them
    ///
    /// Calls [`flush()`](#method.flush) first
    ///
    /// If afterward, the label buffer is not empty, queue the specified buffer at the end
    ///
    /// Otherwise, write the specified buffer directly to the output device
    pub fn write_all(&mut self, buf: &[u8], labels: &BTreeMap<String, u16>) -> io::Result<()> {
        self.flush(labels)?;

        match self.buffer.back_mut() {
            Some(buffered) => {
                buffered.byte_stream.extend(buf);
                Ok(())
            }
            None => self.phys_out.write_all(buf),
        }
    }

    /// Attempt to clear the label queue
    ///
    /// There's no need to call this explicitly,
    /// as [`write_all()`](#method.write_all) will call this funxion before performing any output
    ///
    /// If the label at the front of the queue is present in the specified labelset write its address,
    /// then the buffer queued behind it, and pop it off
    ///
    /// This repeats until the queue is empty or the label thereatfront doesn't exist in the labelset
    pub fn flush(&mut self, labels: &BTreeMap<String, u16>) -> io::Result<()> {
        while !self.buffer.is_empty() {
            if self.buffer[0].write_if_ready(self.phys_out.as_mut(), labels)? {
                self.buffer.pop_front();
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Get all remaining queued labels not present in the specified labelset, or `None` if all were
    pub fn unfound_labels(mut self, labels: &BTreeMap<String, u16>) -> Option<BTreeSet<String>> {
        let lbls: BTreeSet<_> = self.buffer.drain(..).map(|d| d.label).filter(|l| !labels.contains_key(l)).collect();

        if !lbls.is_empty() { Some(lbls) } else { None }
    }
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct BufferedData {
    pub label: String,
    pub offset: i16,
    pub fragment: LabelFragment,
    pub byte_stream: Vec<u8>,
}

impl BufferedData {
    pub fn new(label: String, offset: i16, fragment: LabelFragment) -> BufferedData {
        BufferedData {
            label: label,
            offset: offset,
            fragment: fragment,
            byte_stream: vec![],
        }
    }

    pub fn write_if_ready(&self, to: &mut dyn Write, labels: &BTreeMap<String, u16>) -> io::Result<bool> {
        match labels.get(&self.label) {
            Some(addr) => {
                let addr = if self.offset < 0 {
                    addr.wrapping_sub(-self.offset as u16)
                } else {
                    addr.wrapping_add(self.offset as u16)
                };
                let addr_hi = (addr >> 8) as u8;
                let addr_lo = (addr & 0b1111_1111) as u8;

                match self.fragment {
                    LabelFragment::Full => to.write_all(&[addr_hi, addr_lo])?,
                    LabelFragment::High => to.write_all(&[addr_hi])?,
                    LabelFragment::Low => to.write_all(&[addr_lo])?,
                }
                to.write_all(&self.byte_stream)?;

                Ok(true)
            }
            None => Ok(false),
        }

    }
}
