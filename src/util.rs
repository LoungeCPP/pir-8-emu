//! Module containing various utility functions.


use num_traits::{CheckedShr, PrimInt, Num};


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
pub fn limit_to_width<T: Num + PrimInt + CheckedShr>(number: T, bit_width: u8) -> Option<T> {
    if number.checked_shr(bit_width.into()).unwrap_or(T::zero()) != T::zero() {
        None
    } else {
        Some(number)
    }
}

/// Limit the specified number to be at most the specified bit-width
///
/// # Examples
///
/// ```
/// # use pir_8_emu::util::min_byte_width;
/// assert_eq!(min_byte_width(0x0F), 1);
/// assert_eq!(min_byte_width(0x010F), 2);
/// assert_eq!(min_byte_width(0x00F0010F), 4);
/// ```
pub fn min_byte_width<T: Num + PrimInt + CheckedShr>(number: T) -> u8 {
    let mut cur_bytes = 1;
    while number.checked_shr(8 * cur_bytes).unwrap_or(T::zero()) != T::zero() {
        cur_bytes *= 2;
    }
    cur_bytes as u8
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
/// assert_eq!(parse_with_prefix::<u16>("0b1010_0101"), Some(0b1010_0101));
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

    if from.contains('_') {
            T::from_str_radix(&from[depth..].replace('_', ""), radix).ok()
        } else {
            T::from_str_radix(&from[depth..], radix).ok()
        }
}

/// Strip off all data starting with the specified character, if exists
///
/// # Examples
///
/// ```
/// # use pir_8_emu::util::remove_comment;
/// assert_eq!(remove_comment(';', "UwU ; OwO"), "UwU ");
///
/// assert_eq!(remove_comment(';', "yeehaw"), "yeehaw");
/// ```
pub fn remove_comment(comment_char: char, from: &str) -> &str {
    match from.find(comment_char) {
        Some(idx) => &from[0..idx],
        None => from,
    }
}
