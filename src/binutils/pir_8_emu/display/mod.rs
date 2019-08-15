//! BearLibTerminal display management and handling


use bear_lib_terminal::terminal::{with_colors, print_xy, read_str, put_xy, clear};
use self::super::super::super::vm::{MemoryPortsReadWrittenIterator, Ports};
use self::super::super::super::isa::GeneralPurposeRegisterBank;
use self::super::super::super::isa::instruction::Instruction;
use self::super::super::super::util::parse_with_prefix;
use num_traits::{Unsigned, NumCast, PrimInt, Num};
use bear_lib_terminal::geometry::{Point, Rect};
use bear_lib_terminal::Color;
use std::fmt::UpperHex;
use std::mem::size_of;
use std::borrow::Cow;

pub mod register;
pub mod memory;
pub mod micro;


const COLUMN_WIDTH: i32 = 27;


/// Get the colours for the specified R/W state
///
/// Background contains, if `read`, green, if `written`, red.
///
/// Foreground is white by default and black if `read`.
pub fn colours_for_rw(read: bool, written: bool) -> (Color, Color) {
    let fg = if read {
        Color::from_rgb(0x00, 0x00, 0x00)
    } else {
        Color::from_rgb(0xFF, 0xFF, 0xFF)
    };

    let bg = Color::from_rgb(if written { 0xFF } else { 0x00 }, if read { 0xFF } else { 0x00 }, 0x00);

    (fg, bg)
}

/// Prepare the "Current instruction" window at the specified coords
///
/// The window is `20x2`, laid out as follows:
///
/// ```plaintext
/// Current instruction
/// <instr>
/// ```
///
/// Where `<instr>` is either `{undecoded}`, `{execution finished}`, or the assembly name of the instruction.
pub fn instruction_write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Current instruction");
    print_xy(x_start, y_start + 1, "{undecoded}");
}

/// Update the "Current instruction" window
///
/// See [`instruction_write()`](fn.instruction_write.html) for more info
pub fn instruction_update(x_start: usize, y_start: usize, valid: bool, execution_finished: bool, instr: &Instruction, registers: &GeneralPurposeRegisterBank) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    let disp = if execution_finished {
        Cow::Borrowed("{execution finished}")
    } else if valid {
        Cow::Owned(instr.display(registers).to_string())
    } else {
        Cow::Borrowed("{undecoded}")
    };
    print_xy(x_start, y_start + 1, &disp);

    clear(Some(Rect::from_values(x_start + disp.len() as i32, y_start + 1, COLUMN_WIDTH - disp.len() as i32, 1)));
}

/// Prepare the "Instruction history" window at the specified coords
///
/// The window is `25x(1+N)`, laid out as follows:
///
/// ```plaintext
/// Instruction history
/// <instrs>
/// ```
///
/// Where `<instrs>` is either `{empty}`, or up to `N` lines of `pir-8-disasm`-formatted instructions and data therefor,
/// and `N` is `max_instr_count`, as passed to [`instruction_history_update()`](fn.instruction_history_update.html)
pub fn instruction_history_write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Instruction history");
    print_xy(x_start, y_start + 1, "{empty}");
}

/// Update the "Instruction history" window
///
/// See [`instruction_history_write()`](fn.instruction_history_write.html) for more info
pub fn instruction_history_update<'i, I: IntoIterator<Item = &'i (u16, Instruction, u16)>>(x_start: usize, y_start: usize, instrs: I, max_instr_count: usize,
                                                                                           registers: &GeneralPurposeRegisterBank) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;
    let max_instr_count = max_instr_count as i32;

    clear(Some(Rect::from_values(x_start, y_start + 1, 25, max_instr_count)));

    let mut cur_line = 0;
    for (addr, instr, data) in instrs {
        if cur_line >= max_instr_count {
            break;
        }

        let (clr_start, clr_end) = if cur_line == 0 {
            ("[bkcolor=darker grey]", "[/bkcolor]")
        } else {
            ("", "")
        };

        print_xy(x_start,
                 y_start + 1 + cur_line,
                 &format!("{}{:04X} {} {}{}",
                          clr_start,
                          addr,
                          if !instr.is_valid() { '!' } else { ' ' },
                          instr.display(registers),
                          clr_end));
        cur_line += 1;

        if instr.data_length() != 0 && cur_line < max_instr_count {
            print_xy(x_start,
                     y_start + 1 + cur_line,
                     &format!("{}{:04X} D {:#0w$X}{}",
                              clr_start,
                              addr.wrapping_add(instr.data_length() as u16),
                              data,
                              clr_end,
                              w = instr.data_length() * 2));
            cur_line += 1;
        }
    }

    if cur_line == 0 {
        print_xy(x_start, y_start + 1, "{empty}");
    }
}

/// Prepare the "Port activity" window at the specified coords
///
/// The window is `13x6`, laid out as follows:
///
/// ```plaintext
/// Port activity
/// <port_acts>
/// ```
///
/// Where `<port_acts>` is either `{none}`, or up to 5 lines in the following format:
///
/// ```plaintext
/// 00 ← 0x10
/// 01 → 0x11
/// 02 ≡ 0x12
/// ```
///
/// Where port `0x00` was written to and has value `0x10`,
///       port `0x01` was read from and has value `0x11`, and
///       port `0x02` was read from and written to and has value `0x12`,
pub fn ports_rw_write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Port activity");
    print_xy(x_start, y_start + 1, "{none}");
}

/// Update the "Port activity" window
///
/// See [`ports_rw_write()`](fn.ports_rw_write.html) for more info
pub fn ports_rw_update(x_start: usize, y_start: usize, pts: &mut Ports) {
    mem_ports_rw_update(x_start, y_start, pts.iter_rw());
    pts.reset_rw();
}

/// Write binary config value to the status line
///
/// Equivalent to [`status_line(..., "ON"/"OFF")`](fn.status_line.html)
pub fn config(x_start: usize, y_start: usize, name: &str, is_on: bool) {
    status_line(x_start, y_start, name, if is_on { "ON" } else { "OFF" })
}

/// Clear the status line and write a K/V pair thereto
///
/// The status line is `80x1`, laid out as follows:
///
/// ```plaintext
/// <name>: <value>
/// ```
pub fn status_line(x_start: usize, y_start: usize, name: &str, value: &str) {
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


fn mem_ports_rw_update<IdxT: Num + Unsigned + PrimInt + NumCast + UpperHex>(x_start: usize, y_start: usize, itr: MemoryPortsReadWrittenIterator<IdxT>) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;
    let addr_width = size_of::<IdxT>() * 2;

    let mut cur_line = 0;
    for (addr, val, r, w) in itr {
        let (fg, bg) = colours_for_rw(r, w);
        with_colors(fg, bg, || print_xy(x_start, y_start + 1 + cur_line, &format!("{:0w$X}", addr, w = addr_width)));

        print_xy(x_start + addr_width as i32,
                 y_start + 1 + cur_line,
                 &format!(" {} {:#04X}",
                          match (r, w) {
                              (false, false) => unreachable!(),
                              (false, true) => '←',
                              (true, false) => '→',
                              (true, true) => '≡',
                          },
                          val));

        cur_line += 1;
    }

    if cur_line == 0 {
        print_xy(x_start, y_start + 1, "{none}      ");
        cur_line += 1;
    }

    if cur_line < 5 {
        clear(Some(Rect::from_values(x_start, y_start + 1 + cur_line, 4 + 3 + 4, 5 - cur_line)));
    }
}

fn status_line_header(x_start: i32, y_start: i32, label: &str) {
    let label_len = label.len() as i32;

    clear(Some(Rect::from_values(x_start, y_start, 80, 1)));

    print_xy(x_start, y_start, label);
    put_xy(x_start + label_len, y_start, ':');
}
