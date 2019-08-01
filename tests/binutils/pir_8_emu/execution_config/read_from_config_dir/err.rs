use pir_8_emu::binutils::pir_8_emu::ExecutionConfig;
use std::env::temp_dir;
use std::fs;


#[test]
fn io_utf8() {
    let root = temp_dir().join("pir_8_emu-test").join("binutils-pir_8_emu-ExecutionConfig-read_from_config_dir-err-io_utf8");
    fs::create_dir_all(&root).unwrap();

    // Stolen from https://stackoverflow.com/a/3886015/2851815
    for bytes in &[&[0xc3, 0x28][..],
                   &[0xa0, 0xa1][..],
                   &[0xe2, 0x28, 0xa1][..],
                   &[0xe2, 0x82, 0x28][..],
                   &[0xf0, 0x28, 0x8c, 0xbc][..],
                   &[0xf0, 0x90, 0x28, 0xbc][..],
                   &[0xf0, 0x28, 0x8c, 0x28][..]] {
        fs::write(root.join("exec_cfg.toml"), bytes).unwrap();
        assert_eq!(ExecutionConfig::read_from_config_dir(root.clone()).unwrap_err().unwrap().to_string(),
                   "stream did not contain valid UTF-8");
    }
}

#[test]
fn toml() {
    let root = temp_dir().join("pir_8_emu-test").join("binutils-pir_8_emu-ExecutionConfig-read_from_config_dir-err-toml");
    fs::create_dir_all(&root).unwrap();

    fs::write(root.join("exec_cfg.toml"), b"uwu").unwrap();

    assert_eq!(ExecutionConfig::read_from_config_dir(root.clone()).unwrap_err().unwrap_err().to_string(),
               "expected an equals, found eof at line 1");
}
