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


pub fn colours_for_rw(read: bool, written: bool) -> (Color, Color) {
    let fg = if read {
        Color::from_rgb(0x00, 0x00, 0x00)
    } else {
        Color::from_rgb(0xFF, 0xFF, 0xFF)
    };

    let bg = Color::from_rgb(if written { 0xFF } else { 0x00 }, if read { 0xFF } else { 0x00 }, 0x00);

    (fg, bg)
}


pub fn instruction_write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Current instruction");
    print_xy(x_start, y_start + 1, "{undecoded}");
}

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

pub fn instruction_history_write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Instruction history");
    print_xy(x_start, y_start + 1, "{empty}");
}

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

pub fn config(x_start: usize, y_start: usize, name: &str, is_on: bool) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;
    let name_len = name.len() as i32;

    clear(Some(Rect::from_values(x_start, y_start, 80, 1)));

    print_xy(x_start, y_start, name);
    put_xy(x_start + name_len, y_start, ':');
    print_xy(x_start + name_len + 1 + 1, y_start, if is_on { "ON" } else { "OFF" });
}

pub fn read_address(x_start: usize, y_start: usize, label: &str) -> Option<u16> {
    let x_start = x_start as i32;
    let y_start = y_start as i32;
    let label_len = label.len() as i32;
    let x_past_header_start = x_start + label_len + 1 + 1;

    clear(Some(Rect::from_values(x_start, y_start, 80, 1)));

    print_xy(x_start, y_start, label);
    put_xy(x_start + label_len, y_start, ':');

    match read_str(Point::new(x_past_header_start, y_start), 80 - x_past_header_start) {
        Some(raw_addr) => {
            match parse_with_prefix(&raw_addr) {
                Some(addr) => {
                    print_xy(x_past_header_start, y_start, &format!("{:#06X}", addr));
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

pub fn ports_rw_write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Port activity");
    print_xy(x_start, y_start + 1, "{none}");
}

pub fn ports_rw_update(x_start: usize, y_start: usize, pts: &mut Ports) {
    mem_ports_rw_update(x_start, y_start, pts.iter_rw());
    pts.reset_rw();
}


fn mem_ports_rw_update<IdxT: Num + Unsigned + PrimInt + NumCast + UpperHex>(x_start: usize, y_start: usize, itr: MemoryPortsReadWrittenIterator<IdxT>) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    let mut cur_line = 0;
    for (addr, val, r, w) in itr {
        let (fg, bg) = colours_for_rw(r, w);
        with_colors(fg,
                    bg,
                    || print_xy(x_start, y_start + 1 + cur_line, &format!("{:0w$X}", addr, w = size_of::<IdxT>() * 2)));

        print_xy(x_start + 4,
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
