use self::super::super::super::super::isa::{GeneralPurposeRegisterBank, SpecialPurposeRegister};
use bear_lib_terminal::terminal::{with_colors, print_xy, put_xy};
use self::super::super::super::super::rw::ReadWritable;
use num_traits::{Unsigned, PrimInt, Num};
use self::super::colours_for_rw;
use std::fmt::UpperHex;
use std::mem::size_of;


/// ```plaintext
///  General-purpose registers
/// F 0x00 S 0x00 X 0x00 Y 0x00
/// ```
pub fn gp_write(x_start: usize, y_start: usize, registers: &mut GeneralPurposeRegisterBank) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start + 1, y_start, "General-purpose registers");
    for reg in registers {
        let y_offset = reg.address() / 4;
        let x_offset = (reg.address() % 4) * (1 + 1 + 2 + 2 + 1);

        print_xy(x_start + x_offset as i32,
                 y_start + 1 + y_offset as i32,
                 &format!("{} {:#04X}", reg.letter(), **reg));

        reg.reset_rw();
    }
}

pub fn gp_update(x_start: usize, y_start: usize, registers: &mut GeneralPurposeRegisterBank) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    for reg in registers {
        let y_offset = reg.address() / 4;
        let x_offset = (reg.address() % 4) * (1 + 1 + 2 + 2 + 1);

        let r = reg.was_read();
        let w = reg.was_written();

        let (fg, bg) = colours_for_rw(r, w);
        with_colors(fg, bg, || put_xy(x_start + x_offset as i32, y_start + 1 + y_offset as i32, reg.letter()));

        if w {
            print_xy(x_start + x_offset as i32 + 1 + 1, y_start + 1 + y_offset as i32, &format!("{:#04X}", **reg))
        }

        reg.reset_rw();
    }
}


/// ```plaintext
///  Special-purpose registers
/// PC  0x0000       SP  0x0000
/// ADR 0x0000       INS 0x00
/// ```
pub fn sp_write(x_start: usize, y_start: usize, pc: &mut SpecialPurposeRegister<u16>, sp: &mut SpecialPurposeRegister<u16>,
                          adr: &mut SpecialPurposeRegister<u16>, ins: &mut SpecialPurposeRegister<u8>) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start + 1, y_start, "Special-purpose registers");
    write_sp_register(x_start, y_start + 1, pc);
    write_sp_register(x_start + 3 + 1 + 6 + 7, y_start + 1, sp);
    write_sp_register(x_start, y_start + 2, adr);
    write_sp_register(x_start + 3 + 1 + 6 + 7, y_start + 2, ins);
}

fn write_sp_register<T: Num + Unsigned + PrimInt + UpperHex>(x: i32, y: i32, reg: &mut SpecialPurposeRegister<T>) {
    print_xy(x as i32,
             y as i32,
             &format!("{}{:sw$} {:#0dw$X}",
                      reg.short_name(),
                      "",
                      **reg,
                      sw = 3 - reg.short_name().len().min(3),
                      dw = 2 + size_of::<T>() * 2));

    reg.reset_rw();
}

pub fn sp_update(x_start: usize, y_start: usize, pc: &mut SpecialPurposeRegister<u16>, sp: &mut SpecialPurposeRegister<u16>,
                           adr: &mut SpecialPurposeRegister<u16>, ins: &mut SpecialPurposeRegister<u8>) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    update_sp_register(x_start, y_start + 1, pc);
    update_sp_register(x_start + 3 + 1 + 6 + 7, y_start + 1, sp);
    update_sp_register(x_start, y_start + 2, adr);
    update_sp_register(x_start + 3 + 1 + 6 + 7, y_start + 2, ins);
}

fn update_sp_register<T: Num + Unsigned + PrimInt + UpperHex>(x: i32, y: i32, reg: &mut SpecialPurposeRegister<T>) {
    let r = reg.was_read();
    let w = reg.was_written();

    let (fg, bg) = colours_for_rw(r, w);
    with_colors(fg, bg, || print_xy(x, y, reg.short_name()));

    if w {
        print_xy(x + 3 + 1, y, &format!("{:#0dw$X}", **reg, dw = 2 + size_of::<T>() * 2))
    }

    reg.reset_rw();
}
