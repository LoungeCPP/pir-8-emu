extern crate pir_8_emu;

use pir_8_emu::isa::default_general_purpose_registers;
use pir_8_emu::isa::instruction::Instruction;
use std::fs::File;
use std::io::Read;
use std::env;


fn main() {
    let regs = default_general_purpose_registers();
    let f = File::open(env::args().skip(1).next().expect("File argument not passed")).expect("Passed file nonexistant");

    let mut bb = f.bytes().enumerate();
    while let Some((i, b)) = bb.next() {
        let b = b.unwrap();
        let ins = Instruction::from(b);

        println!("{:08X}   {:02X} {} {}", i, b, if ins.is_valid() { ' ' } else { '!' }, ins.display(&regs));
        match ins.data_length() {
            0 => {}
            len => {
                let mut data = 0u16;
                let mut i = i;
                for _ in 0..len {
                    let (new_i, b) = bb.next().expect("Missing bytes");

                    i = new_i;
                    data = (data << 8) | (b.unwrap() as u16);
                }

                println!("{:08X} {}{d:0w$X} D {d:#0w_x$X}",
                         i,
                         if len == 1 { "  " } else { "" },
                         d = data,
                         w = len * 2,
                         w_x = len * 2 + 2);
            }
        }
    }
}
