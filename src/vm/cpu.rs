use self::super::super::isa::{GeneralPurposeRegisterBank, GeneralPurposeRegister, SpecialPurposeRegister};
use self::super::Memory;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cpu {
    /// Address of the next instruction to be fetched
    pub pc: SpecialPurposeRegister<u16>,
    /// Current address of the stack
    pub sp: SpecialPurposeRegister<u16>,
    /// Current address of RAM being accessed
    pub adr: SpecialPurposeRegister<u16>,
    /// Instruction currently being executed
    pub ins: SpecialPurposeRegister<u8>,

    /// There are eight 8-bit General Purpose registers, each has an internal address for use within the CPU
    pub general_purpose: GeneralPurposeRegisterBank,

    /// The entire 64KiB of addressable memory
    pub memory: Memory,
}

impl Cpu {
    /// Create a fresh CPU context with default registers and zeroed-out memory
    pub fn new() -> Cpu {
        Cpu {
            pc: SpecialPurposeRegister::new("Program Counter", "PC"),
            sp: SpecialPurposeRegister::new("Stack Pointer", "SP"),
            adr: SpecialPurposeRegister::new("Memory Address", "ADR"),
            ins: SpecialPurposeRegister::new("Instruction", "INS"),

            general_purpose: GeneralPurposeRegister::defaults(),

            memory: Memory::new(),
        }
    }
}
