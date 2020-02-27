use pir_8_emu::isa::instruction::{InstructionLoadImmediateWideRegisterPair, AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType,
                                  InstructionJumpCondition, InstructionPortDirection, InstructionMadrDirection, InstructionStckDirection,
                                  InstructionRegisterPair, AluOperation, Instruction};
use pir_8_emu::isa::GeneralPurposeRegister;
use self::super::super::alt_gp_registers;
use std::convert::TryFrom;


#[test]
fn load_immediate_byte() {
    rrr("LOAD IMM BYTE", |rrr| Instruction::LoadImmediateByte { rrr: rrr.address() });
}

#[test]
fn load_indirect() {
    rrr("LOAD IND", |rrr| Instruction::LoadIndirect { rrr: rrr.address() });
}

#[test]
fn load_immediate_wide() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::from_str("LOAD IMM WIDE A&B", regs),
                   Ok(Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Ab }));

        assert_eq!(Instruction::from_str("LOAD IMM WIDE C&D", regs),
                   Ok(Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Cd }));

        assert_eq!(Instruction::from_str("LOAD IMM WIDE X&Y", regs),
                   Ok(Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Xy }));

        assert_eq!(Instruction::from_str("LOAD IMM WIDE ADR", regs),
                   Ok(Instruction::LoadImmediateWide { rr: InstructionLoadImmediateWideRegisterPair::Adr }));
    }
}

#[test]
fn raw() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for i in 0..=0b1111_1111 {
            assert_eq!(Instruction::from_str(&format!("{}", i), regs), Ok(Instruction::from(i as u8)));
            assert_eq!(Instruction::from_str(&format!("{:#0x}", i), regs), Ok(Instruction::from(i as u8)));
            assert_eq!(Instruction::from_str(&format!("{:#0X}", i), regs), Ok(Instruction::from(i as u8)));
            assert_eq!(Instruction::from_str(&format!("{:#0o}", i), regs), Ok(Instruction::from(i as u8)));
            assert_eq!(Instruction::from_str(&format!("{:#0b}", i), regs), Ok(Instruction::from(i as u8)));
        }
    }
}

#[test]
fn jump() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::from_str("JMPZ", regs), Ok(Instruction::Jump(InstructionJumpCondition::Jmpz)));
        assert_eq!(Instruction::from_str("JMPP", regs), Ok(Instruction::Jump(InstructionJumpCondition::Jmpp)));
        assert_eq!(Instruction::from_str("JMPG", regs), Ok(Instruction::Jump(InstructionJumpCondition::Jmpg)));
        assert_eq!(Instruction::from_str("JMPC", regs), Ok(Instruction::Jump(InstructionJumpCondition::Jmpc)));
        assert_eq!(Instruction::from_str("JMZG", regs), Ok(Instruction::Jump(InstructionJumpCondition::Jmzg)));
        assert_eq!(Instruction::from_str("JMZL", regs), Ok(Instruction::Jump(InstructionJumpCondition::Jmzl)));
        assert_eq!(Instruction::from_str("JMPL", regs), Ok(Instruction::Jump(InstructionJumpCondition::Jmpl)));
        assert_eq!(Instruction::from_str("JUMP", regs), Ok(Instruction::Jump(InstructionJumpCondition::Jump)));
    }
}

#[test]
fn save() {
    rrr("SAVE", |rrr| Instruction::Save { rrr: rrr.address() });
}

#[test]
fn alu_raw() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for i in 0..=0b1111 {
            assert_eq!(Instruction::from_str(&format!("ALU {}", i), regs),
                       Ok(Instruction::Alu(AluOperation::try_from(i as u8).unwrap())));

            assert_eq!(Instruction::from_str(&format!("ALU {:#0x}", i), regs),
                       Ok(Instruction::Alu(AluOperation::try_from(i as u8).unwrap())));

            assert_eq!(Instruction::from_str(&format!("ALU {:#0X}", i), regs),
                       Ok(Instruction::Alu(AluOperation::try_from(i as u8).unwrap())));

            assert_eq!(Instruction::from_str(&format!("ALU {:#0o}", i), regs),
                       Ok(Instruction::Alu(AluOperation::try_from(i as u8).unwrap())));

            assert_eq!(Instruction::from_str(&format!("ALU {:#0b}", i), regs),
                       Ok(Instruction::Alu(AluOperation::try_from(i as u8).unwrap())));
        }
    }
}

#[test]
fn alu() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::from_str("ALU ADD", regs), Ok(Instruction::Alu(AluOperation::Add)));
        assert_eq!(Instruction::from_str("ALU SUB", regs), Ok(Instruction::Alu(AluOperation::Sub)));
        assert_eq!(Instruction::from_str("ALU ADDC", regs), Ok(Instruction::Alu(AluOperation::AddC)));
        assert_eq!(Instruction::from_str("ALU SUBC", regs), Ok(Instruction::Alu(AluOperation::SubC)));
        assert_eq!(Instruction::from_str("ALU OR", regs), Ok(Instruction::Alu(AluOperation::Or)));
        assert_eq!(Instruction::from_str("ALU XOR", regs), Ok(Instruction::Alu(AluOperation::Xor)));
        assert_eq!(Instruction::from_str("ALU AND", regs), Ok(Instruction::Alu(AluOperation::And)));
        assert_eq!(Instruction::from_str("ALU NOT", regs), Ok(Instruction::Alu(AluOperation::Not)));

        assert_eq!(Instruction::from_str("ALU SOR LEFT LSF", regs),
                   Ok(Instruction::Alu(AluOperation::ShiftOrRotate {
                       d: AluOperationShiftOrRotateDirection::Left,
                       tt: AluOperationShiftOrRotateType::Lsf,
                   })));
        assert_eq!(Instruction::from_str("ALU SOR LEFT ASF", regs),
                   Ok(Instruction::Alu(AluOperation::ShiftOrRotate {
                       d: AluOperationShiftOrRotateDirection::Left,
                       tt: AluOperationShiftOrRotateType::Asf,
                   })));
        assert_eq!(Instruction::from_str("ALU SOR LEFT RTC", regs),
                   Ok(Instruction::Alu(AluOperation::ShiftOrRotate {
                       d: AluOperationShiftOrRotateDirection::Left,
                       tt: AluOperationShiftOrRotateType::Rtc,
                   })));
        assert_eq!(Instruction::from_str("ALU SOR LEFT RTW", regs),
                   Ok(Instruction::Alu(AluOperation::ShiftOrRotate {
                       d: AluOperationShiftOrRotateDirection::Left,
                       tt: AluOperationShiftOrRotateType::Rtw,
                   })));
        assert_eq!(Instruction::from_str("ALU SOR RIGHT LSF", regs),
                   Ok(Instruction::Alu(AluOperation::ShiftOrRotate {
                       d: AluOperationShiftOrRotateDirection::Right,
                       tt: AluOperationShiftOrRotateType::Lsf,
                   })));
        assert_eq!(Instruction::from_str("ALU SOR RIGHT ASF", regs),
                   Ok(Instruction::Alu(AluOperation::ShiftOrRotate {
                       d: AluOperationShiftOrRotateDirection::Right,
                       tt: AluOperationShiftOrRotateType::Asf,
                   })));
        assert_eq!(Instruction::from_str("ALU SOR RIGHT RTC", regs),
                   Ok(Instruction::Alu(AluOperation::ShiftOrRotate {
                       d: AluOperationShiftOrRotateDirection::Right,
                       tt: AluOperationShiftOrRotateType::Rtc,
                   })));
        assert_eq!(Instruction::from_str("ALU SOR RIGHT RTW", regs),
                   Ok(Instruction::Alu(AluOperation::ShiftOrRotate {
                       d: AluOperationShiftOrRotateDirection::Right,
                       tt: AluOperationShiftOrRotateType::Rtw,
                   })));
    }
}

#[test]
fn move_() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for qqq in regs {
            for rrr in regs {
                assert_eq!(Instruction::from_str(&format!("MOVE {} {}", qqq.letter(), rrr.letter()), regs),
                           Ok(Instruction::Move {
                               qqq: qqq.address(),
                               rrr: rrr.address(),
                           }));
            }
        }
    }
}

#[test]
fn madr() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::from_str("MADR WRITE A&B", regs),
                   Ok(Instruction::Madr {
                       d: InstructionMadrDirection::Write,
                       r: InstructionRegisterPair::Ab,
                   }));

        assert_eq!(Instruction::from_str("MADR WRITE C&D", regs),
                   Ok(Instruction::Madr {
                       d: InstructionMadrDirection::Write,
                       r: InstructionRegisterPair::Cd,
                   }));

        assert_eq!(Instruction::from_str("MADR READ A&B", regs),
                   Ok(Instruction::Madr {
                       d: InstructionMadrDirection::Read,
                       r: InstructionRegisterPair::Ab,
                   }));

        assert_eq!(Instruction::from_str("MADR READ C&D", regs),
                   Ok(Instruction::Madr {
                       d: InstructionMadrDirection::Read,
                       r: InstructionRegisterPair::Cd,
                   }));
    }
}

#[test]
fn port() {
    rrr("PORT IN", |rrr| {
        Instruction::Port {
            d: InstructionPortDirection::In,
            rrr: rrr.address(),
        }
    });

    rrr("PORT OUT", |rrr| {
        Instruction::Port {
            d: InstructionPortDirection::Out,
            rrr: rrr.address(),
        }
    });
}

#[test]
fn comp() {
    rrr("COMP", |rrr| Instruction::Comp { rrr: rrr.address() });
}

#[test]
fn stck() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::from_str("STCK PUSH A&B", regs),
                   Ok(Instruction::Stck {
                       d: InstructionStckDirection::Push,
                       r: InstructionRegisterPair::Ab,
                   }));

        assert_eq!(Instruction::from_str("STCK PUSH C&D", regs),
                   Ok(Instruction::Stck {
                       d: InstructionStckDirection::Push,
                       r: InstructionRegisterPair::Cd,
                   }));

        assert_eq!(Instruction::from_str("STCK POP A&B", regs),
                   Ok(Instruction::Stck {
                       d: InstructionStckDirection::Pop,
                       r: InstructionRegisterPair::Ab,
                   }));

        assert_eq!(Instruction::from_str("STCK POP C&D", regs),
                   Ok(Instruction::Stck {
                       d: InstructionStckDirection::Pop,
                       r: InstructionRegisterPair::Cd,
                   }));
    }
}

#[test]
fn clrf() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::from_str("CLRF", regs), Ok(Instruction::Clrf));
    }
}

#[test]
fn halt() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(Instruction::from_str("HALT", regs), Ok(Instruction::Halt));
    }
}


fn rrr(base: &str, ins: fn(&GeneralPurposeRegister) -> Instruction) {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for rrr in regs {
            assert_eq!(Instruction::from_str(&format!("{} {}", base, rrr.letter()), regs), Ok(ins(rrr)));
        }
    }
}
