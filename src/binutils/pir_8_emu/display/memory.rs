//! Memory view and update window handling


use self::super::super::super::super::isa::SpecialPurposeRegister;
use self::super::super::super::super::vm::Memory;
use bear_lib_terminal::terminal::print_xy;
use self::super::mem_ports_rw_update;


/// Prepare the "Memory access" window at the specified coords
///
/// The window is `13x6`, laid out as follows:
///
/// ```plaintext
/// Memory access
/// <mem_acts>
/// ```
///
/// Where `<mem_acts>` is either `{none}`, or up to 5 lines in the following format:
///
/// ```plaintext
/// 0000 ← 0x10
/// 0001 → 0x11
/// 0002 ≡ 0x12
/// ```
///
/// Where memory at `0x0000` was written to and has value `0x10`,
///       memory at `0x0001` was read from and has value `0x11`, and
///       memory at `0x0002` was read from and written to and has value `0x12`,
pub fn rw_write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Memory access");
    print_xy(x_start, y_start + 1, "{none}");
}

/// Update the "Memory access" window
///
/// See [`rw_write()`](fn.rw_write.html) for more info
pub fn rw_update(x_start: usize, y_start: usize, mem: &mut Memory) {
    mem_ports_rw_update(x_start, y_start, mem.iter_rw());
    mem.reset_rw();
}

/// Prepare the "Memory view" window at the specified coords
///
/// The window is `21x11`, laid out as follows:
///
/// ```plaintext
/// Memory view
/// FFFC 0x00 0b0000_0000
/// FFFD 0x00 0b0000_0000
/// FFFE 0x00 0b0000_0000
/// FFFF 0x00 0b0000_0000
/// 0000 0x00 0b0000_0000
/// 0001 0x00 0b0000_0000
/// 0002 0x00 0b0000_0000
/// 0003 0x00 0b0000_0000
/// 0004 0x00 0b0000_0000
/// 0005 0x00 0b0000_0000
/// ```
///
/// Where all memory is `0x00`, and `ADR` is `0x0000`
pub fn view_write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Memory view");
}

/// Update the "Memory view" window
///
/// See [`view_write()`](fn.view_write.html) for more info
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
