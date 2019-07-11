use self::super::isa::{GeneralPurposeRegister, SpecialPurposeRegister};


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

            general_purpose: [GeneralPurposeRegister::new(0b000, 'F').expect("F register"), // Flag register (can also be used to get a zero value)
                              GeneralPurposeRegister::new(0b001, 'S').expect("S register"), // Output of the ALU - ALU operations will overwrite any value stored
                              GeneralPurposeRegister::new(0b010, 'X').expect("X register"), // Input to ALU (Only input for unary operations)
                              GeneralPurposeRegister::new(0b011, 'Y').expect("Y register"), // Second input for ALU
                              GeneralPurposeRegister::new(0b100, 'A').expect("A register"),
                              GeneralPurposeRegister::new(0b101, 'B').expect("B register"),
                              GeneralPurposeRegister::new(0b110, 'C').expect("C register"),
                              GeneralPurposeRegister::new(0b111, 'D').expect("D register")],
        }
    }
}
