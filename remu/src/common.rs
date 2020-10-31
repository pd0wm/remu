use crate::mmu::{Mmu, VirtAddr};
use crate::riscv::instruction::parse_instruction;
use crate::riscv::register::Register;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum VmExit {
    InvalidOpcode(u32),
    NotImpl,
    Syscall,
}

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

    pub fn print_state(&self){
        print!("|");
        for i in 1..33 {
            let r = Register::from(i);
            print!(" {:?}: {:} |", r, self.registers[i as usize] as i64);

            if i % 16 == 0 {
                println!("");
            }
        }
    }

    pub fn get_r(&self, reg : Register) -> u64 {
        self.registers[reg as usize]
    }

    pub fn set_r(&mut self, reg : Register, value : u64) -> Result<(), VmExit> {
        if reg != Register::Zero {
            self.registers[reg as usize] = value;
        }
        Ok(())
    }

    pub fn get_pc(&self) -> VirtAddr {
        VirtAddr(self.get_r(Register::Pc) as usize)
    }

    pub fn step(&mut self) -> Result<(), VmExit> {
        let pc = self.get_pc();
        let inst_u32 = self.mmu.read_u32(pc);

        let inst = parse_instruction(inst_u32)?;
        println!("\t{:x}: {:08x}\t\t{:}", pc.0, inst_u32, inst.disassemble());

        inst.emulate(self)?;

        self.set_r(Register::Pc, self.get_r(Register::Pc).wrapping_add(4))?;
        self.print_state();
        Ok(())
    }
}

pub trait Disassemble {
    fn disassemble(&self) -> String;
}

pub trait Emulate {
    fn emulate(&self, m : &mut Machine) -> Result<(), VmExit>;
}

pub trait Instruction : Disassemble + Emulate {}
impl<T: Disassemble + Emulate> Instruction for T {}


#[macro_export]
macro_rules! instr {
    ($n:ident, $t:ident, $mn:expr, $ev:expr) => {
        struct $n {
            i: $t,
        }

        impl $n {
            fn new(inst: $t) -> Self {
                Self {i: inst}
            }
        }

        impl Disassemble for $n {
            fn disassemble(&self) -> String {
                format!("{:} {:}", $mn, self.i.disassemble())
            }
        }

        impl Emulate for $n {
            fn emulate(&self, m : &mut Machine) -> Result<(), VmExit> {
                $ev(&self.i, m)
            }
        }

    };
}
