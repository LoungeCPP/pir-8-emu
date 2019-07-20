use downcast_rs::Downcast;
use std::num::NonZeroU8;


pub trait PortHandler: Downcast {
    fn port_count(&self) -> NonZeroU8;
    fn init(&mut self, ports: &[u8]);

    // fn handle_read(&mut self) -> u8;
    // fn handle_write(&mut self, byte: u8);

    fn clone(&self) -> Box<PortHandler>;
}

// `Any` is very good
impl_downcast!(PortHandler);

impl Clone for Box<PortHandler> {
    fn clone(&self) -> Box<PortHandler> {
        self.as_ref().clone()
    }
}
