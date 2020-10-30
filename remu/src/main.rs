mod mmu;
mod riscv;

use std::path::PathBuf;
use clap::{Arg, App};
use mmu::{Mmu, VirtAddr};

use riscv::instruction::parse_instruction;
use riscv::register::Register;



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
        r.registers[Register::Pc as usize] = entry_point.0 as u64;
        r
    }

    pub fn get_register(&self, reg : Register) -> u64 {
        self.registers[reg as usize]
    }

    pub fn set_register(&mut self, reg : Register, value : u64) {
        assert!(reg != Register::Zero);
        self.registers[reg as usize] = value;
    }

    pub fn get_pc(&self) -> VirtAddr {
        VirtAddr(self.get_register(Register::Pc) as usize)
    }

    pub fn step(&mut self) {
        let pc = self.get_pc();
        let inst_u32 = self.mmu.read_u32(pc);

        let inst = parse_instruction(inst_u32);
        println!("\t{:x}: {:08x}\t\t{:}", pc.0, inst_u32, inst.disassemble());
        self.set_register(Register::Pc, (pc.0 + 4) as u64);
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
    loop {
        machine.step();
    }
}
