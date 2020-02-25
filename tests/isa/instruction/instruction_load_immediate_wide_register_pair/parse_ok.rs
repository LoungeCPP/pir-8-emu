use pir_8_emu::isa::instruction::InstructionLoadImmediateWideRegisterPair;
use std::convert::TryFrom;


#[test]
fn a_b() {
    assert_eq!(InstructionLoadImmediateWideRegisterPair::try_from(0b00), Ok(InstructionLoadImmediateWideRegisterPair::Ab));
}

#[test]
fn c_d() {
    assert_eq!(InstructionLoadImmediateWideRegisterPair::try_from(0b01), Ok(InstructionLoadImmediateWideRegisterPair::Cd));
}

#[test]
fn x_y() {
    assert_eq!(InstructionLoadImmediateWideRegisterPair::try_from(0b10), Ok(InstructionLoadImmediateWideRegisterPair::Xy));
}

#[test]
fn adr() {
    assert_eq!(InstructionLoadImmediateWideRegisterPair::try_from(0b11), Ok(InstructionLoadImmediateWideRegisterPair::Adr));
}
