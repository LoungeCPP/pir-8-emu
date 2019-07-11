use pir_8_emu::isa::instruction::{AluOperation, Instruction};


#[test]
fn alu_reserved() {
    assert!(!Instruction::Alu(AluOperation::Reserved(0b0011)).is_valid());
    assert!(!Instruction::Alu(AluOperation::Reserved(0b0111)).is_valid());
}


#[test]
fn reserved_block_0() {
    reserved_block(0b0000_0000, 0b1111);
}

#[test]
fn reserved_block_1() {
    reserved_block(0b1000_0000, 0b11_1111);
}

#[test]
fn reserved_block_2() {
    reserved_block(0b1100_0000, 0b1_1111);
}

#[test]
fn reserved_block_3() {
    reserved_block(0b1110_0000, 0b1111);
}

#[test]
fn reserved_block_4() {
    reserved_block(0b1111_1100, 0b1);
}

fn reserved_block(base: u8, max: u8) {
    for i in 0..=max {
        let raw = base | i;
        assert!(!Instruction::Reserved(raw).is_valid());
    }
}
