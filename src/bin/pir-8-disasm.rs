extern crate pir_8_emu;

use std::io::{self, Write, Read, stdout, stdin};
use std::process::exit;
use std::borrow::Cow;
use std::fs::File;


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let mut opts = pir_8_emu::options::DisassemblerOptions::parse();
    let registers = pir_8_emu::isa::default_general_purpose_registers();


    let mut output = stdout();

    let (mut input, input_name): (Box<Read>, Cow<'static, str>) = match opts.input.take() {
        Some((name, path)) => {
            (Box::new(File::open(path).map_err(|err| {
                     eprintln!("Couldn't open input file \"{}\": {}", name, err);
                     3
                 })?),
             name.into())
        }
        None => (Box::new(stdin()), "<stdin>".into()),
    };


    if opts.skip != 0 {
        io::copy(&mut (&mut input).take(opts.skip as u64), &mut io::sink()).map_err(|err| {
                eprintln!("Failed to skip {} header bytes of file {}: {}", opts.skip, input_name, err);
                5
            })?;
    }


    let mut data_buf = 0u16;
    let mut data_remaining = 0;
    let mut last_instruction = pir_8_emu::isa::instruction::Instruction::Reserved(0);

    let mut bytes_to_keep = 0;

    let mut bb = input.bytes().enumerate();
    while let Some((byte_idx, byte)) = bb.next() {
        let byte = byte.map_err(|err| {
                eprintln!("Failed to read byte {:#0w$X} of file {}: {}",
                          byte_idx,
                          input_name,
                          err,
                          w = 2 + pir_8_emu::util::min_byte_width(byte_idx) as usize * 2);
                5
            })?;


        if bytes_to_keep == 0 {
            if let Some(&(_, length)) = opts.keep.iter().find(|(s, l)| *l != 0 && *s == byte_idx) {
                bytes_to_keep = length;

                writeln!(output,
                         "{:08X}      S skipping {:#0w$X} bytes",
                         byte_idx,
                         length,
                         w = 2 + pir_8_emu::util::min_byte_width(length) as usize * 2).map_err(|err| {
                        eprintln!("Failed to write skip message ({:08X}, {:#04X}) from {}: {}", byte_idx, length, input_name, err);
                        4
                    })?;
            }
        }

        if bytes_to_keep != 0 {
            bytes_to_keep -= 1;
            continue;
        }


        if data_remaining != 0 {
            data_buf = (data_buf << 8) | (byte as u16);
            data_remaining -= 1;

            if data_remaining == 0 {
                let len = last_instruction.data_length();
                writeln!(output,
                         "{:08X} {}{d:0w$X} D {d:#0w_x$X}",
                         byte_idx,
                         if len == 1 { "  " } else { "" },
                         d = data_buf,
                         w = len * 2,
                         w_x = len * 2 + 2).map_err(|err| {
                        eprintln!("Failed to write instruction data {:#0w_x$X} for {} from {}:{:#10X}: {}",
                                  data_buf,
                                  last_instruction.display(&registers),
                                  input_name,
                                  byte_idx,
                                  err,
                                  w_x = len * 2 + 2);
                        4
                    })?;

                data_buf = 0;
            }
        } else {
            last_instruction = pir_8_emu::isa::instruction::Instruction::from(byte);
            data_remaining = last_instruction.data_length();

            writeln!(output,
                     "{:08X}   {:02X} {} {}",
                     byte_idx,
                     byte,
                     if last_instruction.is_valid() {
                         ' '
                     } else {
                         '!'
                     },
                     last_instruction.display(&registers)).map_err(|err| {
                    eprintln!("Failed to write instruction {} ({:#04X}) from {}:{:#10X}: {}",
                              last_instruction.display(&registers),
                              byte,
                              input_name,
                              byte_idx,
                              err);
                    4
                })?;
        }
    }


    if data_remaining != 0 {
        let len = last_instruction.data_length();
        eprintln!("Insufficient data for instruction {}: have {}/{} at {:#0w$X}",
                  last_instruction.display(&registers),
                  len - data_remaining,
                  len,
                  data_buf,
                  w = 2 + len * 2);

        return Err(7);
    }


    Ok(())
}
