use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionStckRegisterPair, InstructionStckDirection,
                                  AluOperation, Instruction};


#[test]
fn jump() {
    single_register(0b0001_0000, |r| Instruction::Jump { xxx: r })
}

#[test]
fn load_immediate() {
    single_register(0b0001_1000, |r| Instruction::LoadImmediate { aaa: r })
}

#[test]
fn load_indirect() {
    single_register(0b0010_0000, |r| Instruction::LoadIndirect { aaa: r })
}

#[test]
fn save() {
    single_register(0b0010_1000, |r| Instruction::Save { aaa: r })
}

#[test]
fn alu_valid() {
    let raw: u8 = Instruction::Alu(AluOperation::Add).into();
    assert_eq!(raw, 0b0011_0000);

    let raw: u8 = Instruction::Alu(AluOperation::Sub).into();
    assert_eq!(raw, 0b0011_0001);

    let raw: u8 = Instruction::Alu(AluOperation::Not).into();
    assert_eq!(raw, 0b0011_0010);

    let raw: u8 = Instruction::Alu(AluOperation::Or).into();
    assert_eq!(raw, 0b0011_0100);

    let raw: u8 = Instruction::Alu(AluOperation::Xor).into();
    assert_eq!(raw, 0b0011_0101);

    let raw: u8 = Instruction::Alu(AluOperation::And).into();
    assert_eq!(raw, 0b0011_0110);
}

#[test]
fn alu_valid_shift_or_rotate() {
    for &(dir, d) in &[(0b0000, AluOperationShiftOrRotateDirection::Right), (0b0100, AluOperationShiftOrRotateDirection::Left)] {
        assert_eq!(Instruction::from(0b0011_1000 | dir),
                   Instruction::Alu(AluOperation::ShiftOrRotate {
                       d: d,
                       tt: AluOperationShiftOrRotateType::Lsf,
                   }));
        assert_eq!(Instruction::from(0b0011_1001 | dir),
                   Instruction::Alu(AluOperation::ShiftOrRotate {
                       d: d,
                       tt: AluOperationShiftOrRotateType::Asf,
                   }));
        assert_eq!(Instruction::from(0b0011_1010 | dir),
                   Instruction::Alu(AluOperation::ShiftOrRotate {
                       d: d,
                       tt: AluOperationShiftOrRotateType::Rtc,
                   }));
        assert_eq!(Instruction::from(0b0011_1011 | dir),
                   Instruction::Alu(AluOperation::ShiftOrRotate {
                       d: d,
                       tt: AluOperationShiftOrRotateType::Rtw,
                   }));
    }
}

#[test]
fn alu_reserved() {
    assert_eq!(Instruction::from(0b0011_0011), Instruction::Alu(AluOperation::Reserved(0b0011)));
    assert_eq!(Instruction::from(0b0011_0111), Instruction::Alu(AluOperation::Reserved(0b0111)));
}

#[test]
fn move_() {
    for aaa in 0..=0b111 {
        for bbb in 0..=0b111 {
            let eraw = 0b0100_0000 | (aaa << 3) | bbb;
            let raw: u8 = Instruction::Move {
                    aaa: aaa,
                    bbb: bbb,
                }
                .into();
            assert_eq!(raw, eraw);
        }
    }
}

#[test]
fn comp() {
    single_register(0b1111_0000, |r| Instruction::Comp { aaa: r })
}

#[test]
fn stck() {
    let raw: u8 = Instruction::Stck {
            d: InstructionStckDirection::Push,
            r: InstructionStckRegisterPair::Ab,
        }
        .into();
    assert_eq!(raw, 0b1111_1000);

    let raw: u8 = Instruction::Stck {
            d: InstructionStckDirection::Push,
            r: InstructionStckRegisterPair::Cd,
        }
        .into();
    assert_eq!(raw, 0b1111_1001);

    let raw: u8 = Instruction::Stck {
            d: InstructionStckDirection::Pop,
            r: InstructionStckRegisterPair::Ab,
        }
        .into();
    assert_eq!(raw, 0b1111_1010);

    let raw: u8 = Instruction::Stck {
            d: InstructionStckDirection::Pop,
            r: InstructionStckRegisterPair::Cd,
        }
        .into();
    assert_eq!(raw, 0b1111_1011);
}

#[test]
fn clrf() {
    let raw: u8 = Instruction::Clrf.into();
    assert_eq!(raw, 0b1111_1110);
}

#[test]
fn halt() {
    let raw: u8 = Instruction::Halt.into();
    assert_eq!(raw, 0b1111_1111);
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


fn single_register(base: u8, wanted: fn(u8) -> Instruction) {
    for register in 0..=0b111 {
        let eraw = base | register;
        let raw: u8 = wanted(register).into();
        assert_eq!(raw, eraw);
    }
}

fn reserved_block(base: u8, max: u8) {
    for i in 0..=max {
        let eraw = base | i;
        let raw: u8 = Instruction::Reserved(eraw).into();
        assert_eq!(raw, eraw);
    }
}
