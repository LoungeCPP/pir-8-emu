use pir_8_emu::binutils::pir_8_as::{AssemblerDirectiveObeyError, AssemblerDirective};
use rand::distributions::{Alphanumeric, Distribution};
use std::collections::BTreeMap;
use rand::thread_rng;


#[test]
fn set_origin() {
    for addr in 0x10..0x21 {
        let addr = addr << 8 | addr;

        let mut next_output_address = Some(addr + 1);
        let mut labels = BTreeMap::new();

        assert_eq!(AssemblerDirective::SetOrigin(addr).obey(&mut next_output_address, &mut labels),
                   Err(AssemblerDirectiveObeyError::OutputAddressAlreadySet(addr + 1, addr)));

        assert_eq!(next_output_address, Some(addr + 1));
        assert_eq!(labels, BTreeMap::new());
    }
}

#[test]
fn save_label() {
    for token_len in 1..5 {
        for _ in 0..5 {
            let label: String = Alphanumeric.sample_iter(thread_rng()).take(token_len as usize).collect();

            let mut next_output_address = None;
            let mut labels = vec![(label.clone(), token_len * 0x1A)].into_iter().collect();

            assert_eq!(AssemblerDirective::SaveLabel(&label).obey(&mut next_output_address, &mut labels),
                       Err(AssemblerDirectiveObeyError::LabelNameTaken(&label)));

            assert_eq!(next_output_address, None);
            assert_eq!(labels, vec![(label.clone(), token_len * 0x1A)].into_iter().collect());
        }
    }
}
