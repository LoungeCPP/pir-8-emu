use pir_8_emu::binutils::pir_8_emu::ExecutionConfig;
use std::env::temp_dir;
use std::fs;


#[test]
fn dir() {
    let root = temp_dir().join("pir_8_emu-test").join("binutils-pir_8_emu-ExecutionConfig-write_to_config_dir-err-io_dir");
    fs::create_dir_all(root.join("exec_cfg.toml")).unwrap();

    assert_eq!(ExecutionConfig::new().write_to_config_dir(root).unwrap_err().to_string(),
               "Access is denied. (os error 5)");
}
