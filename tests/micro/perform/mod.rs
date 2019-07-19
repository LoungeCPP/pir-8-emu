use pir_8_emu::isa::{GeneralPurposeRegisterBank, GeneralPurposeRegister, SpecialPurposeRegister};
use pir_8_emu::vm::{Memory, Ports};


mod err;
mod ok;


fn universe()
    -> (Memory,
        Ports,
        GeneralPurposeRegisterBank,
        SpecialPurposeRegister<u16>,
        SpecialPurposeRegister<u16>,
        SpecialPurposeRegister<u16>,
        SpecialPurposeRegister<u8>)
{
    (Memory::new(),
     Ports::new(),
     GeneralPurposeRegister::defaults(),
     SpecialPurposeRegister::new("Program Counter", "PC"),
     SpecialPurposeRegister::new("Stack Pointer", "SP"),
     SpecialPurposeRegister::new("Memory Address", "ADR"),
     SpecialPurposeRegister::new("Instruction", "INS"))
}
