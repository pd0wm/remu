use crate::mmu::{Mmu, VirtAddr};
use crate::riscv::instruction::parse_instruction;
use crate::riscv::register::Register;

pub struct Machine {
    pub mmu : Mmu,
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

    pub fn get_r(&self, reg : Register) -> u64 {
        self.registers[reg as usize]
    }

    pub fn set_r(&mut self, reg : Register, value : u64) {
        assert!(reg != Register::Zero);
        self.registers[reg as usize] = value;
    }

    pub fn get_pc(&self) -> VirtAddr {
        VirtAddr(self.get_r(Register::Pc) as usize)
    }

    pub fn step(&mut self) {
        let pc = self.get_pc();
        let inst_u32 = self.mmu.read_u32(pc);

        let inst = parse_instruction(inst_u32);
        println!("\t{:x}: {:08x}\t\t{:}", pc.0, inst_u32, inst.disassemble());
        self.set_r(Register::Pc, (pc.0 + 4) as u64);
    }
}

pub trait Disassemble {
    fn disassemble(&self) -> String;
}

pub trait Emulate {
    fn emulate(&self, m : &mut Machine);
}

pub trait Instruction : Disassemble + Emulate {}
impl<T: Disassemble + Emulate> Instruction for T {}
