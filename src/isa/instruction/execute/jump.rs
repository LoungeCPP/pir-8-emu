use self::super::super::InstructionJumpCondition;


const FLAG_BIT_ZERO: usize = 0; // Zero flag
const FLAG_BIT_CARRY: usize = 1; // Carry flag
const FLAG_BIT_PARITY: usize = 2; // Parity (even number of set bits)
const FLAG_BIT_GREATER: usize = 4; // Greater than

const FLAG_MASK_ZERO: u8 = 1u8 << FLAG_BIT_ZERO;
const FLAG_MASK_CARRY: u8 = 1u8 << FLAG_BIT_CARRY;
const FLAG_MASK_PARITY: u8 = 1u8 << FLAG_BIT_PARITY;
const FLAG_MASK_GREATER: u8 = 1u8 << FLAG_BIT_GREATER;


impl InstructionJumpCondition {
    /// Check, whether the specified flagset satisfies this jump condition
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::instruction::InstructionJumpCondition;
    /// assert!(InstructionJumpCondition::Jmpg.is_satisfied(0b10100));
    ///
    /// assert!(!InstructionJumpCondition::Jmpl.is_satisfied(0b00101));
    /// ```
    pub fn is_satisfied(self, flags: u8) -> bool {
        match self {
            // Zero flag
            InstructionJumpCondition::Jmpz => (flags & FLAG_MASK_ZERO) != 0,
            // Parity flag
            InstructionJumpCondition::Jmpp => (flags & FLAG_MASK_PARITY) != 0,
            // NOT Zero AND Greater than flag (i.e. greater than)
            InstructionJumpCondition::Jmpg => ((flags & FLAG_MASK_ZERO) == 0) && ((flags & FLAG_MASK_GREATER) != 0),
            // Carry flag
            InstructionJumpCondition::Jmpc => (flags & FLAG_MASK_CARRY) != 0,
            // Zero OR Greater than flags
            InstructionJumpCondition::Jmzg => ((flags & FLAG_MASK_ZERO) != 0) || ((flags & FLAG_MASK_GREATER) != 0),
            // Zero OR NOT Greater than flag
            InstructionJumpCondition::Jmzl => ((flags & FLAG_MASK_ZERO) != 0) || ((flags & FLAG_MASK_GREATER) == 0),
            // NOT Zero AND NOT Greater than flag (i.e. less than)
            InstructionJumpCondition::Jmpl => ((flags & FLAG_MASK_ZERO) == 0) && ((flags & FLAG_MASK_GREATER) == 0),
            // Unconditional Jump (always jumps)
            InstructionJumpCondition::Jump => true,
        }
    }
}
