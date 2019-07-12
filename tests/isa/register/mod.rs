use pir_8_emu::isa;

mod general;
mod special;


#[test]
fn default_general_purpose_registers() {
    assert_eq!(isa::default_general_purpose_registers(),
               [isa::GeneralPurposeRegister::new(0b000, 'F').unwrap(),
                isa::GeneralPurposeRegister::new(0b001, 'S').unwrap(),
                isa::GeneralPurposeRegister::new(0b010, 'X').unwrap(),
                isa::GeneralPurposeRegister::new(0b011, 'Y').unwrap(),
                isa::GeneralPurposeRegister::new(0b100, 'A').unwrap(),
                isa::GeneralPurposeRegister::new(0b101, 'B').unwrap(),
                isa::GeneralPurposeRegister::new(0b110, 'C').unwrap(),
                isa::GeneralPurposeRegister::new(0b111, 'D').unwrap()]);
}
