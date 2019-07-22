use pir_8_emu::binutils::pir_8_as::OutputWithQueue;
use std::collections::BTreeMap;


#[test]
fn wait_for_label_write_no() {
    let mut dest = vec![];
    let mut output = OutputWithQueue::new(unsafe { &mut *(&mut dest as *mut _) });

    output.wait_for_label("owo".to_string());
    output.write_all(&['U' as u8, 'w' as u8, 'U' as u8], &BTreeMap::new()).unwrap();
    output.wait_for_label("eWe".to_string());
    output.write_all(&['O' as u8, 'm' as u8, 'O' as u8], &BTreeMap::new()).unwrap();

    assert_eq!(output.unfound_labels(&BTreeMap::new()), Some(vec!["owo".to_string(), "eWe".to_string()]));
    assert_eq!(&dest, &[]);
}

#[test]
fn wait_for_label_write_yes() {
    let mut dest = vec![];
    let mut output = OutputWithQueue::new(unsafe { &mut *(&mut dest as *mut _) });

    let labels = vec![("owo".to_string(), 0x0110)].into_iter().collect();

    output.wait_for_label("owo".to_string());
    output.write_all(&['U' as u8, 'w' as u8, 'U' as u8], &labels).unwrap();
    output.wait_for_label("eWe".to_string());
    output.write_all(&['O' as u8, 'm' as u8, 'O' as u8], &labels).unwrap();

    assert_eq!(output.unfound_labels(&labels), Some(vec!["eWe".to_string()]));
    assert_eq!(&dest, &[0x01, 0x10, 'U' as u8, 'w' as u8, 'U' as u8]);
}

#[test]
fn flush_no() {
    let mut dest = vec![];
    let mut output = OutputWithQueue::new(unsafe { &mut *(&mut dest as *mut _) });

    output.wait_for_label("owo".to_string());
    output.wait_for_label("eWe".to_string());
    output.flush(&BTreeMap::new()).unwrap();

    assert_eq!(output.unfound_labels(&BTreeMap::new()), Some(vec!["owo".to_string(), "eWe".to_string()]));
    assert_eq!(&dest, &[]);
}

#[test]
fn flush_yes() {
    let mut dest = vec![];
    let mut output = OutputWithQueue::new(unsafe { &mut *(&mut dest as *mut _) });

    let labels = vec![("owo".to_string(), 0x0110)].into_iter().collect();

    output.wait_for_label("owo".to_string());
    output.wait_for_label("eWe".to_string());
    output.flush(&labels).unwrap();

    assert_eq!(output.unfound_labels(&labels), Some(vec!["eWe".to_string()]));
    assert_eq!(&dest, &[0x01u8, 0x10]);
}
