extern crate bear_lib_terminal;
extern crate pir_8_emu;
extern crate nfd;

use nfd::{Response as OpenFileResponse, open_file_dialog};
use bear_lib_terminal::terminal::{self, KeyCode, Event};
use bear_lib_terminal::geometry::Rect;
use bear_lib_terminal::Color;
use std::process::exit;
use std::{env, fs};


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    terminal::open("pir-8-emu", 80, 24);
    let _bear_lib_terminal_destructor = pir_8_emu::binutils::pir_8_emu::QuickscopeWrapper(Some(|| terminal::close()));

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

    let _icon_path_destructor = pir_8_emu::binutils::pir_8_emu::QuickscopeWrapper(Some(move || if let Some(icon_path) = icon_path {
        if let Err(err) = fs::remove_file(&icon_path) {
            eprintln!("warning: failed to remove temporary icon file {}: {}", icon_path.display(), err);
        }
    }));


    let mut config = pir_8_emu::binutils::pir_8_emu::ExecutionConfig::new();
    let mut vm = pir_8_emu::binutils::pir_8_emu::Vm::new();
    let vm_perform_err = |err: pir_8_emu::micro::MicroOpPerformError, vm: &mut pir_8_emu::binutils::pir_8_emu::Vm| {
        eprintln!("error: failed to perform micro-op: {}", err);
        eprintln!("VM state follows");
        eprintln!("{:?}", vm);
        1
    };
    let flush_instruction_load = |vm: &mut pir_8_emu::binutils::pir_8_emu::Vm, config: &pir_8_emu::binutils::pir_8_emu::ExecutionConfig| -> Result<bool, i32> {
        let mut new_ops = false;
        if config.auto_load_next_instruction {
            while !vm.instruction_valid && !vm.execution_finished {
                new_ops = vm.perform_next_op().map_err(|err| vm_perform_err(err, vm))?;
            }
        }
        Ok(new_ops)
    };

    pir_8_emu::binutils::pir_8_emu::display::register::gp_write(0, 1, &mut vm.registers);
    pir_8_emu::binutils::pir_8_emu::display::register::sp_write(0, 5, &mut vm.pc, &mut vm.sp, &mut vm.adr, &mut vm.ins);
    pir_8_emu::binutils::pir_8_emu::display::instruction_write(0, 9);
    pir_8_emu::binutils::pir_8_emu::display::micro::stack::write(0, 12);
    pir_8_emu::binutils::pir_8_emu::display::micro::ops::write(0, 15);
    terminal::refresh();

    pir_8_emu::binutils::pir_8_emu::display::micro::ops::new(0, 15, &vm.ops, &vm.registers);
    for ev in terminal::events() {
        let mut new_ops = false;
        match ev {
            Event::Close |
            Event::KeyPressed { key: KeyCode::C, ctrl: true, .. } => break,
            Event::KeyPressed { key: KeyCode::A, ctrl: true, shift: true } => {
                config.auto_load_next_instruction = !config.auto_load_next_instruction;

                terminal::clear(Some(Rect::from_values(0, 0, 80, 1)));
                terminal::print_xy(0, 0, "Auto load next instruction:");
                terminal::print_xy(27 + 1,
                                   0,
                                   if config.auto_load_next_instruction {
                                       "ON"
                                   } else {
                                       "OFF"
                                   });

                new_ops = flush_instruction_load(&mut vm, &config)?;
            }
            Event::KeyPressed { key: KeyCode::O, ctrl: true, .. } => {
                match open_file_dialog(Some("p8b,bin"), None) {
                    Ok(OpenFileResponse::Okay(fname)) => {
                        match fs::read(&fname) {
                            Ok(mem) => {
                                if !terminal::set(terminal::config::Window::empty().title(format!("pir-8-emu – {}", fname))) {
                                    eprintln!("warning: failed to set window title for loaded memory image at {}", fname);
                                }

                                vm.reset(&mem);
                            }
                            Err(err) => eprintln!("warning: failed to read memory image from {}: {}", fname, err),
                        }
                    }
                    Ok(OpenFileResponse::OkayMultiple(_)) => unreachable!(),
                    Ok(OpenFileResponse::Cancel) => {}
                    Err(err) => eprintln!("warning: failed to open file open dialog: {}", err),
                }
            }
            Event::KeyPressed { key: KeyCode::Space, .. } => {
                new_ops = vm.perform_next_op().map_err(|err| vm_perform_err(err, &mut vm))?;
                new_ops |= flush_instruction_load(&mut vm, &config)?;
            }
            _ => {}
        }

        pir_8_emu::binutils::pir_8_emu::display::register::gp_update(0, 1, &mut vm.registers);
        pir_8_emu::binutils::pir_8_emu::display::register::sp_update(0, 5, &mut vm.pc, &mut vm.sp, &mut vm.adr, &mut vm.ins);
        if new_ops {
            pir_8_emu::binutils::pir_8_emu::display::instruction_update(0, 9, vm.instruction_valid, vm.execution_finished, &vm.instruction, &vm.registers);
        }
        pir_8_emu::binutils::pir_8_emu::display::micro::stack::update(0, 12, &vm.stack);
        if vm.execution_finished {
            pir_8_emu::binutils::pir_8_emu::display::micro::ops::finished(0, 15);
        } else if new_ops || vm.curr_op == 0 {
            pir_8_emu::binutils::pir_8_emu::display::micro::ops::new(0, 15, &vm.ops, &vm.registers);
        } else {
            pir_8_emu::binutils::pir_8_emu::display::micro::ops::update(0, 15, vm.curr_op);
        }

        terminal::refresh();
    }


    Ok(())
}
