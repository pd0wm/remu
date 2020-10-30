pub mod mmu;
pub mod riscv;

use std::path::PathBuf;
use clap::{Arg, App};
use mmu::{Mmu, VirtAddr};


struct Machine {
    mmu : Mmu,
    registers: [u64; 33],
}

impl Machine {
    pub fn new(mmu : Mmu) -> Self {
        let entry_point = mmu.entry_point.unwrap();
        let mut r = Machine {
            mmu : mmu,
            registers : [0; 33],
        };
        r.registers[riscv::Register::Pc as usize] = entry_point.0 as u64;
        r
    }

    pub fn get_pc(&self) -> VirtAddr {
        VirtAddr(self.registers[riscv::Register::Pc as usize] as usize)
    }

    pub fn step(&mut self) {
        let instruction = self.mmu.read_u32(self.get_pc());
        println!("{:08X?}", instruction);
    }
}


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
    machine.step();
}
