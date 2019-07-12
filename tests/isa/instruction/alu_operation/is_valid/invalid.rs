use pir_8_emu::isa::instruction::AluOperation;
use std::convert::TryFrom;


#[test]
fn reserved_block_0() {
    reserved_block(0b0011);
}

#[test]
fn reserved_block_1() {
    reserved_block(0b0111);
}

fn reserved_block(val: u8) {
    let parsed = AluOperation::try_from(val).unwrap();

    assert_eq!(parsed, AluOperation::Reserved(val));
    assert!(!parsed.is_valid());
}
