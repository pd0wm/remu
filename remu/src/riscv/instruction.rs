use super::instruction_types::{Utype, Itype};

macro_rules! ins {
    ($n:ident, $t:ident) => {
        struct $n {
            inst: $t,
        }

        impl $n {
            fn new(inst: $t) -> Self {
                Self {inst: inst}
            }
        }

    };
}

pub trait Disassemble {
    fn disassemble(&self) -> String;
}

pub trait Instruction : Disassemble {}
impl<T: Disassemble> Instruction for T {}


ins!(AUIPC, Utype);
impl Disassemble for AUIPC {
    fn disassemble(&self) -> String {
        format!("auipc {:}", self.inst.disassemble())
    }
}

ins!(LUI, Utype);
impl Disassemble for LUI {
    fn disassemble(&self) -> String {
        format!("lui {:}", self.inst.disassemble())
    }
}

ins!(ADDI, Itype);
impl Disassemble for ADDI {
    fn disassemble(&self) -> String {
        format!("addi {:}", self.inst.disassemble())
    }
}


pub fn parse_instruction(inst: u32) -> Box<dyn Instruction> {
    let opcode = inst & 0b1111111;
    match opcode {
        0b0010111 => {Box::new(AUIPC::new(Utype::from(inst)))},
        0b0110111 => {Box::new(LUI::new(Utype::from(inst)))},
        0b0010011 => {
            let inst = Itype::from(inst);
            match inst.funct3 {
                0b00 => {Box::new(ADDI::new(inst))}
                _ => unimplemented!("Unhandled Itype {:#03b}\n", inst.funct3),
            }
        },
        _ => unimplemented!("Unhandled opcode {:#09b}\n", opcode),
    }
}
