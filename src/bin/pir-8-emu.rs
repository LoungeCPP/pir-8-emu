extern crate bear_lib_terminal;
extern crate tinyfiledialogs;
extern crate arraydeque;
extern crate pir_8_emu;
extern crate dlopen;
extern crate time;

use arraydeque::{ArrayDeque, Wrapping as ArrayDequeBehaviourWrapping};
use bear_lib_terminal::terminal::{self, KeyCode, Event};
use dlopen::utils::PLATFORM_FILE_EXTENSION;
use tinyfiledialogs::open_file_dialog;
use pir_8_emu::vm::PortHandler;
use bear_lib_terminal::Color;
use time::precise_time_ns;
use std::time::Duration;
use std::process::exit;
use std::thread::sleep;
use std::{env, fs};
use std::ops::Add;


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

    let mut vm = pir_8_emu::binutils::pir_8_emu::Vm::new(&config.general_purpose_register_letters.iter().collect::<String>())
        .expect("ExecutionConfig::general_purpose_register_letters is validated");
    let vm_perform_err = |err: pir_8_emu::micro::MicroOpPerformError, vm: &mut pir_8_emu::binutils::pir_8_emu::Vm| {
        eprintln!("error: failed to perform micro-op: {}", err);
        eprintln!("VM state follows");
        eprintln!("{:?}", vm);
        1
    };
    let flush_instruction_load = |vm: &mut pir_8_emu::binutils::pir_8_emu::Vm, config: &pir_8_emu::binutils::pir_8_emu::ExecutionConfig| -> Result<bool, i32> {
        let mut new_ops = false;

        if config.auto_load_next_instruction {
            while !vm.instruction_valid && !vm.execution_finished && !vm.active_breakpoint.is_some() {
                new_ops = vm.perform_next_op().map_err(|err| vm_perform_err(err, vm))?;
            }
        }

        if let Some(breakpoint_addr) = vm.active_breakpoint {
            pir_8_emu::binutils::pir_8_emu::display::status::line(0, 0, "Hit breakpoint", &format!("{:#06X}", breakpoint_addr));
        }

        Ok(new_ops)
    };

    let write_main_screen = |vm: &mut pir_8_emu::binutils::pir_8_emu::Vm| {
        pir_8_emu::binutils::pir_8_emu::display::register::gp_write(0, 1, &mut vm.registers);
        pir_8_emu::binutils::pir_8_emu::display::register::sp_write(0, 5, &mut vm.pc, &mut vm.sp, &mut vm.adr, &mut vm.ins);
        pir_8_emu::binutils::pir_8_emu::display::instruction_write(0, 9);
        pir_8_emu::binutils::pir_8_emu::display::micro::stack::write(0, 12);
        pir_8_emu::binutils::pir_8_emu::display::micro::ops::write(0, 15);
        pir_8_emu::binutils::pir_8_emu::display::micro::ops::new(0, 15, &vm.ops, &vm.registers, vm.active_breakpoint.is_some());

        pir_8_emu::binutils::pir_8_emu::display::instruction_history_write(30, 1);
        pir_8_emu::binutils::pir_8_emu::display::ports_rw_write(30, 13);

        pir_8_emu::binutils::pir_8_emu::display::memory::view_write(30 + 25, 1);
        pir_8_emu::binutils::pir_8_emu::display::memory::view_update(30 + 25, 1, vm.adr, &vm.memory);

        pir_8_emu::binutils::pir_8_emu::display::memory::rw_write(30 + 25, 13);
    };

    let update_main_screen = |vm: &mut pir_8_emu::binutils::pir_8_emu::Vm, new_ops| {
        pir_8_emu::binutils::pir_8_emu::display::register::gp_update(0, 1, &mut vm.registers);
        pir_8_emu::binutils::pir_8_emu::display::register::sp_update(0, 5, &mut vm.pc, &mut vm.sp, &mut vm.adr, &mut vm.ins);
        if new_ops {
            pir_8_emu::binutils::pir_8_emu::display::instruction_update(0, 9, vm.instruction_valid, vm.execution_finished, &vm.instruction, &vm.registers);
        }
        pir_8_emu::binutils::pir_8_emu::display::micro::stack::update(0, 12, &vm.stack);
        if vm.execution_finished {
            pir_8_emu::binutils::pir_8_emu::display::micro::ops::finished(0, 15);
        } else if new_ops || vm.curr_op == 0 {
            pir_8_emu::binutils::pir_8_emu::display::micro::ops::new(0, 15, &vm.ops, &vm.registers, vm.active_breakpoint.is_some());
        } else {
            pir_8_emu::binutils::pir_8_emu::display::micro::ops::update(0, 15, vm.curr_op, vm.active_breakpoint.is_some());
        }

        pir_8_emu::binutils::pir_8_emu::display::instruction_history_update(30, 1, &vm.instruction_history, vm.instruction_history.capacity(), &vm.registers);
        pir_8_emu::binutils::pir_8_emu::display::ports_rw_update(30, 13, &mut vm.ports);

        pir_8_emu::binutils::pir_8_emu::display::memory::view_update(30 + 25, 1, vm.adr, &vm.memory);
        pir_8_emu::binutils::pir_8_emu::display::memory::rw_update(30 + 25, 13, &mut vm.memory);
    };

    write_main_screen(&mut vm);
    terminal::refresh();

    let step = |vm: &mut pir_8_emu::binutils::pir_8_emu::Vm, config: &pir_8_emu::binutils::pir_8_emu::ExecutionConfig| -> Result<bool, i32> {
        let mut new_ops = false;

        if config.execute_full_instructions && vm.instruction_valid {
            for _ in vm.curr_op..vm.ops.1 {
                new_ops |= vm.perform_next_op().map_err(|err| vm_perform_err(err, vm))?;
                if vm.active_breakpoint.is_some() {
                    break;
                }
            }
        } else {
            new_ops |= vm.perform_next_op().map_err(|err| vm_perform_err(err, vm))?;
        }
        new_ops |= flush_instruction_load(vm, &config)?;

        Ok(new_ops)
    };

    let mut help_page = None;
    let mut current_memory_image = None;

    for ev in terminal::events() {
        let showing_help = help_page.is_some();
        let mut new_ops = false;
        match ev {
            Event::Close |
            Event::KeyPressed { key: KeyCode::C, ctrl: true, .. } => break,
            Event::KeyPressed { key: KeyCode::Escape, .. } if showing_help => {
                help_page = None;

                terminal::clear(None);
                write_main_screen(&mut vm);
            }
            Event::KeyPressed { key: KeyCode::Escape, .. } if !showing_help => {
                if let Some(breakpoint_addr) = vm.active_breakpoint.take() {
                    pir_8_emu::binutils::pir_8_emu::display::status::line(0, 0, "Cleared breakpoint", &format!("{:#06X}", breakpoint_addr));
                }

                new_ops |= flush_instruction_load(&mut vm, &config)?;
            }
            Event::KeyPressed { key: KeyCode::F1, .. } => {
                help_page = Some(0);
            }
            Event::KeyPressed { key: KeyCode::A, ctrl: true, shift: true } if !showing_help => {
                config.auto_load_next_instruction = !config.auto_load_next_instruction;

                pir_8_emu::binutils::pir_8_emu::display::status::config_bool(0, 0, "Auto load next instruction", config.auto_load_next_instruction);

                new_ops |= flush_instruction_load(&mut vm, &config)?;
            }
            Event::KeyPressed { key: KeyCode::F, ctrl: true, shift: true } if !showing_help => {
                config.execute_full_instructions = !config.execute_full_instructions;

                pir_8_emu::binutils::pir_8_emu::display::status::config_bool(0, 0, "Execute full instructions", config.execute_full_instructions);
            }
            Event::KeyPressed { key: KeyCode::R, ctrl: true, shift: true } if !showing_help => {
                if let Some(letters) = pir_8_emu::binutils::pir_8_emu::display::status::read_gp_register_letters(0, 0) {
                    config.general_purpose_register_letters = letters;

                    for (reg, &ltr) in vm.registers.iter_mut().zip(config.general_purpose_register_letters.iter()) {
                        reg.relabel(ltr).expect("ExecutionConfig::general_purpose_register_letters is validated");
                    }

                    pir_8_emu::binutils::pir_8_emu::display::register::gp_write(0, 1, &mut vm.registers);
                }
            }
            Event::KeyPressed { key: KeyCode::O, ctrl: true, .. } if !showing_help => {
                if let Some(fname) = open_file_dialog("Open memory image", "", Some((&["*.p8b", "*.bin"], "Memory image files (*.p8b, *.bin)"))) {
                    match fs::read(&fname) {
                        Ok(mem) => {
                            println!("status: loaded memory image from {}", fname);

                            if !terminal::set(terminal::config::Window::empty().title(format!("pir-8-emu – {}", fname))) {
                                eprintln!("warning: failed to set window title for loaded memory image at {}", fname);
                            }
                            current_memory_image = Some(fname);

                            vm.reset(&config.general_purpose_register_letters.iter().collect::<String>(), &mem)
                                .expect("ExecutionConfig::general_purpose_register_letters is validated");
                            new_ops |= flush_instruction_load(&mut vm, &config)?;
                        }
                        Err(err) => {
                            pir_8_emu::binutils::pir_8_emu::display::status::line(0, 0, "Memory image load", &err.to_string());
                            eprintln!("warning: failed to read memory image from {}: {}", fname, err);
                        }
                    }
                }
            }
            Event::KeyPressed { key: KeyCode::R, ctrl: true, .. } if !showing_help => {
                if let Some(port) = pir_8_emu::binutils::pir_8_emu::display::status::read_number(0, 0, "Read from port") {
                    let byte = vm.ports.read(port);

                    pir_8_emu::binutils::pir_8_emu::display::status::line(0, 0, &format!("Byte read from port {:02X}", port), &format!("{:#04X}", byte));
                }
            }
            Event::KeyPressed { key: KeyCode::W, ctrl: true, .. } if !showing_help => {
                if let Some(port) = pir_8_emu::binutils::pir_8_emu::display::status::read_number(0, 0, "Write to port") {
                    if let Some(byte) = pir_8_emu::binutils::pir_8_emu::display::status::read_number(0, 0, &format!("Byte to write to port {:02X}", port)) {
                        vm.ports.write(port, byte);
                    }
                }
            }
            Event::KeyPressed { key: KeyCode::I, ctrl: true, .. } if !showing_help => {
                if let Some(fname) = open_file_dialog("Install port handler",
                                                      "",
                                                      Some((&[&format!("*.{}", PLATFORM_FILE_EXTENSION)],
                                                            &format!("Dynamically loaded libraries (*.{})", PLATFORM_FILE_EXTENSION)))) {
                    match pir_8_emu::binutils::pir_8_emu::NativePortHandler::load_from_dll(&fname) {
                        Ok(handler) => {
                            let port_count = handler.port_count().get() as usize;

                            let mut ports = Vec::with_capacity(port_count);
                            for port in 0..port_count {
                                match pir_8_emu::binutils::pir_8_emu::display::status::read_number(0, 0, &format!("Port #{}", port)) {
                                    Some(port) => ports.push(port),
                                    None => break,
                                }
                            }

                            if ports.len() == port_count {
                                match vm.ports.install_handler(handler, &ports) {
                                    Ok(handler_idx) => {
                                        pir_8_emu::binutils::pir_8_emu::display::status::line(0,
                                                                                              0,
                                                                                              "Install port handler",
                                                                                              &format!("ID = {:#06X}", handler_idx));

                                        print!("status: installed {} as handler {:#06X} on port{} ",
                                               fname,
                                               handler_idx,
                                               if ports.len() == 1 { "" } else { "s" });
                                        for (i, port) in ports.into_iter().enumerate() {
                                            if i != 0 {
                                                print!(", ");
                                            }

                                            print!("{:02X}", port);
                                        }
                                        println!();
                                    }
                                    Err((hndl, err)) => {
                                        pir_8_emu::binutils::pir_8_emu::display::status::line(0, 0, "Install port handler", &err.to_string());
                                        eprintln!("warning: failed to install port handler {:?}: {}", hndl.path, err);
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            pir_8_emu::binutils::pir_8_emu::display::status::line(0, 0, "Loading port handler DLL", &err.to_string());
                            eprintln!("warning: failed to load port handler DLL from {}: {}", fname, err);
                        }
                    }
                }
            }
            Event::KeyPressed { key: KeyCode::K, ctrl: true, .. } if !showing_help => {
                if let Some(handler_idx) = pir_8_emu::binutils::pir_8_emu::display::status::read_number(0, 0, "Uninstall port handler") {
                    match vm.ports.uninstall_handler(handler_idx) {
                        Some(handler) => {
                            pir_8_emu::binutils::pir_8_emu::display::status::line(0, 0, "Uninstall port handler", &format!("{:#06X} uninstalled", handler_idx));

                            print!("status: uninstalled handler {:#06X}", handler_idx);
                            if let Ok(handler) = handler.downcast::<pir_8_emu::binutils::pir_8_emu::NativePortHandler>() {
                                print!(" ({:?})", handler.path);
                            }
                            println!();
                        }
                        None => {
                            pir_8_emu::binutils::pir_8_emu::display::status::line(0, 0, "Uninstall port handler", &format!("{:#06X} not found", handler_idx))
                        }
                    }
                }
            }
            Event::KeyPressed { key: KeyCode::B, ctrl: true, .. } if !showing_help => {
                if let Some(addr) = pir_8_emu::binutils::pir_8_emu::display::status::read_number(0, 0, "New breakpoint address") {
                    vm.breakpoints.insert(addr);
                    println!("status: added breakpoint for {:#06X}", addr);
                }
            }
            Event::KeyPressed { key: KeyCode::G, ctrl: true, .. } if !showing_help => {
                if let Some(addr) = pir_8_emu::binutils::pir_8_emu::display::status::read_number(0, 0, "Remove breakpoint for address") {
                    vm.breakpoints.remove(&addr);
                    println!("status: removed breakpoint for {:#06X}", addr);
                }
            }
            Event::KeyPressed { key: KeyCode::U, ctrl: true, .. } if !showing_help => {
                if let Some(addr) = pir_8_emu::binutils::pir_8_emu::display::status::read_number(0, 0, "Update address") {
                    *vm.adr = addr;

                    if let Some(byte) = pir_8_emu::binutils::pir_8_emu::display::status::read_number(0,
                                                                                                     0,
                                                                                                     &format!("Change {:04X} from {:#04X}",
                                                                                                              addr,
                                                                                                              vm.memory[addr])) {
                        vm.memory[addr] = byte;
                    }
                }
            }
            Event::KeyPressed { key: KeyCode::J, ctrl: true, .. } if !showing_help => {
                if let Some(addr) = pir_8_emu::binutils::pir_8_emu::display::status::read_number(0, 0, "Jump to address") {
                    vm.jump_to_addr(addr).map_err(|err| vm_perform_err(err, &mut vm))?;
                    flush_instruction_load(&mut vm, &config)?;
                    new_ops = true;
                }
            }
            Event::KeyPressed { key: KeyCode::Space, shift: true, .. } if !showing_help => {
                if let Some(freq) = pir_8_emu::binutils::pir_8_emu::display::status::read_pos_float(0, 0, "Target step frequency") {
                    println!("status: stepping at target frequency {}", freq);

                    let mut max_count = 0;
                    let mut sub_max_nanos = 1_000_000_000f64 / freq;

                    let target_nanos = sub_max_nanos as u64;

                    if sub_max_nanos > pir_8_emu::binutils::pir_8_emu::MAX_UI_DELAY.as_nanos() as f64 {
                        let cnt = sub_max_nanos / pir_8_emu::binutils::pir_8_emu::MAX_UI_DELAY.as_nanos() as f64;

                        max_count = cnt.floor() as u64;
                        sub_max_nanos = cnt.fract() * pir_8_emu::binutils::pir_8_emu::MAX_UI_DELAY.as_nanos() as f64;
                    }

                    let mut framerate_samples: ArrayDeque<[f64; 10], ArrayDequeBehaviourWrapping> = ArrayDeque::new();

                    let sub_max_nanos = sub_max_nanos as u64;
                    let mut frame_start = precise_time_ns() - target_nanos;
                    let mut refresh_ns = 0u64;

                    'steppy: loop {
                        let before = precise_time_ns();

                        let mut max_count = max_count;
                        let mut sub_max_nanos = sub_max_nanos;

                        while refresh_ns > sub_max_nanos && max_count > 0 {
                            max_count -= 1;
                            refresh_ns -= sub_max_nanos;
                            sub_max_nanos = pir_8_emu::binutils::pir_8_emu::MAX_UI_DELAY.as_nanos() as u64 - sub_max_nanos;
                        }
                        sub_max_nanos = sub_max_nanos.checked_sub(refresh_ns).unwrap_or(0);

                        for _ in 0..max_count {
                            sleep(pir_8_emu::binutils::pir_8_emu::MAX_UI_DELAY);
                            if terminal::has_input() {
                                break 'steppy;
                            }
                        }

                        sleep(Duration::from_nanos(sub_max_nanos));
                        if terminal::has_input() {
                            break 'steppy;
                        }

                        let mut new_ops = false;
                        new_ops |= step(&mut vm, &config)?;
                        update_main_screen(&mut vm, new_ops);

                        framerate_samples.push_back(1_000_000_000f64 / ((before - frame_start) as f64));
                        if !terminal::set(terminal::config::Window::empty().title(format!("pir-8-emu{}{} – {:.2} steps/s",
                                                                                          if current_memory_image.is_some() {
                                                                                              " – "
                                                                                          } else {
                                                                                              ""
                                                                                          },
                                                                                          if let Some(cmi) = current_memory_image.as_ref() {
                                                                                              &cmi
                                                                                          } else {
                                                                                              ""
                                                                                          },
                                                                                          framerate_samples.iter().fold(0f64, f64::add) /
                                                                                          framerate_samples.len() as f64))) {
                            eprintln!("warning: failed to set window title for framerate");
                        }
                        terminal::refresh();

                        frame_start = before;
                        refresh_ns = (precise_time_ns() - before).checked_sub(target_nanos).unwrap_or(0);

                        if vm.execution_finished || vm.active_breakpoint.is_some() {
                            break 'steppy;
                        }
                    }

                    if !terminal::set(terminal::config::Window::empty().title(format!("pir-8-emu{}{}",
                                                                                      if current_memory_image.is_some() {
                                                                                          " – "
                                                                                      } else {
                                                                                          ""
                                                                                      },
                                                                                      if let Some(cmi) = current_memory_image.as_ref() {
                                                                                          &cmi
                                                                                      } else {
                                                                                          ""
                                                                                      }))) {
                        eprintln!("warning: failed to set window title after end of auto step");
                    }
                    if !vm.active_breakpoint.is_some() {
                        pir_8_emu::binutils::pir_8_emu::display::status::line(0,
                                                                              0,
                                                                              "Stepping",
                                                                              if vm.execution_finished {
                                                                                  "finished"
                                                                              } else {
                                                                                  "cancelled"
                                                                              });
                    }
                    terminal::refresh();

                    continue;
                }
            }
            Event::KeyPressed { key: KeyCode::Space, .. } if !showing_help => {
                new_ops |= step(&mut vm, &config)?;
            }
            Event::KeyPressed { key: KeyCode::Space, .. } if showing_help => {
                let page_idx = help_page.take().unwrap();
                if page_idx < pir_8_emu::binutils::pir_8_emu::HELP_TEXT_PAGES.len() - 1 {
                    help_page = Some(page_idx + 1);
                } else {
                    terminal::clear(None);
                    write_main_screen(&mut vm);
                }
            }
            _ => {}
        }

        match help_page {
            Some(page_idx) => {
                terminal::clear(None);
                terminal::print_xy(0, 0, pir_8_emu::binutils::pir_8_emu::HELP_TEXT_PAGES[page_idx]);
            }
            None => update_main_screen(&mut vm, new_ops),
        }
        terminal::refresh();
    }


    Ok(())
}
