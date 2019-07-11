//! Module containing various utility functions.


use num_traits::{PrimInt, Num};


/// Limit the specified number to be at most the specified bit-width
///
/// # Examples
///
/// ```
/// # use pir_8_emu::util::limit_to_width;
/// assert_eq!(limit_to_width(0b0101, 3), Some(0b0101));
///
/// assert_eq!(limit_to_width(0b1010, 3), None);
/// ```
pub fn limit_to_width<T: Num + PrimInt>(number: T, bit_width: u8) -> Option<T> {
    if (number >> bit_width as usize) != T::zero() {
        None
    } else {
        Some(number)
    }
}
