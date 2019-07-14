use pir_8_emu::isa::GeneralPurposeRegister;

mod data_length;
mod serialise;
mod from_str;
mod is_valid;
mod display;
mod parse;


fn alt_gp_registers() -> [GeneralPurposeRegister; 8] {
    GeneralPurposeRegister::from_letters("01234567").unwrap()
}
