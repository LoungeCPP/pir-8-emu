use pir_8_emu::isa::instruction::InstructionJumpCondition;
use std::convert::TryFrom;


mod serialise;
mod parse_ok;
mod display;


#[test]
fn parse_err() {
    for i in 0b0000_1000..=0b1111_1111 {
        assert_eq!(InstructionJumpCondition::try_from(i), Err(()));
    }
}
