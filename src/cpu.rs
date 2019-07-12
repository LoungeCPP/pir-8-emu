use self::super::isa::{GeneralPurposeRegister, SpecialPurposeRegister, default_general_purpose_registers};


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

    /// There are eight 8-bit General Purpose registers, each has an internal address for use within the CPU.
    general_purpose: [GeneralPurposeRegister; 8],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: SpecialPurposeRegister::new("Program Counter", "PC"),
            sp: SpecialPurposeRegister::new("Stack Pointer", "SP"),
            adr: SpecialPurposeRegister::new("Memory Address", "ADR"),
            ins: SpecialPurposeRegister::new("Instruction", "INS"),

            general_purpose: default_general_purpose_registers(),
        }
    }
}
