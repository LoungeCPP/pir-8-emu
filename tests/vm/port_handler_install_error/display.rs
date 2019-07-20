use pir_8_emu::vm::PortHandlerInstallError;
use rand::{Rng, thread_rng};


#[test]
fn ports_taken() {
    for len in 1..5 {
        let mut exp = "The following ports were taken: ".to_string();
        let mut ports = vec![thread_rng().gen::<u8>()];
        exp.push_str(&format!("{:#04x}", ports[0]));
        for _ in 1..len {
            let port = thread_rng().gen::<u8>();
            ports.push(port);
            exp.push_str(&format!(", {:#04x}", port));
        }
        assert_eq!(PortHandlerInstallError::PortsTaken(ports).to_string(), exp);
    }
}

#[test]
fn wrong_port_count() {
    for p in 0..0x1FF {
        for s in 0..=0xFF {
            assert_eq!(PortHandlerInstallError::WrongPortCount(p, s).to_string(),
                       format!("Provided {} ports, handler supports {}", p, s));
        }
    }
}

#[test]
fn too_many_handlers() {
    assert_eq!(PortHandlerInstallError::TooManyHandlers.to_string(), "Too many handlers");
}
