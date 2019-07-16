//! https://github.com/TheCatPlusPlus/pir8/blob/3f126ba3dd1201e42e8928d74a20c151b9a97823/PIR8.ISA.Tests/Shifts.cs


use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, AluOperation};
use pir_8_emu::microcode::{MicrocodeExecutionError, MicroOp};
use self::super::super::universe;


#[test]
fn logic_shift_left() {
    logic_arithmetic_shift_left(AluOperationShiftOrRotateType::Lsf);
}

#[test]
fn logic_shift_right() {
    for &test_value in &[// forcebreak
                         0b0110_0001,
                         0b0001_1110,
                         0b0110_0000,
                         0b0000_0001] {
        check_shift(test_value,
                    AluOperation::ShiftOrRotate {
                        d: AluOperationShiftOrRotateDirection::Right,
                        tt: AluOperationShiftOrRotateType::Lsf,
                    });
    }
}

#[test]
fn arithmetic_shift_left() {
    logic_arithmetic_shift_left(AluOperationShiftOrRotateType::Asf);
}

#[test]
fn arithmetic_shift_right() {
    for &test_value in &[// forcebreak
                         0b1110_0001,
                         0b0001_1110,
                         0b1110_0000,
                         0b0000_0001] {
        check_shift(test_value,
                    AluOperation::ShiftOrRotate {
                        d: AluOperationShiftOrRotateDirection::Right,
                        tt: AluOperationShiftOrRotateType::Asf,
                    });
    }
}

#[test]
fn rotate_carry_left() {
    for &test_value in &[// forcebreak
                         0b1000_0111,
                         0b0111_1001,
                         0b1000_0001,
                         0b0000_0111,
                         0b1000_0110,
                         0b0111_1000,
                         0b1000_0000,
                         0b0000_0110] {
        check_shift(test_value,
                    AluOperation::ShiftOrRotate {
                        d: AluOperationShiftOrRotateDirection::Left,
                        tt: AluOperationShiftOrRotateType::Rtc,
                    });
    }
}

#[test]
fn rotate_carry_right() {
    for &test_value in &[// forcebreak
                         0b1110_0001,
                         0b1001_1110,
                         0b1110_0000,
                         0b1000_0001,
                         0b0110_0001,
                         0b0001_1110,
                         0b0110_0000,
                         0b0000_0001] {
        check_shift(test_value,
                    AluOperation::ShiftOrRotate {
                        d: AluOperationShiftOrRotateDirection::Right,
                        tt: AluOperationShiftOrRotateType::Rtc,
                    });
    }
}

#[test]
fn rotate_left() {
    for &test_value in &[// forcebreak
                         0b1000_0111,
                         0b0111_1000,
                         0b1000_0001,
                         0b0000_0110] {
        check_shift(test_value,
                    AluOperation::ShiftOrRotate {
                        d: AluOperationShiftOrRotateDirection::Left,
                        tt: AluOperationShiftOrRotateType::Rtw,
                    });
    }
}

#[test]
fn rotate_right() {
    for &test_value in &[// forcebreak
                         0b1110_0001,
                         0b0001_1110,
                         0b0110_0000,
                         0b1000_0001] {
        check_shift(test_value,
                    AluOperation::ShiftOrRotate {
                        d: AluOperationShiftOrRotateDirection::Right,
                        tt: AluOperationShiftOrRotateType::Rtw,
                    });
    }
}



fn logic_arithmetic_shift_left(tt: AluOperationShiftOrRotateType) {
    for &test_value in &[// forcebreak
                         0b1000_0110,
                         0b0111_1000,
                         0b1000_0000,
                         0b0000_0110] {
        check_shift(test_value,
                    AluOperation::ShiftOrRotate {
                        d: AluOperationShiftOrRotateDirection::Left,
                        tt: tt,
                    });
    }
}


fn check_shift(test_value: u8, shift: AluOperation) {
    for stack_depth in 0..3 {
        for rhs in 0..=0xFF {
            let uni_orig = universe();
            let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) = uni_orig.clone();

            let mut stack = match stack_depth {
                0 => vec![],
                1 => vec![test_value],
                2 => vec![test_value, rhs],
                _ => unreachable!(),
            };

            assert_eq!(MicroOp::Alu(shift).execute(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr),
                       Err(MicrocodeExecutionError::MicrostackUnderflow));

            assert_eq!(memory, uni_orig.0);
            assert_eq!(ports, uni_orig.1);
            assert_eq!(registers, uni_orig.2);
            assert_eq!(pc, uni_orig.3);
            assert_eq!(sp, uni_orig.4);
            assert_eq!(adr, uni_orig.5);

            assert_eq!(stack, vec![]);
        }
    }
}
