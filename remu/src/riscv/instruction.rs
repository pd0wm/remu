use super::instruction_types::{Utype, Itype};

macro_rules! ins {
    ($n:ident, $t:ident, $mn:expr) => {
        struct $n {
            inst: $t,
        }

        impl $n {
            fn new(inst: $t) -> Self {
                Self {inst: inst}
            }
        }

        impl Disassemble for $n {
            fn disassemble(&self) -> String {
                format!("{:} {:}", $mn, self.inst.disassemble())
            }
        }

    };
}

pub trait Disassemble {
    fn disassemble(&self) -> String;
}

pub trait Instruction : Disassemble {}
impl<T: Disassemble> Instruction for T {}


ins!(AUIPC, Utype, "auipc");
ins!(LUI, Utype, "lui");
ins!(ADDI, Itype, "addi");


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
