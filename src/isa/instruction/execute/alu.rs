use self::super::super::{AluOperationShiftOrRotateDirection, AluOperationShiftOrRotateType, AluOperation};


enum RespectCarry {
    No,
    YesAdd,
    YesSub,
}


impl AluOperation {
    pub(in self::super::super) fn perform_impl(self, lhs: u8, rhs: u8, carry: &mut bool) -> u8 {
        match self {
            AluOperation::Reserved(_) => {
                *carry = true;
                0
            }

            AluOperation::Add => add(lhs, rhs, carry, RespectCarry::No),
            // [11:38] Cat Plus Plus: Or basically: just do it as addition but two-complement negate the second operand
            // [11:38] Cat Plus Plus: So 0x02 - 0x04 is 0x02 + 0xFC
            // https://discordapp.com/channels/145079846832308224/411537636994449418/600259783072940044
            AluOperation::Sub => add(lhs, (!rhs).checked_add(1).unwrap_or(0), carry, RespectCarry::No),

            AluOperation::AddC => add(lhs, rhs, carry, RespectCarry::YesAdd),
            AluOperation::SubC => add(lhs, (!rhs).checked_add(1).unwrap_or(0), carry, RespectCarry::YesSub),

            AluOperation::Not => !lhs,
            AluOperation::Or => lhs | rhs,
            AluOperation::Xor => lhs ^ rhs,
            AluOperation::And => lhs & rhs,

            AluOperation::ShiftOrRotate { d: AluOperationShiftOrRotateDirection::Left, tt } => {
                let new_carry = (lhs & 0b1000_0000) != 0;

                let ret = match tt {
                    AluOperationShiftOrRotateType::Lsf |
                    AluOperationShiftOrRotateType::Asf => lhs << 1,
                    AluOperationShiftOrRotateType::Rtc => {
                        let carry_mask = if *carry { 0b0000_0001 } else { 0b0000_0000 };

                        (lhs << 1) | carry_mask
                    }
                    AluOperationShiftOrRotateType::Rtw => lhs.rotate_left(1),
                };

                *carry = new_carry;
                ret
            }

            AluOperation::ShiftOrRotate { d: AluOperationShiftOrRotateDirection::Right, tt } => {
                let new_carry = (lhs & 0b0000_0001) != 0;

                let ret = match tt {
                    AluOperationShiftOrRotateType::Lsf => lhs >> 1,
                    AluOperationShiftOrRotateType::Asf => {
                        let msb_mask = lhs & 0b1000_0000;

                        (lhs >> 1) | msb_mask
                    }
                    AluOperationShiftOrRotateType::Rtc => {
                        let carry_mask = if *carry { 0b1000_0000 } else { 0b0000_0000 };

                        (lhs >> 1) | carry_mask
                    }
                    AluOperationShiftOrRotateType::Rtw => lhs.rotate_right(1),
                };

                *carry = new_carry;
                ret
            }
        }
    }
}

fn add(lhs: u8, rhs: u8, carry: &mut bool, respect: RespectCarry) -> u8 {
    let mut sum = (lhs as u16) + (rhs as u16);

    match (*carry, respect) {
        (false, _) |
        (true, RespectCarry::No) => {}
        (true, RespectCarry::YesAdd) => sum += 1,
        (true, RespectCarry::YesSub) => sum -= 1,
    }

    *carry = (sum & 0b1_0000_0000) != 0;

    (sum & 0b1111_1111) as u8
}
