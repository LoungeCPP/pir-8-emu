use downcast_rs::Downcast;
use std::num::NonZeroU8;


/// A handler for any number of exclusively-allocated ports
pub trait PortHandler: Downcast {
    /// Get the amount of ports this handler handles
    fn port_count(&self) -> NonZeroU8;

    /// Initialise the handler to use the specified ports
    ///
    /// When called through [`Ports::install_handler()`](struct.Ports.html#method.install_handler),
    /// the length of the specified port set will be equal to what was last returned by [`port_count()`](#method.port_count)
    fn init(&mut self, ports: &[u8]);

    /// Deinitialise the handler
    ///
    /// Called during [`Ports::uninstall_handler()`](struct.Ports.html#method.uninstall_handler) or during `Ports`' destructor
    fn uninit(&mut self);

    /// Handle reading the returned byte from the specified port
    ///
    /// When called through [`Ports::read()`](struct.Ports.html#method.read),
    /// the specified port is guaranteed to have been contained within the port set specified during the [`init()`](#method.init) call,
    /// which is guaranteed to have run before this
    fn handle_read(&mut self, port: u8) -> u8;

    /// Handle writing the specified byte to the specified port
    ///
    /// When called through [`Ports::write()`](struct.Ports.html#method.write),
    /// the specified port is guaranteed to have been contained within the port set specified during the [`init()`](#method.init) call,
    /// which is guaranteed to have run before this
    fn handle_write(&mut self, port: u8, byte: u8);
}

// `Any` is very good
impl_downcast!(PortHandler);
