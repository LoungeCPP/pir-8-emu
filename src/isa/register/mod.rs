mod general;
mod special;

pub use self::general::GeneralPurposeRegister;
pub use self::special::SpecialPurposeRegister;


/// Convenience typedef for bank of all 8 GP registers.
pub type GeneralPurposeRegisterBank = [GeneralPurposeRegister; 8];
