use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionJumpCondition, AluOperation};
use pir_8_emu::isa::{GeneralPurposeRegisterBank, GeneralPurposeRegister};
use pir_8_emu::micro::MicroOp;


#[test]
fn nop() {
    constant(MicroOp::Nop, "Nop");
}

#[test]
fn halt() {
    constant(MicroOp::Halt, "Halt");
}

#[test]
fn load_instruction() {
    constant(MicroOp::LoadInstruction, "LoadInstruction");
}

#[test]
fn stack_push() {
    constant(MicroOp::StackPush, "StackPush");
}

#[test]
fn stack_pop() {
    constant(MicroOp::StackPop, "StackPop");
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
    constant(MicroOp::PortIn, "PortIn");
}

#[test]
fn port_out() {
    constant(MicroOp::PortOut, "PortOut");
}

#[test]
fn compare() {
    constant(MicroOp::Compare, "Compare");
}

#[test]
fn make_immediate() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        for i in 0..=0xFF {
            assert_eq!(MicroOp::MakeImmediate(i).display(regs).to_string(), format!("MakeImmediate {:#04x}", i));
        }
    }
}

#[test]
fn load_immediate() {
    constant(MicroOp::LoadImmediate, "LoadImmediate");
}

#[test]
fn fetch_address() {
    constant(MicroOp::FetchAddress, "FetchAddress");
}

#[test]
fn write_address() {
    constant(MicroOp::WriteAddress, "WriteAddress");
}

#[test]
fn check_jump_condition() {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmpz).display(regs).to_string(),
                   "CheckJumpCondition JMPZ");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmpp).display(regs).to_string(),
                   "CheckJumpCondition JMPP");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmpg).display(regs).to_string(),
                   "CheckJumpCondition JMPG");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmpc).display(regs).to_string(),
                   "CheckJumpCondition JMPC");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmzg).display(regs).to_string(),
                   "CheckJumpCondition JMZG");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmzl).display(regs).to_string(),
                   "CheckJumpCondition JMZL");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jmpl).display(regs).to_string(),
                   "CheckJumpCondition JMPL");
        assert_eq!(MicroOp::CheckJumpCondition(InstructionJumpCondition::Jump).display(regs).to_string(),
                   "CheckJumpCondition JUMP");
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

fn constant(op: MicroOp, exp: &str) {
    for regs in &[GeneralPurposeRegister::defaults(), alt_gp_registers()] {
        assert_eq!(op.display(regs).to_string(), exp);
    }
}

fn alt_gp_registers() -> GeneralPurposeRegisterBank {
    GeneralPurposeRegister::from_letters("01234567").unwrap()
}
