use pir_8_emu::binutils::pir_8_emu::ExecutionConfig;
use std::env::temp_dir;
use std::fs;


#[test]
fn nonexistant() {
    let root = temp_dir().join("pir_8_emu-test").join("binutils-pir_8_emu-ExecutionConfig-read_from_config_dir-ok-nonexistant");

    assert_eq!(ExecutionConfig::read_from_config_dir(root.clone()).unwrap(), None);
}

#[test]
fn with_auto_load_next_instruction_with_execute_full_instructions() {
    let root = temp_dir()
        .join("pir_8_emu-test")
        .join("binutils-pir_8_emu-ExecutionConfig-read_from_config_dir-ok-with_auto_load_next_instruction_with_execute_full_instructions");
    fs::create_dir_all(&root).unwrap();

    fs::write(root.join("exec_cfg.toml"),
              r#"auto_load_next_instruction = true
                 execute_full_instructions = true
                 hewwo = "uwu"
                 mew = 123"#
                  .as_bytes())
        .unwrap();

    assert_eq!(ExecutionConfig::read_from_config_dir(root.clone()).unwrap(),
               Some(ExecutionConfig {
                   auto_load_next_instruction: true,
                   execute_full_instructions: true,
                   ..ExecutionConfig::new()
               }));
}

#[test]
fn with_auto_load_next_instruction_without_execute_full_instructions() {
    let root = temp_dir()
        .join("pir_8_emu-test")
        .join("binutils-pir_8_emu-ExecutionConfig-read_from_config_dir-ok-with_auto_load_next_instruction_without_execute_full_instructions");
    fs::create_dir_all(&root).unwrap();

    fs::write(root.join("exec_cfg.toml"),
              r#"auto_load_next_instruction = true
                 hewwo = "uwu"
                 mew = 123"#
                  .as_bytes())
        .unwrap();

    assert_eq!(ExecutionConfig::read_from_config_dir(root.clone()).unwrap(),
               Some(ExecutionConfig { auto_load_next_instruction: true, ..ExecutionConfig::new() }));
}

#[test]
fn without_auto_load_next_instruction_with_execute_full_instructions() {
    let root = temp_dir()
        .join("pir_8_emu-test")
        .join("binutils-pir_8_emu-ExecutionConfig-read_from_config_dir-ok-without_auto_load_next_instruction_with_execute_full_instructions");
    fs::create_dir_all(&root).unwrap();

    fs::write(root.join("exec_cfg.toml"),
              r#"execute_full_instructions = true
                 hewwo = "uwu""#
                  .as_bytes())
        .unwrap();

    assert_eq!(ExecutionConfig::read_from_config_dir(root.clone()).unwrap(),
               Some(ExecutionConfig { execute_full_instructions: true, ..ExecutionConfig::new() }));
}

#[test]
fn empty() {
    let root = temp_dir().join("pir_8_emu-test").join("binutils-pir_8_emu-ExecutionConfig-read_from_config_dir-ok-empty");
    fs::create_dir_all(&root).unwrap();

    fs::write(root.join("exec_cfg.toml"),
              r#"hewwo = "uwu"
                 mew = 123"#
                  .as_bytes())
        .unwrap();

    assert_eq!(ExecutionConfig::read_from_config_dir(root.clone()).unwrap(), Some(ExecutionConfig::new()));
}
