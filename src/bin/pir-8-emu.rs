extern crate bear_lib_terminal;
extern crate pir_8_emu;

use bear_lib_terminal::terminal::{self, KeyCode, Event};
use bear_lib_terminal::Color;
use std::process::exit;
use std::{env, fs};


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let (_, _, mut registers, mut pc, mut sp, mut adr, mut ins) = (pir_8_emu::vm::Memory::new(),
                                                                   pir_8_emu::vm::Ports::new(),
                                                                   pir_8_emu::isa::GeneralPurposeRegister::defaults(),
                                                                   pir_8_emu::isa::SpecialPurposeRegister::new("Program Counter", "PC"),
                                                                   pir_8_emu::isa::SpecialPurposeRegister::new("Stack Pointer", "SP"),
                                                                   pir_8_emu::isa::SpecialPurposeRegister::new("Memory Address", "ADR"),
                                                                   pir_8_emu::isa::SpecialPurposeRegister::new("Instruction", "INS"));

    terminal::open("pir-8-emu", 80, 24);
    terminal::set_colors(Color::from_rgb(0xFF, 0xFF, 0xFF), Color::from_rgb(0x00, 0x00, 0x00));

    let icon_path = env::temp_dir().join("pir-8-emu.ico");
    let icon_path = if let Err(err) = fs::write(&icon_path, pir_8_emu::binutils::pir_8_emu::ICON) {
        eprintln!("warning: failed to write window icon to temporary file {}: {}", icon_path.display(), err);
        None
    } else if !terminal::set(terminal::config::Window::empty().icon(&icon_path)) {
        eprintln!("warning: failed to set window icon to temporary file");
        None
    } else {
        Some(icon_path)
    };


    pir_8_emu::binutils::pir_8_emu::display::register::gp_write(0, 1, &mut registers);
    pir_8_emu::binutils::pir_8_emu::display::register::sp_write(0, 5, &mut pc, &mut sp, &mut adr, &mut ins);
    terminal::refresh();

    for ev in terminal::events() {
        match ev {
            Event::Close |
            Event::KeyPressed { key: KeyCode::C, ctrl: true, .. } => break,
            _ => {}
        }

        *registers[1] = *registers[0] + 12;
        *registers[2] = *registers[1] + 0x69;
        *sp = *pc + 12;
        *adr = *sp + 0x69;
        pir_8_emu::binutils::pir_8_emu::display::register::gp_update(0, 1, &mut registers);
        pir_8_emu::binutils::pir_8_emu::display::register::sp_update(0, 5, &mut pc, &mut sp, &mut adr, &mut ins);
        terminal::print_xy(0, 0, &format!("{:?}", ev));

        terminal::refresh();
    }

    terminal::close();

    if let Some(icon_path) = icon_path {
        if let Err(err) = fs::remove_file(&icon_path) {
            eprintln!("warning: failed to remove temporary icon file {}: {}", icon_path.display(), err);
        }
    }

    Ok(())
}
