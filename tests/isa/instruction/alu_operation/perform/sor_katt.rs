//! https://github.com/TheCatPlusPlus/pir8/blob/3f126ba3dd1201e42e8928d74a20c151b9a97823/PIR8.ISA.Tests/Shifts.cs


use pir_8_emu::isa::instruction::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, AluOperation};


const TEST_VALUE_1: u8 = 0b1100_0011;
const TEST_VALUE_2: u8 = 0b0011_1100;
const TEST_VALUE_3: u8 = 0b1100_0000;
const TEST_VALUE_4: u8 = 0b0000_0011;


#[test]
fn logic_shift_left() {
    logic_arithmetic_shift_left(AluOperationShiftOrRotateType::Lsf);
}

#[test]
fn logic_shift_right() {
    for &start_carry in &[false, true] {
        for &(expected_value, expected_carry, test_value) in
            &[// forcebreak
              (0b0110_0001, true, TEST_VALUE_1),
              (0b0001_1110, false, TEST_VALUE_2),
              (0b0110_0000, false, TEST_VALUE_3),
              (0b0000_0001, true, TEST_VALUE_4)] {
            check_shift(expected_value,
                        expected_carry,
                        test_value,
                        AluOperation::ShiftOrRotate {
                            d: AluOperationShiftOrRotateDirection::Right,
                            tt: AluOperationShiftOrRotateType::Lsf,
                        },
                        start_carry);
        }
    }
}

#[test]
fn arithmetic_shift_left() {
    logic_arithmetic_shift_left(AluOperationShiftOrRotateType::Asf);
}

#[test]
fn arithmetic_shift_right() {
    for &start_carry in &[false, true] {
        for &(expected_value, expected_carry, test_value) in
            &[// forcebreak
              (0b1110_0001, true, TEST_VALUE_1),
              (0b0001_1110, false, TEST_VALUE_2),
              (0b1110_0000, false, TEST_VALUE_3),
              (0b0000_0001, true, TEST_VALUE_4)] {
            check_shift(expected_value,
                        expected_carry,
                        test_value,
                        AluOperation::ShiftOrRotate {
                            d: AluOperationShiftOrRotateDirection::Right,
                            tt: AluOperationShiftOrRotateType::Asf,
                        },
                        start_carry);
        }
    }
}

#[test]
fn rotate_carry_left() {
    for &(expected_value, expected_carry, test_value, test_carry) in
        &[(0b1000_0111, true, TEST_VALUE_1, true),
          (0b0111_1001, false, TEST_VALUE_2, true),
          (0b1000_0001, true, TEST_VALUE_3, true),
          (0b0000_0111, false, TEST_VALUE_4, true),
          (0b1000_0110, true, TEST_VALUE_1, false),
          (0b0111_1000, false, TEST_VALUE_2, false),
          (0b1000_0000, true, TEST_VALUE_3, false),
          (0b0000_0110, false, TEST_VALUE_4, false)] {
        check_shift(expected_value,
                    expected_carry,
                    test_value,
                    AluOperation::ShiftOrRotate {
                        d: AluOperationShiftOrRotateDirection::Left,
                        tt: AluOperationShiftOrRotateType::Rtc,
                    },
                    test_carry);
    }
}

#[test]
fn rotate_carry_right() {
    for &(expected_value, expected_carry, test_value, test_carry) in
        &[(0b1110_0001, true, TEST_VALUE_1, true),
          (0b1001_1110, false, TEST_VALUE_2, true),
          (0b1110_0000, false, TEST_VALUE_3, true),
          (0b1000_0001, true, TEST_VALUE_4, true),
          (0b0110_0001, true, TEST_VALUE_1, false),
          (0b0001_1110, false, TEST_VALUE_2, false),
          (0b0110_0000, false, TEST_VALUE_3, false),
          (0b0000_0001, true, TEST_VALUE_4, false)] {
        check_shift(expected_value,
                    expected_carry,
                    test_value,
                    AluOperation::ShiftOrRotate {
                        d: AluOperationShiftOrRotateDirection::Right,
                        tt: AluOperationShiftOrRotateType::Rtc,
                    },
                    test_carry);
    }
}

#[test]
fn rotate_left() {
    for &start_carry in &[false, true] {
        for &(expected_value, expected_carry, test_value) in
            &[// forcebreak
              (0b1000_0111, true, TEST_VALUE_1),
              (0b0111_1000, false, TEST_VALUE_2),
              (0b1000_0001, true, TEST_VALUE_3),
              (0b0000_0110, false, TEST_VALUE_4)] {
            check_shift(expected_value,
                        expected_carry,
                        test_value,
                        AluOperation::ShiftOrRotate {
                            d: AluOperationShiftOrRotateDirection::Left,
                            tt: AluOperationShiftOrRotateType::Rtw,
                        },
                        start_carry);
        }
    }
}

#[test]
fn rotate_right() {
    for &start_carry in &[false, true] {
        for &(expected_value, expected_carry, test_value) in
            &[// forcebreak
              (0b1110_0001, true, TEST_VALUE_1),
              (0b0001_1110, false, TEST_VALUE_2),
              (0b0110_0000, false, TEST_VALUE_3),
              (0b1000_0001, true, TEST_VALUE_4)] {
            check_shift(expected_value,
                        expected_carry,
                        test_value,
                        AluOperation::ShiftOrRotate {
                            d: AluOperationShiftOrRotateDirection::Right,
                            tt: AluOperationShiftOrRotateType::Rtw,
                        },
                        start_carry);
        }
    }
}


fn logic_arithmetic_shift_left(tt: AluOperationShiftOrRotateType) {
    for &start_carry in &[false, true] {
        for &(expected_value, expected_carry, test_value) in
            &[// forcebreak
              (0b1000_0110, true, TEST_VALUE_1),
              (0b0111_1000, false, TEST_VALUE_2),
              (0b1000_0000, true, TEST_VALUE_3),
              (0b0000_0110, false, TEST_VALUE_4)] {
            check_shift(expected_value,
                        expected_carry,
                        test_value,
                        AluOperation::ShiftOrRotate {
                            d: AluOperationShiftOrRotateDirection::Left,
                            tt,
                        },
                        start_carry);
        }
    }
}

fn check_shift(expected_value: u8, expected_carry: bool, test_value: u8, shift: AluOperation, test_carry: bool) {
    for rhs in 0..=0xFF {
        let mut carry = test_carry;
        let value = shift.perform(test_value, rhs, &mut carry);

        assert_eq!(value, expected_value);
        assert_eq!(carry, expected_carry);
    }
}
