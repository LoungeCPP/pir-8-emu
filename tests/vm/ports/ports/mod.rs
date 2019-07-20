use std::sync::atomic::{AtomicBool, Ordering};
use pir_8_emu::vm::{PortHandler, Ports};
use std::num::NonZeroU8;

mod install_handler;


static UNINITED: AtomicBool = AtomicBool::new(false);


#[derive(Debug, PartialEq, Eq)]
struct UninitHandler;

impl PortHandler for UninitHandler {
    fn port_count(&self) -> NonZeroU8 {
        NonZeroU8::new(1).unwrap()
    }

    fn init(&mut self, _: &[u8]) {}

    fn uninit(&mut self) {
        UNINITED.store(true, Ordering::Relaxed);
    }

    fn handle_read(&mut self, _: u8) -> u8 {
        0
    }

    fn handle_write(&mut self, _: u8, _: u8) {}
}


#[test]
fn drop() {
    assert!(!UNINITED.load(Ordering::Relaxed));

    {
        let mut ports = Ports::new();
        ports.install_handler(UninitHandler, &[0xA1]).unwrap();
    }

    assert!(UNINITED.load(Ordering::Relaxed));
}
