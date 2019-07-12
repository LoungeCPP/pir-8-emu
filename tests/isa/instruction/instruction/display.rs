use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionStckRegisterPair, InstructionStckDirection,
                                  AluOperation, Instruction};
use pir_8_emu::isa::{GeneralPurposeRegister, default_general_purpose_registers};


#[test]
fn jump() {
    single_register("JUMP", |r| Instruction::Jump { xxx: r })
}

#[test]
fn load_immediate() {
    single_register("LOAD IMM", |r| Instruction::LoadImmediate { aaa: r })
}

#[test]
fn load_indirect() {
    single_register("LOAD IND", |r| Instruction::LoadIndirect { aaa: r })
}

#[test]
fn save() {
    single_register("SAVE", |r| Instruction::Save { aaa: r })
}

#[test]
fn alu_valid() {
    assert_eq!(Instruction::from(0b0011_0000), Instruction::Alu(AluOperation::Add));
    assert_eq!(Instruction::from(0b0011_0001), Instruction::Alu(AluOperation::Sub));
    assert_eq!(Instruction::from(0b0011_0010), Instruction::Alu(AluOperation::Not));
    assert_eq!(Instruction::from(0b0011_0100), Instruction::Alu(AluOperation::Or));
    assert_eq!(Instruction::from(0b0011_0101), Instruction::Alu(AluOperation::Xor));
    assert_eq!(Instruction::from(0b0011_0110), Instruction::Alu(AluOperation::And));
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
    for regs in &[default_general_purpose_registers(), alt_gp_registers()] {
        for aaa in regs {
            for bbb in regs {
                assert_eq!(Instruction::Move {
                                   aaa: aaa.address(),
                                   bbb: bbb.address(),
                               }
                               .display(regs)
                               .to_string(),
                           format!("MOVE {} {}", aaa.letter(), bbb.letter()));
            }
        }
    }
}

#[test]
fn comp() {
    single_register("COMP", |r| Instruction::Comp { aaa: r })
}

#[test]
fn stck() {
    for regs in &[default_general_purpose_registers(), alt_gp_registers()] {
        assert_eq!(Instruction::Stck {
                           d: InstructionStckDirection::Push,
                           r: InstructionStckRegisterPair::Ab,
                       }
                       .display(regs)
                       .to_string(),
                   "STCK PUSH A&B");

        assert_eq!(Instruction::Stck {
                           d: InstructionStckDirection::Push,
                           r: InstructionStckRegisterPair::Cd,
                       }
                       .display(regs)
                       .to_string(),
                   "STCK PUSH C&D");

        assert_eq!(Instruction::Stck {
                           d: InstructionStckDirection::Pop,
                           r: InstructionStckRegisterPair::Ab,
                       }
                       .display(regs)
                       .to_string(),
                   "STCK POP A&B");

        assert_eq!(Instruction::Stck {
                           d: InstructionStckDirection::Pop,
                           r: InstructionStckRegisterPair::Cd,
                       }
                       .display(regs)
                       .to_string(),
                   "STCK POP C&D");
    }
}

#[test]
fn clrf() {
    for regs in &[default_general_purpose_registers(), alt_gp_registers()] {
        assert_eq!(Instruction::Clrf.display(regs).to_string(), "CLRF");
    }
}

#[test]
fn halt() {
    for regs in &[default_general_purpose_registers(), alt_gp_registers()] {
        assert_eq!(Instruction::Halt.display(regs).to_string(), "HALT");
    }
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


fn alt_gp_registers() -> [GeneralPurposeRegister; 8] {
    [GeneralPurposeRegister::new(0b000, '0').expect("0"),
     GeneralPurposeRegister::new(0b001, '1').expect("1"),
     GeneralPurposeRegister::new(0b010, '2').expect("2"),
     GeneralPurposeRegister::new(0b011, '3').expect("3"),
     GeneralPurposeRegister::new(0b100, '4').expect("4"),
     GeneralPurposeRegister::new(0b101, '5').expect("5"),
     GeneralPurposeRegister::new(0b110, '6').expect("6"),
     GeneralPurposeRegister::new(0b111, '7').expect("7")]
}

fn single_register(base: &str, instr: fn(u8) -> Instruction) {
    for regs in &[default_general_purpose_registers(), alt_gp_registers()] {
        for register in regs {
            assert_eq!(instr(register.address()).display(regs).to_string(), format!("{} {}", base, register.letter()));
        }
    }
}

fn reserved_block(base: u8, max: u8) {
    for regs in &[default_general_purpose_registers(), alt_gp_registers()] {
        for i in 0..=max {
            let raw = base | i;
            assert_eq!(Instruction::Reserved(raw).display(regs).to_string(),
                       format!("0b{:04b}_{:04b}", (raw & 0b1111_0000) >> 4, raw & 0b0000_1111));
        }
    }
}
