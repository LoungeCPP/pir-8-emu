use self::super::super::isa::{GeneralPurposeRegisterBank, SpecialPurposeRegister};
use self::super::{MicrocodeExecutionError, MicroOp};
use self::super::super::{Memory, Ports};


const FLAG_BIT_ZERO: usize = 0; // Zero flag
const FLAG_BIT_CARRY: usize = 1; // Carry flag
const FLAG_BIT_PARITY: usize = 2; // Parity (even number of set bits)
const FLAG_BIT_EQUALS: usize = 3; // Equals flag
const FLAG_BIT_GREATER: usize = 4; // Greater than

const FLAG_MASK_ZERO: u8 = 1u8 << FLAG_BIT_ZERO;
const FLAG_MASK_CARRY: u8 = 1u8 << FLAG_BIT_CARRY;
const FLAG_MASK_PARITY: u8 = 1u8 << FLAG_BIT_PARITY;
const FLAG_MASK_EQUALS: u8 = 1u8 << FLAG_BIT_EQUALS;
const FLAG_MASK_GREATER: u8 = 1u8 << FLAG_BIT_GREATER;

const FLAG_CLEARFLAGS_MASK_ALU_OP: u8 = !(FLAG_MASK_ZERO | FLAG_MASK_CARRY | FLAG_MASK_PARITY);
const FLAG_CLEARFLAGS_MASK_COMP: u8 = !(FLAG_MASK_ZERO | FLAG_MASK_PARITY | FLAG_MASK_EQUALS | FLAG_MASK_GREATER);


static VALID_IS_OK_VALUES: &[u8] = &[0, 1];


impl MicroOp {
    /// Execute this μOp
    ///
    /// The `Ok(..)` return value indicates whether to continue execution (i.e. not halt)
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::isa::{GeneralPurposeRegister, SpecialPurposeRegister};
    /// # use pir_8_emu::microcode::MicroOp;
    /// # use pir_8_emu::{Memory, Ports};
    /// # let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr) =
    /// #     (Memory::new(), Ports::new(), GeneralPurposeRegister::defaults(),
    /// #      SpecialPurposeRegister::new("Program Counter", "PC"), SpecialPurposeRegister::new("Stack Pointer", "SP"),
    /// #      SpecialPurposeRegister::new("Memory Address", "ADR"));
    /// memory[0x1A00] = 0x69;
    ///
    /// let mut stack = vec![0x1A, 0x00];
    /// assert_eq!(MicroOp::FetchAddress.execute(&mut stack, &mut memory, &mut ports, &mut registers,
    ///                                          &mut pc, &mut sp, &mut adr),
    ///            Ok(true));
    /// assert_eq!(stack, &[0x69]);
    /// ```
    pub fn execute(&self, stack: &mut Vec<u8>, memory: &mut Memory, ports: &mut Ports, registers: &mut GeneralPurposeRegisterBank,
                   pc: &mut SpecialPurposeRegister<u16>, sp: &mut SpecialPurposeRegister<u16>, _adr: &mut SpecialPurposeRegister<u16>)
                   -> Result<bool, MicrocodeExecutionError> {
        match *self {
            MicroOp::Nop => {}
            MicroOp::Halt => return Ok(false),

            MicroOp::StackPush => {
                let byte = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;

                **sp = sp.checked_add(1).ok_or(MicrocodeExecutionError::StackOverflow)?;
                memory[**sp as usize] = byte;
            }
            MicroOp::StackPop => {
                let byte = memory[**sp as usize];
                **sp = sp.checked_sub(1).ok_or(MicrocodeExecutionError::StackUnderflow)?;

                stack.push(byte);
            }

            MicroOp::Alu(op) => {
                let flags = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;
                let rhs = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;
                let lhs = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;

                let mut carry = (flags & FLAG_MASK_CARRY) != 0;
                let result = op.perform(lhs, rhs, &mut carry);

                let flags = (flags & FLAG_CLEARFLAGS_MASK_ALU_OP)         |
                            if carry { FLAG_MASK_CARRY } else { 0b00000 } | // forcebreak
                            s_reg_flags(result);

                stack.push(result);
                stack.push(flags);
            }

            MicroOp::PortIn => {
                let port = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;

                let data = ports[port as usize];

                stack.push(data);
            }
            MicroOp::PortOut => {
                let port = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;
                let data = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;

                ports[port as usize] = data;
            }

            MicroOp::Compare => {
                let flags = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;
                let rhs = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;
                let lhs = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;

                let flags = (flags & FLAG_CLEARFLAGS_MASK_COMP)                   |
                            (if lhs == rhs { FLAG_MASK_EQUALS } else { 0b00000 }) |
                            (if lhs > rhs { FLAG_MASK_GREATER } else { 0b00000 }) | // forcebreak
                            s_reg_flags(lhs);

                stack.push(flags);
            }

            MicroOp::MakeImmediate(b) => stack.push(b),
            MicroOp::LoadImmediate => {
                let byte = memory[**pc as usize];
                **pc = pc.checked_add(1).ok_or(MicrocodeExecutionError::ProgramOverflow)?;

                stack.push(byte);
            }

            MicroOp::FetchAddress => {
                let address = pop_address(stack)?;

                let byte = memory[address as usize];

                stack.push(byte);
            }
            MicroOp::WriteAddress => {
                let address = pop_address(stack)?;
                let byte = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;

                memory[address as usize] = byte;
            }

            MicroOp::CheckJumpCondition(cond) => {
                let flags = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;

                let val = if cond.is_satisfied(flags) { 1 } else { 0 };

                stack.push(val);
            }
            MicroOp::Jump => {
                let is_ok = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;
                let address = pop_address(stack)?;

                match is_ok {
                    0 => **pc = pc.checked_add(2).ok_or(MicrocodeExecutionError::ProgramOverflow)?,
                    1 => **pc = address,
                    _ => return Err(MicrocodeExecutionError::InvalidMicrostackTop(is_ok, VALID_IS_OK_VALUES)),
                }
            }

            MicroOp::ReadRegister(aaa) => {
                let byte = *registers[aaa as usize];

                stack.push(byte);
            }
            MicroOp::WriteRegister(aaa) => {
                let byte = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;

                *registers[aaa as usize] = byte;
            }
        }

        Ok(true)
    }
}


fn s_reg_flags(s: u8) -> u8 {
    (if s == 0 { FLAG_MASK_ZERO } else { 0b00000 })                     | // forcebreak
    (if (s.count_ones() % 2) == 0 { FLAG_MASK_PARITY } else { 0b00000 })
}

fn pop_address(stack: &mut Vec<u8>) -> Result<u16, MicrocodeExecutionError> {
    let low_byte = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;
    let high_byte = stack.pop().ok_or(MicrocodeExecutionError::MicrostackUnderflow)?;

    Ok(((high_byte as u16) << 8) | (low_byte as u16))
}
