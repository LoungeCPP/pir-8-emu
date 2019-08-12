use self::super::super::super::isa::GeneralPurposeRegisterBank;
use self::super::super::super::isa::instruction::Instruction;
use bear_lib_terminal::terminal::{print_xy, put_xy, clear};
use bear_lib_terminal::geometry::Rect;
use bear_lib_terminal::Color;
use std::borrow::Cow;

pub mod register;
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

    clear(Some(Rect::from_values(x_start, y_start + 1, COLUMN_WIDTH, max_instr_count as i32 * 2)));

    let mut cur_line = 0;
    for (addr, instr, data) in instrs {
        print_xy(x_start,
                 y_start + 1 + cur_line,
                 &format!("{:04X} {} {}", addr, if !instr.is_valid() { '!' } else { ' ' }, instr.display(registers)));
        cur_line += 1;

        if instr.data_length() != 0 {
            print_xy(x_start,
                     y_start + 1 + cur_line,
                     &format!("{:04X} D {:#0w$X}",
                              addr.wrapping_add(instr.data_length() as u16),
                              data,
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