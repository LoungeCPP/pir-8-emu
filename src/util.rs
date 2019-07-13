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

/// Parse a number from the specified string, automatically detecting the base prefix.
///
/// # Examples
///
/// ```
/// # use pir_8_emu::util::parse_with_prefix;
/// assert_eq!(parse_with_prefix::<u16>("0x0420"), Some(0x0420));
/// assert_eq!(parse_with_prefix::<u16>("0o0420"), Some(0o0420));
/// assert_eq!(parse_with_prefix::<u16>("0B0101"), Some(0b0101));
///
/// assert_eq!(parse_with_prefix::<u16>("0"), Some(0));
///
/// assert_eq!(parse_with_prefix::<u16>("0x2OOM"), None);
/// ```
pub fn parse_with_prefix<T: Num + PrimInt>(from: &str) -> Option<T> {
    let mut cc = from.chars();

    let (radix, depth) = if cc.next()? == '0' {
        match cc.next() {
            Some('x') | Some('X') => (16, 2),
            Some('o') | Some('O') => (8, 2),
            Some('b') | Some('B') => (2, 2),
            Some(_) => (10, 0),
            None => return Some(T::zero()),
        }
    } else {
        (10, 0)
    };

    T::from_str_radix(&from[depth..], radix).ok()
}
