use pir_8_emu::isa::instruction::InstructionLoadImmediateWideRegisterPair;


#[test]
fn a_b() {
    assert_eq!(InstructionLoadImmediateWideRegisterPair::Ab.to_string(), "A&B");
}

#[test]
fn c_d() {
    assert_eq!(InstructionLoadImmediateWideRegisterPair::Cd.to_string(), "C&D");
}

#[test]
fn x_y() {
    assert_eq!(InstructionLoadImmediateWideRegisterPair::Xy.to_string(), "X&Y");
}

#[test]
fn adr() {
    assert_eq!(InstructionLoadImmediateWideRegisterPair::Adr.to_string(), "ADR");
}
