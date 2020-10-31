mod mmu;

mod common;
mod riscv;

use std::path::PathBuf;
use clap::{Arg, App};
use crate::mmu::Mmu;
use crate::common::Machine;



fn main() {
    let matches = App::new("Emulator")
        .arg(Arg::with_name("input")
             .long("input").
             takes_value(true))
        .get_matches();

    let myfile = matches.value_of("input").unwrap();
    let path = PathBuf::from(myfile);

    let mut mmu = Mmu::new(1024 * 1024);
    mmu.load_elf(&path);

    let mut machine = Machine::new(mmu);
    // loop {

    for _ in 0..10 {
        machine.step().unwrap();
    }
}
