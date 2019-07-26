use self::super::super::super::super::super::isa::GeneralPurposeRegisterBank;
use self::super::super::super::super::super::micro::{MicroOpBlock, MicroOp};
use bear_lib_terminal::terminal::{print_xy, put_xy, clear};
use bear_lib_terminal::geometry::Rect;
use self::super::super::COLUMN_WIDTH;
use std::mem::size_of;


const MAX_HEIGHT: usize = size_of::<MicroOpBlock>() / size_of::<MicroOp>();


pub fn write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Current μOps");
}

pub fn new(x_start: usize, y_start: usize, ops: &(MicroOpBlock, usize), registers: &GeneralPurposeRegisterBank) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    clear(Some(Rect::from_values(x_start, y_start + 1, COLUMN_WIDTH, MAX_HEIGHT as i32)));

    put_xy(x_start, y_start + 1, '>');

    for i in 0..ops.1 {
        print_xy(x_start + 1, y_start + 1 + i as i32, &ops.0[i].display(registers).to_string());
    }
}

pub fn update(x_start: usize, y_start: usize, current_op: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;
    let current_op = current_op as i32;

    clear(Some(Rect::from_values(x_start, y_start + 1 + current_op - 1, 1, 1)));
    put_xy(x_start, y_start + 1 + current_op, '>');
}
