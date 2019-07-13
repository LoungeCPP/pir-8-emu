use pir_8_emu::isa::GeneralPurposeRegister;

mod data_length;
mod serialise;
mod from_str;
mod is_valid;
mod display;
mod parse;


fn alt_gp_registers() -> [GeneralPurposeRegister; 8] {
    [GeneralPurposeRegister::new(0b000, '0').expect("0"),
     GeneralPurposeRegister::new(0b001, '1').expect("1"),
     GeneralPurposeRegister::new(0b010, '2').expect("2"),
     GeneralPurposeRegister::new(0b011, '3').expect("3"),
     GeneralPurposeRegister::new(0b100, '4').expect("4"),
     GeneralPurposeRegister::new(0b101, '5').expect("5"),
     GeneralPurposeRegister::new(0b110, '6').expect("6"),
     GeneralPurposeRegister::new(0b111, '7').expect("7")]
}
