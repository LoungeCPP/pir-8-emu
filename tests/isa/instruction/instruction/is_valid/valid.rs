use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionStckRegisterPair, InstructionJumpCondition,
                                  InstructionStckDirection, AluOperation, Instruction};


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
        assert!(Instruction::Jump(cond).is_valid());
    }
}

#[test]
fn load_immediate() {
    single_register(|r| Instruction::LoadImmediate { aaa: r })
}

#[test]
fn load_indirect() {
    single_register(|r| Instruction::LoadIndirect { aaa: r })
}

#[test]
fn save() {
    single_register(|r| Instruction::Save { aaa: r })
}

#[test]
fn alu() {
    assert!(Instruction::Alu(AluOperation::Add).is_valid());
    assert!(Instruction::Alu(AluOperation::Sub).is_valid());
    assert!(Instruction::Alu(AluOperation::Not).is_valid());
    assert!(Instruction::Alu(AluOperation::Or).is_valid());
    assert!(Instruction::Alu(AluOperation::Xor).is_valid());
    assert!(Instruction::Alu(AluOperation::And).is_valid());
}

#[test]
fn alu_shift_or_rotate() {
    for &d in &[AluOperationShiftOrRotateDirection::Right, AluOperationShiftOrRotateDirection::Left] {
        for &tt in &[AluOperationShiftOrRotateType::Lsf,
                     AluOperationShiftOrRotateType::Asf,
                     AluOperationShiftOrRotateType::Rtc,
                     AluOperationShiftOrRotateType::Rtw] {
            assert!(Instruction::Alu(AluOperation::ShiftOrRotate { d: d, tt: tt }).is_valid());
        }
    }
}

#[test]
fn move_() {
    for aaa in 0..=0b111 {
        for bbb in 0..=0b111 {
            assert!(Instruction::Move {
                    aaa: aaa,
                    bbb: bbb,
                }
                .is_valid());
        }
    }
}

#[test]
fn comp() {
    single_register(|r| Instruction::Comp { aaa: r })
}

#[test]
fn stck() {
    for &d in &[InstructionStckDirection::Push, InstructionStckDirection::Pop] {
        for &r in &[InstructionStckRegisterPair::Ab, InstructionStckRegisterPair::Cd] {
            assert!(Instruction::Stck { d: d, r: r }.is_valid());
        }
    }
}

#[test]
fn clrf() {
    assert!(Instruction::Clrf.is_valid());
}

#[test]
fn halt() {
    assert!(Instruction::Halt.is_valid());
}


fn single_register(instr: fn(u8) -> Instruction) {
    for register in 0..=0b111 {
        assert!(instr(register).is_valid());
    }
}
