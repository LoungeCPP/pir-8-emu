use pir_8_emu::isa::instruction::AluOperationShiftOrRotateType;
use std::convert::TryFrom;


mod serialise;
mod parse_ok;
mod display;


#[test]
fn parse_err() {
    for i in 0b0000_0100..=0b1111_1111 {
        assert_eq!(AluOperationShiftOrRotateType::try_from(i), Err(()));
    }
}
