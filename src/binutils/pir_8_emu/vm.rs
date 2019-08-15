use self::super::super::super::isa::{GeneralPurposeRegisterBank, GeneralPurposeRegister, SpecialPurposeRegister};
use self::super::super::super::micro::{MicroOpPerformError, MicroOpBlock, MicroOp, NEXT_INSTRUCTION};
use arraydeque::{ArrayDeque, Wrapping as ArrayDequeBehaviourWrapping};
use self::super::super::super::isa::instruction::Instruction;
use self::super::super::super::vm::{Memory, Ports};
use self::super::super::super::rw::ReadWritable;


/// Container for all data needed and/or useful for running a `pir-8-emu` virtual machine
///
/// # Examples
///
/// ```
/// # use pir_8_emu::ReadWritable;
/// # use pir_8_emu::binutils::pir_8_emu::Vm;
/// # use pir_8_emu::isa::instruction::Instruction;
/// let mut vm = Vm::new();
/// vm.reset(&[
///     Instruction::Halt.into(),
///     Instruction::LoadImmediate { aaa: 0b000 }.into(),
///     0x69,
///     Instruction::Save { aaa: 0b000 }.into(),
///     0x04,
///     0x20,
///     Instruction::Halt.into(),
/// ]);
///
/// vm.jump_to_addr(0x0001).unwrap();
/// while !vm.execution_finished {
///     vm.perform_next_op().unwrap();
///     vm.ins.reset_rw();
/// }
///
/// assert_eq!(vm.memory[0x0420], 0x69);
/// ```
#[derive(Debug)]
pub struct Vm {
    pub memory: Memory,
    pub ports: Ports,
    pub registers: GeneralPurposeRegisterBank,
    pub pc: SpecialPurposeRegister<u16>,
    pub sp: SpecialPurposeRegister<u16>,
    pub adr: SpecialPurposeRegister<u16>,
    pub ins: SpecialPurposeRegister<u8>,

    pub ops: (MicroOpBlock, usize),
    pub curr_op: usize,

    pub instruction: Instruction,
    /// If this is set, [`instruction`](#structfield.instruction) contains the current instruction and
    /// [`ops`](#structfield.ops) contains the μOps corresponding thereto
    pub instruction_valid: bool,
    pub execution_finished: bool,

    pub stack: Vec<u8>,

    /// Any instruction successfully loaded will be added to the front of this queue
    pub instruction_history: ArrayDeque<[(u16, Instruction, u16); 10], ArrayDequeBehaviourWrapping>,
}

impl Vm {
    /// Create a new, default-initialised VM
    pub fn new() -> Vm {
        Vm {
            memory: Memory::new(),
            ports: Ports::new(),
            registers: GeneralPurposeRegister::defaults(),
            pc: SpecialPurposeRegister::new("Program Counter", "PC"),
            sp: SpecialPurposeRegister::new("Stack Pointer", "SP"),
            adr: SpecialPurposeRegister::new("Memory Address", "ADR"),
            ins: SpecialPurposeRegister::new("Instruction", "INS"),

            ops: NEXT_INSTRUCTION,
            curr_op: 0,

            instruction: Instruction::Halt,
            instruction_valid: false,
            execution_finished: false,

            stack: vec![],

            instruction_history: ArrayDeque::new(),
        }
    }

    /// Reset this VM to a default state but with the specified memory buffer
    pub fn reset(&mut self, memory: &[u8]) {
        self.memory = Memory::from(memory);

        self.ports = Ports::new();
        self.registers = GeneralPurposeRegister::defaults();
        self.pc = SpecialPurposeRegister::new("Program Counter", "PC");
        self.sp = SpecialPurposeRegister::new("Stack Pointer", "SP");
        self.adr = SpecialPurposeRegister::new("Memory Address", "ADR");
        self.ins = SpecialPurposeRegister::new("Instruction", "INS");

        self.ops = NEXT_INSTRUCTION;
        self.curr_op = 0;
        self.instruction_valid = false;
        self.execution_finished = false;
        self.stack.clear();
        self.instruction_history.clear();
    }

    /// Safely jump to the specified address
    ///
    /// The current μOp set will be executed, then `PC` updated to the specified address, and μOps set to
    /// [`NEXT_INSTRUCTION`](../../../micro/static.NEXT_INSTRUCTION.html)
    pub fn jump_to_addr(&mut self, to_addr: u16) -> Result<(), MicroOpPerformError> {
        for _ in self.curr_op..self.ops.1 {
            self.perform_next_op()?;
        }

        *self.pc = to_addr;
        self.ops = NEXT_INSTRUCTION;
        self.instruction_valid = false;
        self.curr_op = 0;

        Ok(())
    }

    /// Perform next μOp
    ///
    /// If execution has finished, do nothing
    ///
    /// Otherwise, perform the current μOp and bump the μOp counter
    ///
    /// If the last μOp of the set has been performed:
    ///   * if `INS` was written to, load the instruction therein
    ///   * otherwise, load [`NEXT_INSTRUCTION`](../../../micro/static.NEXT_INSTRUCTION.html)
    ///
    /// The returned value represents whether new μOps are present
    pub fn perform_next_op(&mut self) -> Result<bool, MicroOpPerformError> {
        if self.execution_finished {
            return Ok(false);
        }

        let mut new_ops = false;

        self.execution_finished = !self.ops.0[self.curr_op].perform(&mut self.stack,
                     &mut self.memory,
                     &mut self.ports,
                     &mut self.registers,
                     &mut self.pc,
                     &mut self.sp,
                     &mut self.adr,
                     &mut self.ins)?;
        self.curr_op += 1;

        if self.curr_op >= self.ops.1 {
            if self.ins.was_written() {
                self.instruction = Instruction::from(*self.ins);
                self.ops = MicroOp::from_instruction(self.instruction);
                self.instruction_valid = true;

                let rw = self.adr.was_read() && self.adr.was_written();
                let mut data = 0u16;
                for i in 0..(self.instruction.data_length() as u16) {
                    data = (data << 8) | (self.memory[..][self.adr.wrapping_add(i) as usize] as u16);
                }
                self.instruction_history.push_front((*self.adr, self.instruction, data));
                if !rw {
                    self.adr.reset_rw();
                }
            } else {
                self.ops = NEXT_INSTRUCTION;
                self.instruction_valid = false;
            }

            self.curr_op = 0;
            new_ops = true;
        }

        Ok(new_ops)
    }
}
