use pir_8_emu::isa::instruction::{AluOperation, Instruction};
use std::convert::TryFrom;


#[test]
fn alu_reserved() {
    for &op in &[0b0011, 0b0111] {
        let parsed = AluOperation::try_from(op).unwrap();

        assert_eq!(parsed, AluOperation::Reserved(op));
        assert!(!Instruction::Alu(parsed).is_valid());
    }
}


#[test]
fn reserved_block_0() {
    reserved_block(0b0000_0000, 0b111);
}

#[test]
fn reserved_block_1() {
    reserved_block(0b0000_0100, 0b11);
}

#[test]
fn reserved_block_2() {
    reserved_block(0b1000_0000, 0b11_1111);
}

#[test]
fn reserved_block_3() {
    reserved_block(0b1100_0000, 0b1_1111);
}

#[test]
fn reserved_block_4() {
    reserved_block(0b1111_1100, 0b1);
}

fn reserved_block(base: u8, max: u8) {
    for i in 0..=max {
        let raw = base | i;
        let parsed = Instruction::from(raw);

        assert_eq!(parsed, Instruction::Reserved(raw));
        assert!(!parsed.is_valid());
    }
}
