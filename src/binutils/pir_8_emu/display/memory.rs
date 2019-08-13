use self::super::super::super::super::isa::SpecialPurposeRegister;
use bear_lib_terminal::terminal::{with_colors, print_xy, clear};
use self::super::super::super::super::vm::Memory;
use bear_lib_terminal::geometry::Rect;
use self::super::colours_for_rw;


/// ```plaintext
/// Memory access
/// 01234 = 0xAA
/// ```
pub fn rw_write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Memory access");
    print_xy(x_start, y_start + 1, "{none}");
}

pub fn rw_update(x_start: usize, y_start: usize, mem: &mut Memory) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    let mut cur_line = 0;
    for (addr, val, r, w) in mem.iter_rw() {
        let (fg, bg) = colours_for_rw(r, w);
        with_colors(fg, bg, || print_xy(x_start, y_start + 1 + cur_line, &format!("{:04X}", addr)));

        print_xy(x_start + 4, y_start + 1 + cur_line, &format!(" = {:#04X}", val));

        cur_line += 1;
    }

    if cur_line == 0 {
        print_xy(x_start, y_start + 1, "{none}      ");
        cur_line += 1;
    }

    if cur_line < 5 {
        clear(Some(Rect::from_values(x_start, y_start + 1 + cur_line, 4 + 3 + 4, 5 - cur_line)));
    }

    mem.reset_rw();
}

pub fn view_write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Memory view");
}

pub fn view_update(x_start: usize, y_start: usize, adr: SpecialPurposeRegister<u16>, mem: &Memory) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;
    let adr = *adr;

    for cur_line in 0..10 {
        let addr = adr.wrapping_add(cur_line as u16).wrapping_sub(4);
        let val = mem[..][addr as usize];

        let (clr_start, clr_end) = if addr == adr {
            ("[bkcolor=darker grey]", "[/bkcolor]")
        } else {
            ("", "")
        };

        print_xy(x_start,
                 y_start + 1 + cur_line,
                 &format!("{}{:04X} {:#04X} {:#06b}_{:04b}{}", clr_start, addr, val, val >> 4, val & 0b1111, clr_end));
    }
}
