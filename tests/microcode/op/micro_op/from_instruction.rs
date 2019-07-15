use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, InstructionStckRegisterPair, InstructionJumpCondition,
                                  InstructionPortDirection, InstructionStckDirection, AluOperation, Instruction};
use pir_8_emu::microcode::MicroOp;


const FLAG_REGISTER_ADDRESS: u8 = 0b000;
const S_REGISTER_ADDRESS: u8 = 0b001;
const X_REGISTER_ADDRESS: u8 = 0b010;
const Y_REGISTER_ADDRESS: u8 = 0b011;
const A_REGISTER_ADDRESS: u8 = 0b100;
const B_REGISTER_ADDRESS: u8 = 0b101;
const C_REGISTER_ADDRESS: u8 = 0b110;
const D_REGISTER_ADDRESS: u8 = 0b111;


#[test]
fn reserved_block_0() {
    reserved_block(0b0000_0000, 0b1111, Instruction::Reserved);
}

#[test]
fn reserved_block_1() {
    reserved_block(0b1000_0000, 0b11_1111, Instruction::Reserved);
}

#[test]
fn reserved_block_2() {
    reserved_block(0b1100_0000, 0b1_1111, Instruction::Reserved);
}

#[test]
fn reserved_block_3() {
    reserved_block(0b1110_0000, 0b1111, Instruction::Reserved);
}

#[test]
fn reserved_block_4() {
    reserved_block(0b1111_1100, 0b1, Instruction::Reserved);
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
        let ops = MicroOp::from_instruction(Instruction::Jump(cond));
        let ops = &ops.0[..ops.1];

        assert_eq!(ops,
                   &[MicroOp::LoadImmediate,
                     MicroOp::LoadImmediate,
                     MicroOp::ReadRegister(FLAG_REGISTER_ADDRESS),
                     MicroOp::CheckJumpCondition(cond),
                     MicroOp::Jump]);
    }
}

#[test]
fn load_immediate() {
    single_register(|aaa| Instruction::LoadImmediate { aaa: aaa },
                    |aaa| vec![MicroOp::LoadImmediate, MicroOp::WriteRegister(aaa)]);
}

#[test]
fn load_indirect() {
    single_register(|aaa| Instruction::LoadIndirect { aaa: aaa },
                    |aaa| vec![MicroOp::LoadImmediate, MicroOp::LoadImmediate, MicroOp::FetchAddress, MicroOp::WriteRegister(aaa)]);
}

#[test]
fn save() {
    single_register(|aaa| Instruction::Save { aaa: aaa },
                    |aaa| vec![MicroOp::ReadRegister(aaa), MicroOp::LoadImmediate, MicroOp::LoadImmediate, MicroOp::WriteAddress]);
}

#[test]
fn alu_reserved_block_0() {
    reserved_block(0b0011, 0, |o| Instruction::Alu(AluOperation::Reserved(o)));
}

#[test]
fn alu_reserved_block_1() {
    reserved_block(0b0111, 0, |o| Instruction::Alu(AluOperation::Reserved(o)));
}

#[test]
fn alu() {
    for &op in &[AluOperation::Add, AluOperation::Sub, AluOperation::Not, AluOperation::Or, AluOperation::Xor, AluOperation::And] {
        alu_impl(op);
    }
}

#[test]
fn alu_sor() {
    for &d in &[AluOperationShiftOrRotateDirection::Left, AluOperationShiftOrRotateDirection::Right] {
        for &tt in &[AluOperationShiftOrRotateType::Lsf,
                     AluOperationShiftOrRotateType::Asf,
                     AluOperationShiftOrRotateType::Rtc,
                     AluOperationShiftOrRotateType::Rtw] {
            alu_impl(AluOperation::ShiftOrRotate { d: d, tt: tt });
        }
    }
}

#[test]
fn move_() {
    for aaa in 0..0b111 {
        for bbb in 0..0b111 {
            let ops = MicroOp::from_instruction(Instruction::Move {
                aaa: aaa,
                bbb: bbb,
            });
            let ops = &ops.0[..ops.1];

            assert_eq!(ops, &[MicroOp::ReadRegister(aaa), MicroOp::WriteRegister(bbb)]);
        }
    }
}

#[test]
fn port_in() {
    single_register(|aaa| {
                        Instruction::Port {
                            d: InstructionPortDirection::In,
                            aaa: aaa,
                        }
                    },
                    |aaa| vec![MicroOp::ReadRegister(A_REGISTER_ADDRESS), MicroOp::PortIn, MicroOp::WriteRegister(aaa)]);
}

#[test]
fn port_out() {
    single_register(|aaa| {
                        Instruction::Port {
                            d: InstructionPortDirection::Out,
                            aaa: aaa,
                        }
                    },
                    |aaa| vec![MicroOp::ReadRegister(aaa), MicroOp::ReadRegister(A_REGISTER_ADDRESS), MicroOp::PortOut]);
}

#[test]
fn comp() {
    single_register(|aaa| Instruction::Comp { aaa: aaa }, |aaa| {
        vec![MicroOp::ReadRegister(S_REGISTER_ADDRESS),
             MicroOp::ReadRegister(aaa),
             MicroOp::ReadRegister(FLAG_REGISTER_ADDRESS),
             MicroOp::Compare,
             MicroOp::WriteRegister(FLAG_REGISTER_ADDRESS)]
    });
}

#[test]
fn stck_push() {
    let ops = MicroOp::from_instruction(Instruction::Stck {
        d: InstructionStckDirection::Push,
        r: InstructionStckRegisterPair::Ab,
    });
    let ops = &ops.0[..ops.1];

    assert_eq!(ops,
               &[MicroOp::ReadRegister(A_REGISTER_ADDRESS), MicroOp::StackPush, MicroOp::ReadRegister(B_REGISTER_ADDRESS), MicroOp::StackPush]);


    let ops = MicroOp::from_instruction(Instruction::Stck {
        d: InstructionStckDirection::Push,
        r: InstructionStckRegisterPair::Cd,
    });
    let ops = &ops.0[..ops.1];

    assert_eq!(ops,
               &[MicroOp::ReadRegister(C_REGISTER_ADDRESS), MicroOp::StackPush, MicroOp::ReadRegister(D_REGISTER_ADDRESS), MicroOp::StackPush]);
}

#[test]
fn stck_pop() {
    let ops = MicroOp::from_instruction(Instruction::Stck {
        d: InstructionStckDirection::Pop,
        r: InstructionStckRegisterPair::Ab,
    });
    let ops = &ops.0[..ops.1];

    assert_eq!(ops,
               &[MicroOp::StackPop, MicroOp::WriteRegister(B_REGISTER_ADDRESS), MicroOp::StackPop, MicroOp::WriteRegister(A_REGISTER_ADDRESS)]);


    let ops = MicroOp::from_instruction(Instruction::Stck {
        d: InstructionStckDirection::Pop,
        r: InstructionStckRegisterPair::Cd,
    });
    let ops = &ops.0[..ops.1];

    assert_eq!(ops,
               &[MicroOp::StackPop, MicroOp::WriteRegister(D_REGISTER_ADDRESS), MicroOp::StackPop, MicroOp::WriteRegister(C_REGISTER_ADDRESS)]);
}

#[test]
fn clrf() {
    let ops = MicroOp::from_instruction(Instruction::Clrf);
    let ops = &ops.0[..ops.1];

    assert_eq!(ops, &[MicroOp::MakeImmediate(0), MicroOp::WriteRegister(FLAG_REGISTER_ADDRESS)]);
}

#[test]
fn halt() {
    let ops = MicroOp::from_instruction(Instruction::Halt);
    let ops = &ops.0[..ops.1];

    assert_eq!(ops, &[MicroOp::Halt]);
}


fn single_register(instr: fn(u8) -> Instruction, wops: fn(u8) -> Vec<MicroOp>) {
    for aaa in 0..=0b111 {
        let ops = MicroOp::from_instruction(instr(aaa));
        let ops = &ops.0[..ops.1];

        assert_eq!(ops, &wops(aaa)[..]);
    }
}

fn reserved_block(base: u8, max: u8, instr: fn(u8) -> Instruction) {
    for i in 0..max {
        let raw = base | i;
        assert_eq!(instr(raw).data_length(), 0);
    }
}

fn alu_impl(op: AluOperation) {
    let ops = MicroOp::from_instruction(Instruction::Alu(op));
    let ops = &ops.0[..ops.1];

    assert_eq!(ops,
               &[MicroOp::ReadRegister(X_REGISTER_ADDRESS),
                 MicroOp::ReadRegister(Y_REGISTER_ADDRESS),
                 MicroOp::ReadRegister(FLAG_REGISTER_ADDRESS),
                 MicroOp::Alu(op),
                 MicroOp::WriteRegister(FLAG_REGISTER_ADDRESS),
                 MicroOp::WriteRegister(S_REGISTER_ADDRESS)]);
}
