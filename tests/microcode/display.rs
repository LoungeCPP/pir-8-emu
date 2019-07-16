use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionJumpCondition, AluOperation};
use pir_8_emu::isa::{GeneralPurposeRegisterBank, GeneralPurposeRegister};
use pir_8_emu::microcode::MicroOp;


#[test]
fn nop() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::Nop.display(regs).to_string(), "Nop");
    }
}

#[test]
fn halt() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::Halt.display(regs).to_string(), "Halt");
    }
}

#[test]
fn stack_push() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::StackPush.display(regs).to_string(), "StackPush");
    }
}

#[test]
fn stack_pop() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::StackPop.display(regs).to_string(), "StackPop");
    }
}

#[test]
fn alu() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::Alu(AluOperation::Add).display(regs).to_string(), "Alu ADD");
        assert_eq!(MicroOp::Alu(AluOperation::Sub).display(regs).to_string(), "Alu SUB");
        assert_eq!(MicroOp::Alu(AluOperation::Not).display(regs).to_string(), "Alu NOT");
        assert_eq!(MicroOp::Alu(AluOperation::Or).display(regs).to_string(), "Alu OR");
        assert_eq!(MicroOp::Alu(AluOperation::Xor).display(regs).to_string(), "Alu XOR");
        assert_eq!(MicroOp::Alu(AluOperation::And).display(regs).to_string(), "Alu AND");
    }
}

#[test]
fn alu_shift_or_rotate() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for &d in &[AluOperationShiftOrRotateDirection::Right, AluOperationShiftOrRotateDirection::Left] {
            for &tt in &[AluOperationShiftOrRotateType::Lsf,
                         AluOperationShiftOrRotateType::Asf,
                         AluOperationShiftOrRotateType::Rtc,
                         AluOperationShiftOrRotateType::Rtw] {
                assert_eq!(MicroOp::Alu(AluOperation::ShiftOrRotate { d: d, tt: tt }).display(regs).to_string(),
                           format!("Alu SOR {} {}", d, tt));
            }
        }
    }
}

#[test]
fn port_in() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::PortIn.display(regs).to_string(), "PortIn");
    }
}

#[test]
fn port_out() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::PortOut.display(regs).to_string(), "PortOut");
    }
}

#[test]
fn compare() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::Compare.display(regs).to_string(), "Compare");
    }
}

#[test]
fn make_immediate() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for i in 0..0xFF {
            assert_eq!(MicroOp::MakeImmediate(i).display(regs).to_string(), format!("MakeImmediate {:#04x}", i));
        }
    }
}

#[test]
fn load_immediate() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::LoadImmediate.display(regs).to_string(), "LoadImmediate");
    }
}

#[test]
fn fetch_address() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::FetchAddress.display(regs).to_string(), "FetchAddress");
    }
}

#[test]
fn write_address() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::WriteAddress.display(regs).to_string(), "WriteAddress");
    }
}

#[test]
fn check_jump_condition() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmpz).display(regs).to_string(), "CheckJumpCondition JMPZ");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmpp).display(regs).to_string(), "CheckJumpCondition JMPP");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmpg).display(regs).to_string(), "CheckJumpCondition JMPG");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmpc).display(regs).to_string(), "CheckJumpCondition JMPC");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmzg).display(regs).to_string(), "CheckJumpCondition JMZG");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmzl).display(regs).to_string(), "CheckJumpCondition JMZL");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmpl).display(regs).to_string(), "CheckJumpCondition JMPL");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jump).display(regs).to_string(), "CheckJumpCondition JUMP");
    }
}

#[test]
fn read_register() {
    single_register("ReadRegister", MicroOp::ReadRegister);
}

#[test]
fn write_register() {
    single_register("WriteRegister", MicroOp::WriteRegister);
}


fn single_register(base: &str, op: fn(u8) -> MicroOp) {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for register in regs {
            assert_eq!(op(register.address()).display(regs).to_string(), format!("{} {}", base, register.letter()));
        }
    }
}

fn alt_gp_registers() -> GeneralPurposeRegisterBank {
    GeneralPurposeRegister::from_letters("01234567").unwrap()
}
