use pir_8_emu::vm::{PortHandlerInstallError, PortHandler, Ports};
use std::num::NonZeroU8;


static PORTS: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

#[derive(Debug, PartialEq, Eq)]
struct Handler(u8);

impl PortHandler for Handler {
    fn port_count(&self) -> NonZeroU8 {
        NonZeroU8::new(self.0).unwrap()
    }

    fn init(&mut self, _: &[u8]) {}

    fn handle_read(&mut self, _: u8) -> u8 {
        0
    }

    fn handle_write(&mut self, _: u8, _: u8) {}
}


#[test]
fn wrong_port_count() {
    let mut ports = Ports::new();

    for pc in 1..10 {
        for p in 0..10 {
            if p as u8 == pc {
                continue;
            }

            assert_eq!(ports.install_handler(Handler(pc), &PORTS[..p]),
                       Err((Handler(pc), PortHandlerInstallError::WrongPortCount(p, pc))));
        }
    }
}

#[test]
fn ports_taken() {
    let mut ports = Ports::new();
    ports.install_handler(Handler(10), PORTS).unwrap();

    assert_eq!(ports.install_handler(Handler(10), PORTS),
               Err((Handler(10), PortHandlerInstallError::PortsTaken(PORTS.to_vec()))));
}

#[test]
fn too_many_handlers() {
    let mut ports = Ports::new();
    for i in 0..0xFFFF {
        let id = ports.install_handler(Handler(1), &[(i % 0xFF) as u8]).unwrap();
        ports.uninstall_handler(id).unwrap();
    }

    assert_eq!(ports.install_handler(Handler(1), &[0]),
               Err((Handler(1), PortHandlerInstallError::TooManyHandlers)));
}
