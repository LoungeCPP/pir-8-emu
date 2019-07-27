extern crate bear_lib_terminal;
extern crate pir_8_emu;
extern crate nfd;

use nfd::{Response as OpenFileResponse, open_file_dialog};
use bear_lib_terminal::terminal::{self, KeyCode, Event};
use bear_lib_terminal::Color;
use pir_8_emu::ReadWritable;
use std::process::exit;
use std::{env, fs};


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = (pir_8_emu::vm::Memory::new(),
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
    pir_8_emu::binutils::pir_8_emu::display::instruction_write(0, 9);
    pir_8_emu::binutils::pir_8_emu::display::micro::stack::write(0, 12);
    pir_8_emu::binutils::pir_8_emu::display::micro::ops::write(0, 15);
    terminal::refresh();


    let mut ops = pir_8_emu::micro::NEXT_INSTRUCTION;
    let mut curr_op = 0;

    let mut instr = pir_8_emu::isa::instruction::Instruction::Halt;
    let mut instr_valid = false;

    let mut stack = vec![];

    pir_8_emu::binutils::pir_8_emu::display::micro::ops::new(0, 15, &ops, &registers);
    for ev in terminal::events() {
        let mut new_ops = false;
        match ev {
            Event::Close |
            Event::KeyPressed { key: KeyCode::C, ctrl: true, .. } => break,
            Event::KeyPressed { key: KeyCode::O, ctrl: true, .. } => {
                match open_file_dialog(Some("p8b,bin"), None) {
                    Ok(OpenFileResponse::Okay(fname)) => {
                        match fs::read(&fname) {
                            Ok(mem) => {
                                if !terminal::set(terminal::config::Window::empty().title(format!("pir-8-emu - {}", fname))) {
                                    eprintln!("warning: failed to set window title for loaded memory image at {}", fname);
                                }

                                memory = pir_8_emu::vm::Memory::from(&mem[..]);

                                ports = pir_8_emu::vm::Ports::new();
                                registers = pir_8_emu::isa::GeneralPurposeRegister::defaults();
                                pc = pir_8_emu::isa::SpecialPurposeRegister::new("Program Counter", "PC");
                                sp = pir_8_emu::isa::SpecialPurposeRegister::new("Stack Pointer", "SP");
                                adr = pir_8_emu::isa::SpecialPurposeRegister::new("Memory Address", "ADR");
                                ins = pir_8_emu::isa::SpecialPurposeRegister::new("Instruction", "INS");

                                ops = pir_8_emu::micro::NEXT_INSTRUCTION;
                                curr_op = 0;
                                instr_valid = false;
                                stack.clear();
                            }
                            Err(err) => eprintln!("error: failed to read memory image from {}: {}", fname, err),
                        }
                    }
                    Ok(OpenFileResponse::OkayMultiple(_)) => unreachable!(),
                    Ok(OpenFileResponse::Cancel) => {}
                    Err(err) => eprintln!("error: failed to open file open dialog: {}", err),
                }
            }
            Event::KeyPressed { key: KeyCode::Space, .. } => {
                if !ops.0[curr_op].perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins).unwrap() {
                    break;
                }
                curr_op += 1;

                if curr_op >= ops.1 {
                    if ins.was_written() {
                        instr = pir_8_emu::isa::instruction::Instruction::from(*ins);
                        ops = pir_8_emu::micro::MicroOp::from_instruction(instr);
                        instr_valid = true;
                    } else {
                        ops = pir_8_emu::micro::NEXT_INSTRUCTION;
                        instr_valid = false;
                    }

                    curr_op = 0;
                    new_ops = true;
                }
            }
            _ => {}
        }

        pir_8_emu::binutils::pir_8_emu::display::register::gp_update(0, 1, &mut registers);
        pir_8_emu::binutils::pir_8_emu::display::register::sp_update(0, 5, &mut pc, &mut sp, &mut adr, &mut ins);
        pir_8_emu::binutils::pir_8_emu::display::instruction_update(0, 9, instr_valid, &instr, &registers);
        pir_8_emu::binutils::pir_8_emu::display::micro::stack::update(0, 12, &stack);
        if new_ops || curr_op == 0 {
            pir_8_emu::binutils::pir_8_emu::display::micro::ops::new(0, 15, &ops, &registers);
        } else {
            pir_8_emu::binutils::pir_8_emu::display::micro::ops::update(0, 15, curr_op);
        }
        terminal::print_xy(0, 0, &format!("{:?}", ev));

        terminal::refresh();
    }

    terminal::refresh();
    terminal::delay(1000);
    terminal::close();

    if let Some(icon_path) = icon_path {
        if let Err(err) = fs::remove_file(&icon_path) {
            eprintln!("warning: failed to remove temporary icon file {}: {}", icon_path.display(), err);
        }
    }

    Ok(())
}
