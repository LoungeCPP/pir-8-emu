//! μOp window handling


use self::super::super::super::super::super::isa::GeneralPurposeRegisterBank;
use self::super::super::super::super::super::micro::{MicroOpBlock, MicroOp};
use bear_lib_terminal::terminal::{with_foreground, print_xy, put_xy, clear};
use bear_lib_terminal::geometry::Rect;
use self::super::super::COLUMN_WIDTH;
use bear_lib_terminal::Color;
use std::mem::size_of;


const MAX_HEIGHT: usize = size_of::<MicroOpBlock>() / size_of::<MicroOp>();


/// Prepare the "Current μOps" window at the specified coords
///
/// The window is `27x(1+H)`, laid out as follows:
///
/// ```plaintext
/// Current μOps
/// <ops>
/// ```
///
/// Where `<ops>` is `{execution finished}` or between 1 and `H` μOps,
/// and `H` is the length of [`MicroOpBlock`](../../../../../micro/type.MicroOpBlock.html)
pub fn write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Current μOps");
}

/// Update the "Current μOps" window for new ops
///
/// See [`write()`](fn.write.html) for more info
pub fn new(x_start: usize, y_start: usize, ops: &(MicroOpBlock, usize), registers: &GeneralPurposeRegisterBank, breakpoint_active: bool) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    clear(Some(Rect::from_values(x_start, y_start + 1, COLUMN_WIDTH, MAX_HEIGHT as i32)));

    red_if_breakpoint(breakpoint_active,||put_xy(x_start, y_start + 1, '>'));

    for i in 0..ops.1 {
        print_xy(x_start + 1, y_start + 1 + i as i32, &ops.0[i].display(registers).to_string());
    }
}

/// Update the "Current μOps" window when going from op to op
///
/// See [`write()`](fn.write.html) for more info
pub fn update(x_start: usize, y_start: usize, current_op: usize, breakpoint_active: bool) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;
    let current_op = current_op as i32;

    clear(Some(Rect::from_values(x_start, y_start + 1 + current_op - 1, 1, 1)));
    red_if_breakpoint(breakpoint_active,||put_xy(x_start, y_start + 1 + current_op, '>'));
}

/// Update the "Current μOps" window when execution was finished
///
/// See [`write()`](fn.write.html) for more info
pub fn finished(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    clear(Some(Rect::from_values(x_start, y_start + 1, COLUMN_WIDTH, MAX_HEIGHT as i32)));

    print_xy(x_start, y_start + 1, "{execution finished}");
}

fn red_if_breakpoint<F: FnOnce()>(breakpoint_active: bool, write: F) {
    if breakpoint_active {
        with_foreground(Color::from_rgb(0xFF, 0, 0), write);
    } else {
        write();
    }
}
