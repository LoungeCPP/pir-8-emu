use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionJumpCondition, InstructionPortDirection,
                                  InstructionMadrDirection, InstructionStckDirection, InstructionRegisterPair, AluOperation, Instruction};
use pir_8_emu::isa::GeneralPurposeRegister;
use self::super::super::alt_gp_registers;
use std::convert::TryFrom;


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
fn load_immediate() {
    aaa("LOAD IMM", |aaa| Instruction::LoadImmediate { aaa: aaa.address() });
}

#[test]
fn load_indirect() {
    aaa("LOAD IND", |aaa| Instruction::LoadIndirect { aaa: aaa.address() });
}

#[test]
fn save() {
    aaa("SAVE", |aaa| Instruction::Save { aaa: aaa.address() });
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
        for aaa in regs {
            for bbb in regs {
                assert_eq!(Instruction::from_str(&format!("MOVE {} {}", aaa.letter(), bbb.letter()), regs),
                           Ok(Instruction::Move {
                               aaa: aaa.address(),
                               bbb: bbb.address(),
                           }));
            }
        }
    }
}

#[test]
fn port() {
    aaa("PORT IN", |aaa| {
        Instruction::Port {
            d: InstructionPortDirection::In,
            aaa: aaa.address(),
        }
    });

    aaa("PORT OUT", |aaa| {
        Instruction::Port {
            d: InstructionPortDirection::Out,
            aaa: aaa.address(),
        }
    });
}

#[test]
fn comp() {
    aaa("COMP", |aaa| Instruction::Comp { aaa: aaa.address() });
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


fn aaa(base: &str, ins: fn(&GeneralPurposeRegister) -> Instruction) {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for aaa in regs {
            assert_eq!(Instruction::from_str(&format!("{} {}", base, aaa.letter()), regs), Ok(ins(aaa)));
        }
    }
}
