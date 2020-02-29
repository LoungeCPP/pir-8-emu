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
fn load_indirect_wide() {
    assert_eq!(Instruction::from(0b0000_0000 | 0b1_0000 | 0b00),
               Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Ab });

    assert_eq!(Instruction::from(0b0000_0000 | 0b1_0000 | 0b01),
               Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Cd });

    assert_eq!(Instruction::from(0b0000_0000 | 0b1_0000 | 0b10),
               Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Xy });

    assert_eq!(Instruction::from(0b0000_0000 | 0b1_0000 | 0b11),
               Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Adr });
}

#[test]
fn jump() {
    assert_eq!(Instruction::from(0b0010_0000), Instruction::Jump(InstructionJumpCondition::Jmpz));
    assert_eq!(Instruction::from(0b0010_0001), Instruction::Jump(InstructionJumpCondition::Jmpp));
    assert_eq!(Instruction::from(0b0010_0010), Instruction::Jump(InstructionJumpCondition::Jmpg));
    assert_eq!(Instruction::from(0b0010_0011), Instruction::Jump(InstructionJumpCondition::Jmpc));
    assert_eq!(Instruction::from(0b0010_0100), Instruction::Jump(InstructionJumpCondition::Jmzg));
    assert_eq!(Instruction::from(0b0010_0101), Instruction::Jump(InstructionJumpCondition::Jmzl));
    assert_eq!(Instruction::from(0b0010_0110), Instruction::Jump(InstructionJumpCondition::Jmpl));
    assert_eq!(Instruction::from(0b0010_0111), Instruction::Jump(InstructionJumpCondition::Jump));
}

#[test]
fn save() {
    single_register(0b0010_1000, |r| Instruction::Save { rrr: r });
}

#[test]
fn alu_valid() {
    assert_eq!(Instruction::from(0b0011_0000), Instruction::Alu(AluOperation::Add));
    assert_eq!(Instruction::from(0b0011_0001), Instruction::Alu(AluOperation::Sub));
    assert_eq!(Instruction::from(0b0011_0010), Instruction::Alu(AluOperation::AddC));
    assert_eq!(Instruction::from(0b0011_0011), Instruction::Alu(AluOperation::SubC));
    assert_eq!(Instruction::from(0b0011_0100), Instruction::Alu(AluOperation::Or));
    assert_eq!(Instruction::from(0b0011_0101), Instruction::Alu(AluOperation::Xor));
    assert_eq!(Instruction::from(0b0011_0110), Instruction::Alu(AluOperation::And));
    assert_eq!(Instruction::from(0b0011_0111), Instruction::Alu(AluOperation::Not));
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
            let raw = 0b0100_0000 | (qqq << 3) | rrr;
            assert_eq!(Instruction::from(raw),
                       Instruction::Move {
                           qqq: qqq,
                           rrr: rrr,
                       });
        }
    }
}

#[test]
fn madr() {
    assert_eq!(Instruction::from(0b1101_1000),
               Instruction::Madr {
                   d: InstructionMadrDirection::Write,
                   r: InstructionRegisterPair::Ab,
               });

    assert_eq!(Instruction::from(0b1101_1001),
               Instruction::Madr {
                   d: InstructionMadrDirection::Write,
                   r: InstructionRegisterPair::Cd,
               });

    assert_eq!(Instruction::from(0b1101_1010),
               Instruction::Madr {
                   d: InstructionMadrDirection::Read,
                   r: InstructionRegisterPair::Ab,
               });

    assert_eq!(Instruction::from(0b1101_1011),
               Instruction::Madr {
                   d: InstructionMadrDirection::Read,
                   r: InstructionRegisterPair::Cd,
               });
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
    assert_eq!(Instruction::from(0b1111_1000),
               Instruction::Stck {
                   d: InstructionStckDirection::Push,
                   r: InstructionRegisterPair::Ab,
               });

    assert_eq!(Instruction::from(0b1111_1001),
               Instruction::Stck {
                   d: InstructionStckDirection::Push,
                   r: InstructionRegisterPair::Cd,
               });

    assert_eq!(Instruction::from(0b1111_1010),
               Instruction::Stck {
                   d: InstructionStckDirection::Pop,
                   r: InstructionRegisterPair::Ab,
               });

    assert_eq!(Instruction::from(0b1111_1011),
               Instruction::Stck {
                   d: InstructionStckDirection::Pop,
                   r: InstructionRegisterPair::Cd,
               });
}

#[test]
fn clrf() {
    assert_eq!(Instruction::from(0b1111_1110), Instruction::Clrf);
}

#[test]
fn halt() {
    assert_eq!(Instruction::from(0b1111_1111), Instruction::Halt);
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
        let raw = base | register;
        assert_eq!(Instruction::from(raw), wanted(register));
    }
}

fn reserved_block(base: u8, max: u8) {
    for i in 0..=max {
        let raw = base | i;
        assert_eq!(Instruction::from(raw), Instruction::Reserved(raw));
    }
}
