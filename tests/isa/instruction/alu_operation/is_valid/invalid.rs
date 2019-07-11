use pir_8_emu::isa::instruction::AluOperation;


#[test]
fn reserved_block_0() {
    assert!(!AluOperation::Reserved(0b0011).is_valid());
}

#[test]
fn reserved_block_1() {
    assert!(!AluOperation::Reserved(0b0111).is_valid());
}
