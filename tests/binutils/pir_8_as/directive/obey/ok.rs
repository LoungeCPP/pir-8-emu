use pir_8_emu::binutils::pir_8_as::{AssemblerDirective, LabelLoad};
use rand::distributions::{Alphanumeric, Distribution};
use std::collections::BTreeMap;
use rand::thread_rng;


#[test]
fn set_origin() {
    for addr in 0x10..0x21 {
        let addr = addr << 8 | addr;

        let mut next_output_address = None;
        let mut labels = BTreeMap::new();

        assert_eq!(AssemblerDirective::SetOrigin(addr).obey(&mut next_output_address, &mut labels), Ok(None));

        assert_eq!(next_output_address, Some(addr));
        assert_eq!(labels, BTreeMap::new());
    }
}

#[test]
fn save_label() {
    for _ in 0..5 {
        let mut labels = BTreeMap::new();
        let mut labels_v = vec![];

        for token_len in 1..5 {
            let label: String = Alphanumeric.sample_iter(thread_rng()).take(token_len as usize).collect();
            labels_v.push((label.clone(),
                           match token_len {
                               0 | 1 => 0,
                               _ => token_len - 1,
                           }));

            let mut next_output_address = match token_len {
                0 => None,
                1 => Some(0),
                _ => Some(token_len - 1),
            };

            assert_eq!(AssemblerDirective::SaveLabel(&label).obey(&mut next_output_address, &mut labels), Ok(None));

            assert_eq!(next_output_address, Some(labels_v[token_len as usize - 1].1));
            assert_eq!(labels, labels_v.clone().into_iter().collect());
        }
    }
}

#[test]
fn load_label_present() {
    for token_len in 1..5 {
        for _ in 0..5 {
            let label: String = Alphanumeric.sample_iter(thread_rng()).take(token_len as usize).collect();

            let mut labels = vec![(label.clone(), token_len * 0x1A)].into_iter().collect();
            let mut next_output_address = None;

            assert_eq!(AssemblerDirective::LoadLabel(&label).obey(&mut next_output_address, &mut labels),
                       Ok(Some(LabelLoad::HaveImmediately(token_len * 0x1A))));

            assert_eq!(next_output_address, None);
            assert_eq!(labels, vec![(label, token_len * 0x1A)].into_iter().collect());
        }
    }
}

#[test]
fn load_label_missing() {
    for token_len in 1..5 {
        for _ in 0..5 {
            let label: String = Alphanumeric.sample_iter(thread_rng()).take(token_len as usize).collect();

            let mut labels = BTreeMap::new();
            let mut next_output_address = None;

            assert_eq!(AssemblerDirective::LoadLabel(&label).obey(&mut next_output_address, &mut labels),
                       Ok(Some(LabelLoad::WaitFor(label.clone()))));

            assert_eq!(next_output_address, None);
            assert_eq!(labels, BTreeMap::new());
        }
    }
}
