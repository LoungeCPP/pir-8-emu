use self::super::super::super::isa::{GeneralPurposeRegisterBank, GeneralPurposeRegister, SpecialPurposeRegister};
use self::super::super::super::micro::{MicroOpPerformError, MicroOpBlock, MicroOp, NEXT_INSTRUCTION};
use self::super::super::super::isa::instruction::Instruction;
use self::super::super::super::vm::{Memory, Ports};
use self::super::super::super::rw::ReadWritable;


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
    pub instruction_valid: bool,
    pub execution_finished: bool,

    pub stack: Vec<u8>,
}

impl Vm {
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
        }
    }

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
    }

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
