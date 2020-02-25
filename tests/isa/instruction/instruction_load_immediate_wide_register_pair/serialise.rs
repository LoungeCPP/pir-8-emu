use pir_8_emu::isa::instruction::InstructionLoadImmediateWideRegisterPair;


#[test]
fn a_b() {
    assert_eq!(InstructionLoadImmediateWideRegisterPair::Ab as u8, 0b00);
}

#[test]
fn c_d() {
    assert_eq!(InstructionLoadImmediateWideRegisterPair::Cd as u8, 0b01);
}

#[test]
fn x_y() {
    assert_eq!(InstructionLoadImmediateWideRegisterPair::Xy as u8, 0b10);
}

#[test]
fn adr() {
    assert_eq!(InstructionLoadImmediateWideRegisterPair::Adr as u8, 0b11);
}
