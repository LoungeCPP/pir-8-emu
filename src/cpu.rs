use self::super::isa::SpecialPurposeRegister;


#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cpu {
    /// Address of the next instruction to be fetched
    pc: SpecialPurposeRegister<u16>,
    /// Current address of the stack
    sp: SpecialPurposeRegister<u16>,
    /// Current address of RAM being accessed
    adr: SpecialPurposeRegister<u16>,
    /// Instruction currently being executed
    ins: SpecialPurposeRegister<u8>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: SpecialPurposeRegister::new("Program Counter", "PC"),
            sp: SpecialPurposeRegister::new("Stack Pointer", "SP"),
            adr: SpecialPurposeRegister::new("Memory Address", "ADR"),
            ins: SpecialPurposeRegister::new("Instruction", "INS"),
        }
    }
}
