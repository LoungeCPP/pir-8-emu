use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionJumpCondition, InstructionPortDirection,
                                  InstructionMadrDirection, InstructionStckDirection, InstructionRegisterPair, AluOperation, Instruction};


#[test]
fn madr() {
    for &d in &[InstructionMadrDirection::Write, InstructionMadrDirection::Read] {
        for &r in &[InstructionRegisterPair::Ab, InstructionRegisterPair::Cd] {
            assert_eq!(Instruction::Madr { d: d, r: r }.data_length(), 0);
        }
    }
}

#[test]
fn jump() {
    for &cond in &[InstructionJumpCondition::Jmpz,
                   InstructionJumpCondition::Jmpp,
                   InstructionJumpCondition::Jmpg,
                   InstructionJumpCondition::Jmpc,
                   InstructionJumpCondition::Jmzg,
                   InstructionJumpCondition::Jmzl,
                   InstructionJumpCondition::Jmpl,
                   InstructionJumpCondition::Jump] {
        assert_eq!(Instruction::Jump(cond).data_length(), 0);
    }
}

#[test]
fn load_immediate() {
    single_register(1, |r| Instruction::LoadImmediate { aaa: r });
}

#[test]
fn load_indirect() {
    single_register(0, |r| Instruction::LoadIndirect { aaa: r });
}

#[test]
fn save() {
    single_register(0, |r| Instruction::Save { aaa: r });
}

#[test]
fn alu_valid() {
    assert_eq!(Instruction::Alu(AluOperation::Add).data_length(), 0);
    assert_eq!(Instruction::Alu(AluOperation::Sub).data_length(), 0);
    assert_eq!(Instruction::Alu(AluOperation::Not).data_length(), 0);
    assert_eq!(Instruction::Alu(AluOperation::Or).data_length(), 0);
    assert_eq!(Instruction::Alu(AluOperation::Xor).data_length(), 0);
    assert_eq!(Instruction::Alu(AluOperation::And).data_length(), 0);
}

#[test]
fn alu_valid_shift_or_rotate() {
    for &d in &[AluOperationShiftOrRotateDirection::Right, AluOperationShiftOrRotateDirection::Left] {
        for &tt in &[AluOperationShiftOrRotateType::Lsf,
                     AluOperationShiftOrRotateType::Asf,
                     AluOperationShiftOrRotateType::Rtc,
                     AluOperationShiftOrRotateType::Rtw] {
            assert_eq!(Instruction::Alu(AluOperation::ShiftOrRotate { d: d, tt: tt }).data_length(), 0);
        }
    }
}

#[test]
fn alu_reserved() {
    assert_eq!(Instruction::Alu(AluOperation::Reserved(0b0011)).data_length(), 0);
    assert_eq!(Instruction::Alu(AluOperation::Reserved(0b0111)).data_length(), 0);
}

#[test]
fn move_() {
    for aaa in 0..=0b111 {
        for bbb in 0..=0b111 {
            assert_eq!(Instruction::Move {
                               aaa: aaa,
                               bbb: bbb,
                           }
                           .data_length(),
                       0);
        }
    }
}

#[test]
fn port() {
    single_register(0, |r| {
        Instruction::Port {
            d: InstructionPortDirection::In,
            aaa: r,
        }
    });
    single_register(0, |r| {
        Instruction::Port {
            d: InstructionPortDirection::Out,
            aaa: r,
        }
    });
}

#[test]
fn comp() {
    single_register(0, |r| Instruction::Comp { aaa: r });
}

#[test]
fn stck() {
    for &d in &[InstructionStckDirection::Push, InstructionStckDirection::Pop] {
        for &r in &[InstructionRegisterPair::Ab, InstructionRegisterPair::Cd] {
            assert_eq!(Instruction::Stck { d: d, r: r }.data_length(), 0);
        }
    }
}

#[test]
fn clrf() {
    assert_eq!(Instruction::Clrf.data_length(), 0);
}

#[test]
fn halt() {
    assert_eq!(Instruction::Halt.data_length(), 0);
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


fn single_register(exp: usize, instr: fn(u8) -> Instruction) {
    for register in 0..=0b111 {
        assert_eq!(instr(register).data_length(), exp);
    }
}

fn reserved_block(base: u8, max: u8) {
    for i in 0..=max {
        let raw = base | i;
        assert_eq!(Instruction::Reserved(raw).data_length(), 0);
    }
}
