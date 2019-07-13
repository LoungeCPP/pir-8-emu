use pir_8_emu::isa::instruction::ParseInstructionError;

mod unrecognised_register_letter;
mod missing_register_letter;
mod unrecognised_token;
mod invalid_character;
mod too_many_tokens;
mod missing_token;


#[test]
fn empty_string() {
    assert_eq!(ParseInstructionError::EmptyString.to_string(), "No tokens");
}
