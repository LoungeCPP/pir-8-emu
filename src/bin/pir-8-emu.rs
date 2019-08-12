extern crate bear_lib_terminal;
extern crate tinyfiledialogs;
extern crate pir_8_emu;

use bear_lib_terminal::terminal::{self, KeyCode, Event};
use tinyfiledialogs::open_file_dialog;
use bear_lib_terminal::Color;
use std::process::exit;
use std::{env, fs};


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let opts = pir_8_emu::options::EmulatorOptions::parse();

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


    let mut config = match pir_8_emu::binutils::pir_8_emu::ExecutionConfig::read_from_config_dir(&opts.config_dir.1) {
        Ok(Some(cfg)) => {
            terminal::print_xy(0, 0, "Loaded configuration from");
            terminal::print_xy(25 + 1, 0, &opts.config_dir.0);

            cfg
        }
        Ok(None) => {
            terminal::print_xy(0, 0, "This looks like the first start-up; press [bkcolor=darker grey]F1[/bkcolor] for help");

            pir_8_emu::binutils::pir_8_emu::ExecutionConfig::new()
        }
        Err(err) => {
            let (var, err_s) = match err {
                Ok(ioe) => ("read", ioe.to_string()),
                Err(te) => ("load", te.to_string()),
            };

            terminal::print_xy(0, 0, "Failed to");
            terminal::print_xy(9 + 1, 0, var);
            terminal::print_xy(9 + 1 + 4 + 1, 0, "configuration:");
            terminal::print_xy(9 + 1 + 4 + 1 + 14 + 1, 0, &err_s);

            pir_8_emu::binutils::pir_8_emu::ExecutionConfig::new()
        }
    };
    let _config_destructor = pir_8_emu::binutils::pir_8_emu::QuickscopeWrapper(Some({
        let config = &config as *const pir_8_emu::binutils::pir_8_emu::ExecutionConfig;
        let opts = &opts;
        move || {
            let config = unsafe { *config };
            if let Err(err) = config.write_to_config_dir(&opts.config_dir.1) {
                eprintln!("warning: failed to save config {:?} to {}: {}", config, opts.config_dir.0, err);
            }
        }
    }));

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

    let write_main_screen = |vm: &mut pir_8_emu::binutils::pir_8_emu::Vm| {
        pir_8_emu::binutils::pir_8_emu::display::register::gp_write(0, 1, &mut vm.registers);
        pir_8_emu::binutils::pir_8_emu::display::register::sp_write(0, 5, &mut vm.pc, &mut vm.sp, &mut vm.adr, &mut vm.ins);
        pir_8_emu::binutils::pir_8_emu::display::instruction_write(0, 9);
        pir_8_emu::binutils::pir_8_emu::display::micro::stack::write(0, 12);
        pir_8_emu::binutils::pir_8_emu::display::micro::ops::write(0, 15);
        pir_8_emu::binutils::pir_8_emu::display::micro::ops::new(0, 15, &vm.ops, &vm.registers);

        pir_8_emu::binutils::pir_8_emu::display::instruction_history_write(30, 1);
    };

    write_main_screen(&mut vm);
    terminal::refresh();

    let mut showing_help = false;

    for ev in terminal::events() {
        let mut new_ops = false;
        match ev {
            Event::Close |
            Event::KeyPressed { key: KeyCode::C, ctrl: true, .. } => break,
            Event::KeyPressed { key: KeyCode::Escape, .. } if showing_help => {
                showing_help = false;

                terminal::clear(None);
                write_main_screen(&mut vm);
            }
            Event::KeyPressed { key: KeyCode::F1, .. } => {
                showing_help = true;

                terminal::clear(None);
                terminal::print_xy(0, 0, pir_8_emu::binutils::pir_8_emu::HELP_TEXT);
                terminal::refresh();
            }
            Event::KeyPressed { key: KeyCode::A, ctrl: true, shift: true } if !showing_help => {
                config.auto_load_next_instruction = !config.auto_load_next_instruction;

                pir_8_emu::binutils::pir_8_emu::display::config(0, 0, "Auto load next instruction", config.auto_load_next_instruction);

                new_ops |= flush_instruction_load(&mut vm, &config)?;
            }
            Event::KeyPressed { key: KeyCode::F, ctrl: true, shift: true } if !showing_help => {
                config.execute_full_instructions = !config.execute_full_instructions;

                pir_8_emu::binutils::pir_8_emu::display::config(0, 0, "Execute full instructions", config.execute_full_instructions);
            }
            Event::KeyPressed { key: KeyCode::O, ctrl: true, .. } if !showing_help => {
                if let Some(fname) = open_file_dialog("Open memory image", "", Some((&["*.p8b", "*.bin"], "Memory image files (*.p8b, *.bin)"))) {
                    match fs::read(&fname) {
                        Ok(mem) => {
                            if !terminal::set(terminal::config::Window::empty().title(format!("pir-8-emu – {}", fname))) {
                                eprintln!("warning: failed to set window title for loaded memory image at {}", fname);
                            }

                            vm.reset(&mem);
                            new_ops |= flush_instruction_load(&mut vm, &config)?;
                        }
                        Err(err) => eprintln!("warning: failed to read memory image from {}: {}", fname, err),
                    }
                }
            }
            Event::KeyPressed { key: KeyCode::Space, .. } if !showing_help => {
                if config.execute_full_instructions && vm.instruction_valid {
                    for _ in vm.curr_op..vm.ops.1 {
                        new_ops |= vm.perform_next_op().map_err(|err| vm_perform_err(err, &mut vm))?;
                    }
                } else {
                    new_ops |= vm.perform_next_op().map_err(|err| vm_perform_err(err, &mut vm))?;
                }
                new_ops |= flush_instruction_load(&mut vm, &config)?;
            }
            _ => {}
        }

        if !showing_help {
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

            pir_8_emu::binutils::pir_8_emu::display::instruction_history_update(30,
                                                                                1,
                                                                                &vm.instruction_history,
                                                                                vm.instruction_history.capacity(),
                                                                                &vm.registers);

            terminal::refresh();
        }
    }


    Ok(())
}
