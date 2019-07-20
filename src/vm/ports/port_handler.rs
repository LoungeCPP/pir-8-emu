use downcast_rs::Downcast;
use std::num::NonZeroU8;


pub trait PortHandler: Downcast {
    fn port_count(&self) -> NonZeroU8;
    fn init(&mut self, ports: &[u8]);

    fn handle_read(&mut self, port: u8) -> u8;
    fn handle_write(&mut self, port: u8, byte: u8);
}

// `Any` is very good
impl_downcast!(PortHandler);
