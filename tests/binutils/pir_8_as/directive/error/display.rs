use pir_8_emu::binutils::pir_8_as::AssemblerDirectiveObeyError;
use rand::distributions::{Alphanumeric, Distribution};
use rand::thread_rng;


#[test]
fn label_name_taken() {
    for token_len in 1..5 {
        for _ in 0..5 {
            let label: String = Alphanumeric.sample_iter(thread_rng()).take(token_len).collect();

            assert_eq!(AssemblerDirectiveObeyError::LabelNameTaken(&label).to_string(),
                       format!("Label name \"{}\" already used", label));
        }
    }
}

#[test]
fn output_address_already_set() {
    for cur_addr in 0..0x11 {
        let cur_addr = cur_addr << 8 | cur_addr;

        for req_addr in 0x10..0x21 {
            let req_addr = req_addr << 8 | req_addr;

            assert_eq!(AssemblerDirectiveObeyError::OutputAddressAlreadySet(cur_addr, req_addr).to_string(),
                       format!("Couldn't set origin to {:#06x}, as it was set previously or instructions were processed, \
                               and the next output address is {:#06x}",
                               req_addr,
                               cur_addr));
        }
    }
}
