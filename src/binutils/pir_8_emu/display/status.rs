//! Status bar handling


use self::super::super::super::super::isa::{GeneralPurposeRegisterBank, GeneralPurposeRegister};
use self::super::super::super::super::options::register_bank_validator;
use bear_lib_terminal::terminal::{print_xy, read_str, put_xy, clear};
use self::super::super::super::super::util::parse_with_prefix;
use bear_lib_terminal::geometry::{Point, Rect};
use num_traits::{PrimInt, Num};
use std::fmt::UpperHex;
use std::mem::size_of;
use std::str::FromStr;


const GP_REGISTER_COUNT: usize = size_of::<GeneralPurposeRegisterBank>() / size_of::<GeneralPurposeRegister>();


/// Write binary config value to the status line
///
/// Equivalent to [`line(..., "ON"/"OFF")`](fn.line.html)
pub fn config_bool(x_start: usize, y_start: usize, name: &str, is_on: bool) {
    line(x_start, y_start, name, if is_on { "ON" } else { "OFF" })
}

/// Clear the status line and write a K/V pair thereto
///
/// The status line is `80x1`, laid out as follows:
///
/// ```plaintext
/// <name>: <value>
/// ```
pub fn line(x_start: usize, y_start: usize, name: &str, value: &str) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;
    let name_len = name.len() as i32;

    status_line_header(x_start, y_start, name);

    print_xy(x_start + name_len + 1 + 1, y_start, value);
}

/// Read the specified number type at the status line
///
/// The prompt will appear in the value space, and the value will be set to the hex-formatted in-put value,
/// if the input was cancelled, it'll be `{cancelled}`, and if the number wasn't valid, `{parse failed}`
pub fn read_number<T: Num + PrimInt + UpperHex>(x_start: usize, y_start: usize, label: &str) -> Option<T> {
    let x_start = x_start as i32;
    let y_start = y_start as i32;
    let label_len = label.len() as i32;
    let x_past_header_start = x_start + label_len + 1 + 1;

    status_line_header(x_start, y_start, label);

    match read_str(Point::new(x_past_header_start, y_start), 80 - x_past_header_start) {
        Some(raw_addr) => {
            match parse_with_prefix(&raw_addr) {
                Some(addr) => {
                    print_xy(x_past_header_start, y_start, &format!("{:#0w$X}", addr, w = 2 + size_of::<T>() * 2));
                    Some(addr)
                }
                None => {
                    print_xy(x_past_header_start, y_start, "{parse failed}");
                    None
                }
            }
        }
        None => {
            print_xy(x_past_header_start, y_start, "{cancelled}");
            None
        }
    }
}

/// Read a positive floating-point number at the status line
///
/// The prompt will appear in the value space, and the value will be set to in-put value,
/// if the input was cancelled, it'll be `{cancelled}`, and if the number wasn't valid, `{parse failed}`
pub fn read_pos_float(x_start: usize, y_start: usize, label: &str) -> Option<f64> {
    let x_start = x_start as i32;
    let y_start = y_start as i32;
    let label_len = label.len() as i32;
    let x_past_header_start = x_start + label_len + 1 + 1;

    status_line_header(x_start, y_start, label);

    match read_str(Point::new(x_past_header_start, y_start), 80 - x_past_header_start) {
        Some(raw_flt) => {
            match f64::from_str(&raw_flt) {
                Ok(flt) if flt > 0f64 => {
                    print_xy(x_past_header_start, y_start, &flt.to_string());
                    Some(flt)
                }
                _ => {
                    print_xy(x_past_header_start, y_start, "{parse failed}");
                    None
                }
            }
        }
        None => {
            print_xy(x_past_header_start, y_start, "{cancelled}");
            None
        }
    }
}

/// Read General-purpose register letters at the status line
///
/// The prompt will appear in the value space, and the value will be set to new register letters,
/// if the input was cancelled, it'll be `{cancelled}`, and if the labels weren't valid, `{<error message>}`
pub fn read_gp_register_letters(x_start: usize, y_start: usize) -> Option<[char; GP_REGISTER_COUNT]> {
    static LABEL: &str = "General-purpose register letters";

    let x_start = x_start as i32;
    let y_start = y_start as i32;
    let label_len = LABEL.len() as i32;
    let x_past_header_start = x_start + label_len + 1 + 1;

    status_line_header(x_start, y_start, LABEL);

    match read_str(Point::new(x_past_header_start, y_start), GP_REGISTER_COUNT as i32) {
        Some(letters) => {
            match register_bank_validator(&letters) {
                Ok(()) => {
                    print_xy(x_past_header_start, y_start, &letters);

                    let mut ret = ['\0'; GP_REGISTER_COUNT];
                    for (i, c) in letters.chars().into_iter().enumerate() {
                        ret[i] = c;
                    }
                    Some(ret)
                }
                Err(err) => {
                    let err_msg = &err["Register bank ".len()..];

                    put_xy(x_past_header_start, y_start, '{');
                    print_xy(x_past_header_start + 1, y_start, err_msg);
                    put_xy(x_past_header_start + 1 + err_msg.len() as i32, y_start, '}');
                    None
                }
            }
        }
        None => {
            print_xy(x_past_header_start, y_start, "{cancelled}");
            None
        }
    }
}


fn status_line_header(x_start: i32, y_start: i32, label: &str) {
    let label_len = label.len() as i32;

    clear(Some(Rect::from_values(x_start, y_start, 80, 1)));

    print_xy(x_start, y_start, label);
    put_xy(x_start + label_len, y_start, ':');
}
