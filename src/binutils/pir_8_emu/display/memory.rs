use self::super::super::super::super::isa::SpecialPurposeRegister;
use self::super::super::super::super::vm::Memory;
use bear_lib_terminal::terminal::print_xy;
use self::super::mem_ports_rw_update;


/// ```plaintext
/// Memory access
/// 01234 ≡ 0xAA
/// ```
pub fn rw_write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "Memory access");
    print_xy(x_start, y_start + 1, "{none}");
}

pub fn rw_update(x_start: usize, y_start: usize, mem: &mut Memory) {
    mem_ports_rw_update(x_start, y_start, mem.iter_rw());
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
