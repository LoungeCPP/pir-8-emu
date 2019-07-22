use std::collections::{BTreeMap, VecDeque};
use std::io::{self, Write};


pub struct OutputWithQueue {
    phys_out: Box<Write>,
    buffer: VecDeque<BufferedData>,
}

impl OutputWithQueue {
    pub fn new<W: Write + 'static>(output: W) -> OutputWithQueue {
        OutputWithQueue::new_impl(Box::new(output))
    }

    fn new_impl(output: Box<Write>) -> OutputWithQueue {
        OutputWithQueue {
            phys_out: output,
            buffer: VecDeque::new(),
        }
    }

    pub fn wait_for_label(&mut self, label: String) {
        self.buffer.push_back(BufferedData::new(label))
    }

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

    pub fn flush(&mut self, labels: &BTreeMap<String, u16>) -> io::Result<()> {
        while !self.buffer.is_empty() {
            if self.buffer[0].write_if_ready(&mut self.phys_out, labels)? {
                self.buffer.pop_front();
            } else {
                break;
            }
        }

        Ok(())
    }

    pub fn unfound_labels(mut self, labels: &BTreeMap<String, u16>) -> Option<Vec<String>> {
        if !self.buffer.is_empty() {
            Some(self.buffer.drain(..).map(|d| d.label).filter(|l| !labels.contains_key(l)).collect())
        } else {
            None
        }
    }
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct BufferedData {
    pub label: String,
    pub byte_stream: Vec<u8>,
}

impl BufferedData {
    pub fn new(label: String) -> BufferedData {
        BufferedData {
            label: label,
            byte_stream: vec![],
        }
    }

    pub fn write_if_ready(&self, to: &mut Box<Write>, labels: &BTreeMap<String, u16>) -> io::Result<bool> {
        match labels.get(&self.label) {
            Some(addr) => {
                to.write_all(&[(addr >> 8) as u8, (addr & 0b1111_1111) as u8])?;
                to.write_all(&self.byte_stream)?;

                Ok(true)
            }
            None => Ok(false),
        }

    }
}
