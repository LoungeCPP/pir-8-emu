use pir_8_emu::binutils::pir_8_emu::ExecutionConfig;
use std::env::temp_dir;
use std::fs;

mod err;


#[test]
fn ok() {
    let root = temp_dir().join("pir_8_emu-test").join("binutils-pir_8_emu-ExecutionConfig-write_to_config_dir-ok");
    ExecutionConfig::new().write_to_config_dir(&root).unwrap();

    assert_eq!(fs::read_to_string(root.join("exec_cfg.toml")).unwrap(),
               "auto_load_next_instruction = false\n\
                execute_full_instructions = false\n\
                general_purpose_register_letters = [\n\
                \x20   'F',\n\
                \x20   'S',\n\
                \x20   'X',\n\
                \x20   'Y',\n\
                \x20   'A',\n\
                \x20   'B',\n\
                \x20   'C',\n\
                \x20   'D',\n\
                ]\n");
}
