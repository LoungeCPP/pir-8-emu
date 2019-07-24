extern crate pir_8_emu;

use std::io::{BufReader, BufRead, stdout, stdin};
use std::collections::BTreeMap;
use std::process::exit;
use std::borrow::Cow;
use std::fs::File;


static DATA_REMAINING_EXPECTEDS: &[&[&str]] = &[&["1-byte number"], &["1- or 2-byte number"]];


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let mut opts = pir_8_emu::options::AssemblerOptions::parse();
    let registers = opts.register_lettters
        .take()
        .map(|ll| pir_8_emu::isa::GeneralPurposeRegister::from_letters(&ll).unwrap())
        .unwrap_or_else(pir_8_emu::isa::GeneralPurposeRegister::defaults);

    let mut output = match opts.output {
        Some((name, path)) => {
            pir_8_emu::binutils::pir_8_as::OutputWithQueue::new(File::create(path).map_err(|err| {
                    eprintln!("Error: couldn't create output file \"{}\": {}", name, err);
                    2
                })?)
        }
        None => pir_8_emu::binutils::pir_8_as::OutputWithQueue::new(stdout()),
    };


    let mut first_output = true;
    let mut next_output_address = None;
    let mut labels = BTreeMap::new();

    for input in opts.input {
        let (input, input_name): (Box<BufRead>, Cow<'static, str>) = match input {
            Some((name, path)) => {
                (Box::new(BufReader::new(File::open(path).map_err(|err| {
                         eprintln!("Error: couldn't open input file \"{}\": {}", name, err);
                         3
                     })?)),
                 name.into())
            }
            None => (Box::new(BufReader::new(stdin())), "<stdin>".into()),  // Using a wrapper struct around stdin().lock() would be ideal but
        };


        let mut data_remaining = 0;
        let mut last_instruction = pir_8_emu::isa::instruction::Instruction::Reserved(0);
        for (line_number, line) in input.lines().enumerate() {
            let line_number = line_number + 1;
            let line_orig = line.map_err(|err| {
                    eprintln!("Error: failed to read line {} of file {}: {}", line_number, input_name, err);
                    5
                })?;

            let line = pir_8_emu::util::remove_comment(';', &line_orig).trim_end();
            if line.is_empty() {
                continue;
            }

            let mut label_data = None;
            if let Some(directive) = pir_8_emu::binutils::pir_8_as::AssemblerDirective::from_str(line).map_err(|err| {
                    eprintln!("Error: failed to parse assembler directive at {}:{}:", input_name, line_number);
                    eprintln!("{}", line_orig);
                    eprintln!("{}", err);
                    8
                })? {

                if let Some(ll) = directive.obey(&mut next_output_address, &mut labels)
                    .map_err(|err| {
                        eprintln!("Error: failed to obey assembler directive at {}:{}:", input_name, line_number);
                        eprintln!("{}", line_orig);
                        eprintln!("{}", err);
                        8
                    })? {
                    if data_remaining != 2 {
                        eprintln!("Error: attempted to load label data when expecting {} bytes at {}:{}:",
                                  data_remaining,
                                  input_name,
                                  line_number);
                        eprintln!("{}", line_orig);
                        return Err(8);
                    }

                    match ll {
                        pir_8_emu::binutils::pir_8_as::LabelLoad::HaveImmediately(addr) => label_data = Some(addr),
                        pir_8_emu::binutils::pir_8_as::LabelLoad::WaitFor(lbl, offset) => {
                            output.wait_for_label(lbl, offset);
                            data_remaining = 0;
                            next_output_address = Some(next_output_address.unwrap_or(0) + 2);
                            continue;
                        }
                    }
                } else {
                    continue;
                }
            }

            if first_output && next_output_address.is_some() {
                for _ in 0..next_output_address.unwrap() {
                    output.write_all(&[0x00], &labels)
                        .map_err(|err| {
                            eprintln!("Error: failed to write origin padding: {}", err);
                            4
                        })?;
                }
            }
            first_output = false;

            if data_remaining != 0 {
                let line = line.trim_start();

                let data: u16 = if let Some(addr) = label_data {
                    addr
                } else {
                    pir_8_emu::util::parse_with_prefix(line).and_then(|data| pir_8_emu::util::limit_to_width(data, data_remaining * 8))
                        .ok_or_else(|| {
                            eprintln!("Error: failed to parse instruction data for {} ({} bytes remaining) at {}:{}:",
                                      last_instruction.display(&registers),
                                      data_remaining,
                                      input_name,
                                      line_number);
                            eprintln!("{}", line_orig);
                            eprintln!("{}",
                                      pir_8_emu::isa::instruction::ParseInstructionError::UnrecognisedToken((line.as_ptr() as usize) -
                                                                                                            (line_orig.as_ptr() as usize),
                                                                                                            DATA_REMAINING_EXPECTEDS[data_remaining as usize -
                                                                                                            1]));
                            7
                        })?
                };

                let data_length = data_remaining; // pir_8_emu::util::min_byte_width(data) doesn't handle, e.g. JUMP 0x0000
                next_output_address = Some(next_output_address.unwrap_or(0) + data_length as u16);

                data_remaining = 0;

                if data_length == 1 {
                        output.write_all(&[data as u8], &labels)
                    } else {
                        output.write_all(&[(data >> 8) as u8, (data & 0b1111_1111) as u8], &labels)
                    }.map_err(|err| {
                        eprintln!("Error: failed to write instruction data {:#w$x} for {} from {}:{}: {}",
                                  data,
                                  last_instruction.display(&registers),
                                  input_name,
                                  line_number,
                                  err,
                                  w = 2 + data_length as usize * 2);
                        4
                    })?;
            } else {
                last_instruction = pir_8_emu::isa::instruction::Instruction::from_str(line, &registers).map_err(|err| {
                        eprintln!("Error: failed to parse instruction at {}:{}:", input_name, line_number);
                        eprintln!("{}", line_orig);
                        eprintln!("{}", err);
                        6
                    })?;
                data_remaining = last_instruction.data_length() as u8;
                next_output_address = Some(next_output_address.unwrap_or(0) + 1);

                output.write_all(&[last_instruction.into()], &labels)
                    .map_err(|err| {
                        eprintln!("Error: failed to write instruction {} from {}:{}: {}",
                                  last_instruction.display(&registers),
                                  input_name,
                                  line_number,
                                  err);
                        4
                    })?;
            }
        }
    }


    output.flush(&labels)
        .map_err(|err| {
            eprintln!("Failed to flush output buffer: {}", err);
            4
        })?;

    if let Some(labels) = output.unfound_labels(&labels) {
        eprintln!("Error: the following {} labels were not found:", labels.len());
        for label in labels {
            eprintln!("  {}", label);
        }

        return Err(9);
    }


    Ok(())
}
