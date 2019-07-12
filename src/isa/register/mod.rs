mod general;
mod special;

pub use self::general::GeneralPurposeRegister;
pub use self::special::SpecialPurposeRegister;


/// Get the default 8 GP registers specified in the ISA
pub fn default_general_purpose_registers() -> [GeneralPurposeRegister; 8] {
    [GeneralPurposeRegister::new(0b000, 'F').expect("F register"), // Flag register (can also be used to get a zero value)
     GeneralPurposeRegister::new(0b001, 'S').expect("S register"), // Output of the ALU - ALU operations will overwrite any value stored
     GeneralPurposeRegister::new(0b010, 'X').expect("X register"), // Input to ALU (Only input for unary operations)
     GeneralPurposeRegister::new(0b011, 'Y').expect("Y register"), // Second input for ALU
     GeneralPurposeRegister::new(0b100, 'A').expect("A register"),
     GeneralPurposeRegister::new(0b101, 'B').expect("B register"),
     GeneralPurposeRegister::new(0b110, 'C').expect("C register"),
     GeneralPurposeRegister::new(0b111, 'D').expect("D register")]
}
