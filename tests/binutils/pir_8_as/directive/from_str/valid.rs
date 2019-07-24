use pir_8_emu::binutils::pir_8_as::AssemblerDirective;
use rand::distributions::{Alphanumeric, Distribution};
use rand::thread_rng;


#[test]
fn set_origin() {
    for addr in 0..=0xFF {
        let addr = addr << 8 | addr;

        assert_eq!(AssemblerDirective::from_str(&format!(":origin {}", addr)),
                   Ok(Some(AssemblerDirective::SetOrigin(addr))));
        assert_eq!(AssemblerDirective::from_str(&format!(":origin {:#06x}", addr)),
                   Ok(Some(AssemblerDirective::SetOrigin(addr))));
        assert_eq!(AssemblerDirective::from_str(&format!(":origin {:#09x}", addr)),
                   Ok(Some(AssemblerDirective::SetOrigin(addr))));
        assert_eq!(AssemblerDirective::from_str(&format!(":origin {:#018b}", addr)),
                   Ok(Some(AssemblerDirective::SetOrigin(addr))));
    }
}

#[test]
fn save_label() {
    label("save", "", |l| AssemblerDirective::SaveLabel(l));
}

#[test]
fn load_label() {
    label("load", "", |l| AssemblerDirective::LoadLabel(l, 0));
}

#[test]
fn load_label_offset() {
    for i in -0x10..0x10 {
        label("load-offset", &format!("{}", i), |l| AssemblerDirective::LoadLabel(l, i));
        label("load-offset", &format!("{:#x}", i), |l| AssemblerDirective::LoadLabel(l, i));
    }
}


fn label<F: Fn(&str) -> AssemblerDirective<'_>>(op: &str, suff: &str, dir: F) {
    for pad_lleft in 0..5 {
        for pad_left in 0..5 {
            for pad_center in 1..5 {
                for pad_right in 1..5 {
                    for pad_rright in 1..5 {
                        for token_len in 1..5 {
                            for _ in 0..5 {
                                let label: String = Alphanumeric.sample_iter(thread_rng()).take(token_len).collect();

                                let dir_str = format!("{e:wll$}:{e:wl$}label{e:wc$}{}{e:wr$}{}{e:wrr$}{}",
                                                      op,
                                                      label,
                                                      suff,
                                                      e = "",
                                                      wll = pad_lleft,
                                                      wl = pad_left,
                                                      wc = pad_center,
                                                      wr = pad_right,
                                                      wrr = pad_rright);

                                assert_eq!(AssemblerDirective::from_str(&dir_str), Ok(Some(dir(&label))), "{}", dir_str);
                            }
                        }
                    }
                }
            }
        }
    }
}
