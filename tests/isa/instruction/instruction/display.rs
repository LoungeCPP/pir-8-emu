use pir_8_emu::isa::instruction::{InstructionLoadImmediateWideRegisterPair, AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType,
                                  InstructionJumpCondition, InstructionPortDirection, InstructionMadrDirection, InstructionStckDirection,
                                  InstructionRegisterPair, AluOperation, Instruction};
use pir_8_emu::isa::GeneralPurposeRegister;
use self::super::alt_gp_registers;


#[test]
fn load_immediate_byte() {
    single_register("LOAD IMM BYTE", |r| Instruction::LoadImmediateByte { rrr: r });
}

#[test]
fn load_indirect() {
    single_register("LOAD IND", |r| Instruction::LoadIndirect { rrr: r });
}

#[test]
fn load_immediate_wide() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Ab }.display(regs).to_string(),
                   "LOAD IMM WIDE A&B");
        assert_eq!(Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Cd }.display(regs).to_string(),
                   "LOAD IMM WIDE C&D");
        assert_eq!(Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Xy }.display(regs).to_string(),
                   "LOAD IMM WIDE X&Y");
        assert_eq!(Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Adr }.display(regs).to_string(),
                   "LOAD IMM WIDE ADR");
    }
}

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
fn save() {
    single_register("SAVE", |r| Instruction::Save { rrr: r });
}

#[test]
fn alu_valid() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::Alu(AluOperation::Add).display(regs).to_string(), "ALU ADD");
        assert_eq!(Instruction::Alu(AluOperation::Sub).display(regs).to_string(), "ALU SUB");
        assert_eq!(Instruction::Alu(AluOperation::AddC).display(regs).to_string(), "ALU ADDC");
        assert_eq!(Instruction::Alu(AluOperation::SubC).display(regs).to_string(), "ALU SUBC");
        assert_eq!(Instruction::Alu(AluOperation::Or).display(regs).to_string(), "ALU OR");
        assert_eq!(Instruction::Alu(AluOperation::Xor).display(regs).to_string(), "ALU XOR");
        assert_eq!(Instruction::Alu(AluOperation::And).display(regs).to_string(), "ALU AND");
        assert_eq!(Instruction::Alu(AluOperation::Not).display(regs).to_string(), "ALU NOT");
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
fn move_() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for qqq in regs {
            for rrr in regs {
                assert_eq!(Instruction::Move {
                                   qqq: qqq.address(),
                                   rrr: rrr.address(),
                               }
                               .display(regs)
                               .to_string(),
                           format!("MOVE {} {}", qqq.letter(), rrr.letter()));
            }
        }
    }
}

#[test]
fn madr() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::Madr {
                           d: InstructionMadrDirection::Write,
                           r: InstructionRegisterPair::Ab,
                       }
                       .display(regs)
                       .to_string(),
                   "MADR WRITE A&B");

        assert_eq!(Instruction::Madr {
                           d: InstructionMadrDirection::Write,
                           r: InstructionRegisterPair::Cd,
                       }
                       .display(regs)
                       .to_string(),
                   "MADR WRITE C&D");

        assert_eq!(Instruction::Madr {
                           d: InstructionMadrDirection::Read,
                           r: InstructionRegisterPair::Ab,
                       }
                       .display(regs)
                       .to_string(),
                   "MADR READ A&B");

        assert_eq!(Instruction::Madr {
                           d: InstructionMadrDirection::Read,
                           r: InstructionRegisterPair::Cd,
                       }
                       .display(regs)
                       .to_string(),
                   "MADR READ C&D");
    }
}

#[test]
fn port() {
    single_register("PORT IN", |r| {
        Instruction::Port {
            d: InstructionPortDirection::In,
            rrr: r,
        }
    });
    single_register("PORT OUT", |r| {
        Instruction::Port {
            d: InstructionPortDirection::Out,
            rrr: r,
        }
    });
}

#[test]
fn comp() {
    single_register("COMP", |r| Instruction::Comp { rrr: r });
}

#[test]
fn stck() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::Stck {
                           d: InstructionStckDirection::Push,
                           r: InstructionRegisterPair::Ab,
                       }
                       .display(regs)
                       .to_string(),
                   "STCK PUSH A&B");

        assert_eq!(Instruction::Stck {
                           d: InstructionStckDirection::Push,
                           r: InstructionRegisterPair::Cd,
                       }
                       .display(regs)
                       .to_string(),
                   "STCK PUSH C&D");

        assert_eq!(Instruction::Stck {
                           d: InstructionStckDirection::Pop,
                           r: InstructionRegisterPair::Ab,
                       }
                       .display(regs)
                       .to_string(),
                   "STCK POP A&B");

        assert_eq!(Instruction::Stck {
                           d: InstructionStckDirection::Pop,
                           r: InstructionRegisterPair::Cd,
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
