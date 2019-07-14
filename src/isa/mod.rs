//! All data specified directly in [the ISA](//github.com/thecoshman/pir-8/blob/master/ISA.md).
//!
//! Unless stated otherwise, bits are always represented from MSB to LSB (reading left to right) and multi-bytes sequences are
//! big-endian. So, a jump instruction followed by a two byte address would have the following sequence of bytes "jump", "high
//! address byte", "low address byte".


mod register;

pub mod instruction;

pub use self::register::{GeneralPurposeRegisterBank, GeneralPurposeRegister, SpecialPurposeRegister};
