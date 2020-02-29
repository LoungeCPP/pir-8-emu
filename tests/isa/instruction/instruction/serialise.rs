use pir_8_emu::isa::instruction::{InstructionLoadImmediateWideRegisterPair, AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType,
                                  InstructionJumpCondition, InstructionPortDirection, InstructionMadrDirection, InstructionStckDirection,
                                  InstructionRegisterPair, AluOperation, Instruction};


#[test]
fn load_immediate_byte() {
    single_register(0b0000_0000 | 0b0_0000, |r| Instruction::LoadImmediateByte { rrr: r });
}

#[test]
fn load_indirect() {
    single_register(0b0000_0000 | 0b0_1000, |r| Instruction::LoadIndirect { rrr: r });
}

#[test]
fn load_immediate_wide() {
    let raw: u8 = Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Ab }.into();
    assert_eq!(raw, 0b0000_0000 | 0b1_0000 | 0b00);

    let raw: u8 = Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Cd }.into();
    assert_eq!(raw, 0b0000_0000 | 0b1_0000 | 0b01);

    let raw: u8 = Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Xy }.into();
    assert_eq!(raw, 0b0000_0000 | 0b1_0000 | 0b10);

    let raw: u8 = Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Adr }.into();
    assert_eq!(raw, 0b0000_0000 | 0b1_0000 | 0b11);
}

#[test]
fn jump() {
    let raw: u8 = Instruction::Jump(InstructionJumpCondition::Jmpz).into();
    assert_eq!(raw, 0b0010_0000);

    let raw: u8 = Instruction::Jump(InstructionJumpCondition::Jmpp).into();
    assert_eq!(raw, 0b0010_0001);

    let raw: u8 = Instruction::Jump(InstructionJumpCondition::Jmpg).into();
    assert_eq!(raw, 0b0010_0010);

    let raw: u8 = Instruction::Jump(InstructionJumpCondition::Jmpc).into();
    assert_eq!(raw, 0b0010_0011);

    let raw: u8 = Instruction::Jump(InstructionJumpCondition::Jmzg).into();
    assert_eq!(raw, 0b0010_0100);

    let raw: u8 = Instruction::Jump(InstructionJumpCondition::Jmzl).into();
    assert_eq!(raw, 0b0010_0101);

    let raw: u8 = Instruction::Jump(InstructionJumpCondition::Jmpl).into();
    assert_eq!(raw, 0b0010_0110);

    let raw: u8 = Instruction::Jump(InstructionJumpCondition::Jump).into();
    assert_eq!(raw, 0b0010_0111);
}

#[test]
fn save() {
    single_register(0b0010_1000, |r| Instruction::Save { rrr: r });
}

#[test]
fn alu_valid() {
    let raw: u8 = Instruction::Alu(AluOperation::Add).into();
    assert_eq!(raw, 0b0011_0000);

    let raw: u8 = Instruction::Alu(AluOperation::Sub).into();
    assert_eq!(raw, 0b0011_0001);

    let raw: u8 = Instruction::Alu(AluOperation::AddC).into();
    assert_eq!(raw, 0b0011_0010);

    let raw: u8 = Instruction::Alu(AluOperation::SubC).into();
    assert_eq!(raw, 0b0011_0011);

    let raw: u8 = Instruction::Alu(AluOperation::Or).into();
    assert_eq!(raw, 0b0011_0100);

    let raw: u8 = Instruction::Alu(AluOperation::Xor).into();
    assert_eq!(raw, 0b0011_0101);

    let raw: u8 = Instruction::Alu(AluOperation::And).into();
    assert_eq!(raw, 0b0011_0110);

    let raw: u8 = Instruction::Alu(AluOperation::Not).into();
    assert_eq!(raw, 0b0011_0111);
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
fn move_() {
    for qqq in 0..=0b111 {
        for rrr in 0..=0b111 {
            let eraw = 0b0100_0000 | (qqq << 3) | rrr;
            let raw: u8 = Instruction::Move {
                    qqq: qqq,
                    rrr: rrr,
                }
                .into();
            assert_eq!(raw, eraw);
        }
    }
}

#[test]
fn madr() {
    let raw: u8 = Instruction::Madr {
            d: InstructionMadrDirection::Write,
            r: InstructionRegisterPair::Ab,
        }
        .into();
    assert_eq!(raw, 0b1101_1000);

    let raw: u8 = Instruction::Madr {
            d: InstructionMadrDirection::Write,
            r: InstructionRegisterPair::Cd,
        }
        .into();
    assert_eq!(raw, 0b1101_1001);

    let raw: u8 = Instruction::Madr {
            d: InstructionMadrDirection::Read,
            r: InstructionRegisterPair::Ab,
        }
        .into();
    assert_eq!(raw, 0b1101_1010);

    let raw: u8 = Instruction::Madr {
            d: InstructionMadrDirection::Read,
            r: InstructionRegisterPair::Cd,
        }
        .into();
    assert_eq!(raw, 0b1101_1011);
}

#[test]
fn port() {
    single_register(0b1110_1000, |r| {
        Instruction::Port {
            d: InstructionPortDirection::In,
            rrr: r,
        }
    });
    single_register(0b1110_0000, |r| {
        Instruction::Port {
            d: InstructionPortDirection::Out,
            rrr: r,
        }
    });
}

#[test]
fn comp() {
    single_register(0b1111_0000, |r| Instruction::Comp { rrr: r });
}

#[test]
fn stck() {
    let raw: u8 = Instruction::Stck {
            d: InstructionStckDirection::Push,
            r: InstructionRegisterPair::Ab,
        }
        .into();
    assert_eq!(raw, 0b1111_1000);

    let raw: u8 = Instruction::Stck {
            d: InstructionStckDirection::Push,
            r: InstructionRegisterPair::Cd,
        }
        .into();
    assert_eq!(raw, 0b1111_1001);

    let raw: u8 = Instruction::Stck {
            d: InstructionStckDirection::Pop,
            r: InstructionRegisterPair::Ab,
        }
        .into();
    assert_eq!(raw, 0b1111_1010);

    let raw: u8 = Instruction::Stck {
            d: InstructionStckDirection::Pop,
            r: InstructionRegisterPair::Cd,
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
    reserved_block(0b0001_0100, 0b11);
}

#[test]
fn reserved_block_1() {
    reserved_block(0b0001_1000, 0b111);
}

#[test]
fn reserved_block_2() {
    reserved_block(0b1000_0000, 0b111111);
}

#[test]
fn reserved_block_3() {
    reserved_block(0b1100_0000, 0b1111);
}

#[test]
fn reserved_block_4() {
    reserved_block(0b1101_0000, 0b111);
}

#[test]
fn reserved_block_5() {
    reserved_block(0b1101_1100, 0b11);
}

#[test]
fn reserved_block_6() {
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
