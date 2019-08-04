use pir_8_emu::binutils::pir_8_emu::ExecutionConfig;

mod read_from_config_dir;
mod write_to_config_dir;


#[test]
fn new() {
    assert_eq!(ExecutionConfig::new(),
               ExecutionConfig {
                   auto_load_next_instruction: false,
                   execute_full_instructions: false,
               });
}
