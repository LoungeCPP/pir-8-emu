//! Status bar handling


use bear_lib_terminal::terminal::{print_xy, read_str, put_xy, clear};
use self::super::super::super::super::util::parse_with_prefix;
use bear_lib_terminal::geometry::{Point, Rect};
use num_traits::{PrimInt, Num};
use std::fmt::UpperHex;
use std::mem::size_of;


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
/// The prompt will appear in the value space, and the value will be set to the hex-formatter in-put value,
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


fn status_line_header(x_start: i32, y_start: i32, label: &str) {
    let label_len = label.len() as i32;

    clear(Some(Rect::from_values(x_start, y_start, 80, 1)));

    print_xy(x_start, y_start, label);
    put_xy(x_start + label_len, y_start, ':');
}
