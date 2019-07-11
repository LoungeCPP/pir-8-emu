use pir_8_emu::isa::instruction::AluOperation;
use std::convert::TryFrom;


mod parse_ok;
// mod display;


#[test]
fn parse_err() {
    for i in 0b0001_0000..=0b1111_1111 {
        assert_eq!(AluOperation::try_from(i), Err(()));
    }
}
