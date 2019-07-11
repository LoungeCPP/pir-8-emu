extern crate pir_8_emu;

use pir_8_emu::isa::instruction::Instruction;
use std::env;
use std::fs;


fn main() {
    for (i, (b, ins)) in fs::read(env::args().skip(1).next().expect("File argument not passed"))
        .expect("Passed file nonexistant")
        .into_iter()
        .map(|b| (b, Instruction::from(b)))
        .enumerate() {
        println!("{:08X} {:02X} {} {:?}", i, b, if ins.is_valid() { ' ' } else { '!' }, ins);
    }
}
