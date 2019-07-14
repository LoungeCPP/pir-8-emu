use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionStckRegisterPair, InstructionJumpCondition,
                                  InstructionPortDirection, InstructionStckDirection, AluOperation, Instruction};
use pir_8_emu::isa::GeneralPurposeRegister;
use self::super::alt_gp_registers;


#[test]
fn jump() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::Jump(InstructionJumpCondition::Jmpz).display(regs).to_string(), "JMPZ");
        assert_eq!(Instruction::Jump(InstructionJumpCondition::Jmpp).display(regs).to_string(), "JMPP");
        assert_eq!(Instruction::Jump(InstructionJumpCondition::Jmpg).display(regs).to_string(), "JMPG");
        assert_eq!(Instruction::Jump(InstructionJumpCondition::Jmpc).display(regs).to_string(), "JMPC");
        assert_eq!(Instruction::Jump(InstructionJumpCondition::Jmzg).display(regs).to_string(), "JMZG");
        assert_eq!(Instruction::Jump(InstructionJumpCondition::Jmzl).display(regs).to_string(), "JMZL");
        assert_eq!(Instruction::Jump(InstructionJumpCondition::Jmpl).display(regs).to_string(), "JMPL");
        assert_eq!(Instruction::Jump(InstructionJumpCondition::Jump).display(regs).to_string(), "JUMP");
    }
}

#[test]
fn load_immediate() {
    single_register("LOAD IMM", |r| Instruction::LoadImmediate { aaa: r });
}

#[test]
fn load_indirect() {
    single_register("LOAD IND", |r| Instruction::LoadIndirect { aaa: r });
}

#[test]
fn save() {
    single_register("SAVE", |r| Instruction::Save { aaa: r });
}

#[test]
fn alu_valid() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::Alu(AluOperation::Add).display(regs).to_string(), "ALU ADD");
        assert_eq!(Instruction::Alu(AluOperation::Sub).display(regs).to_string(), "ALU SUB");
        assert_eq!(Instruction::Alu(AluOperation::Not).display(regs).to_string(), "ALU NOT");
        assert_eq!(Instruction::Alu(AluOperation::Or).display(regs).to_string(), "ALU OR");
        assert_eq!(Instruction::Alu(AluOperation::Xor).display(regs).to_string(), "ALU XOR");
        assert_eq!(Instruction::Alu(AluOperation::And).display(regs).to_string(), "ALU AND");
    }
}

#[test]
fn alu_valid_shift_or_rotate() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for &d in &[AluOperationShiftOrRotateDirection::Right, AluOperationShiftOrRotateDirection::Left] {
            for &tt in &[AluOperationShiftOrRotateType::Lsf,
                         AluOperationShiftOrRotateType::Asf,
                         AluOperationShiftOrRotateType::Rtc,
                         AluOperationShiftOrRotateType::Rtw] {
                assert_eq!(Instruction::Alu(AluOperation::ShiftOrRotate { d: d, tt: tt }).display(regs).to_string(),
                           format!("ALU SOR {} {}", d, tt));
            }
        }
    }
}

#[test]
fn alu_reserved() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::Alu(AluOperation::Reserved(0b0011)).display(regs).to_string(), "ALU 0b0011");
        assert_eq!(Instruction::Alu(AluOperation::Reserved(0b0111)).display(regs).to_string(), "ALU 0b0111");
    }
}

#[test]
fn move_() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
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
fn port() {
    single_register("PORT IN", |r| Instruction::Port { d: InstructionPortDirection::In, aaa: r });
    single_register("PORT OUT", |r| Instruction::Port { d: InstructionPortDirection::Out, aaa: r });
}

#[test]
fn comp() {
    single_register("COMP", |r| Instruction::Comp { aaa: r });
}

#[test]
fn stck() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
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
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::Clrf.display(regs).to_string(), "CLRF");
    }
}

#[test]
fn halt() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
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


fn single_register(base: &str, instr: fn(u8) -> Instruction) {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for register in regs {
            assert_eq!(instr(register.address()).display(regs).to_string(), format!("{} {}", base, register.letter()));
        }
    }
}

fn reserved_block(base: u8, max: u8) {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for i in 0..=max {
            let raw = base | i;
            assert_eq!(Instruction::Reserved(raw).display(regs).to_string(),
                       format!("0b{:04b}_{:04b}", (raw & 0b1111_0000) >> 4, raw & 0b0000_1111));
        }
    }
}
