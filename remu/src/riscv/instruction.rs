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

pub trait Instruction {
    fn disassemble(&self) -> String;
}

ins!(AUIPC, Utype);
impl Instruction for AUIPC {
    fn disassemble(&self) -> String {
        format!("auipc {:?},{:#02x?}", self.inst.rd, self.inst.imm)
    }
}

ins!(LUI, Utype);
impl Instruction for LUI {
    fn disassemble(&self) -> String {
        format!("auipc {:?},{:#02x?}", self.inst.rd, self.inst.imm)
    }
}

ins!(ADDI, Itype);
impl Instruction for ADDI {
    fn disassemble(&self) -> String {
        format!("addi {:?},{:?},{:}", self.inst.rd, self.inst.rs1, self.inst.imm)
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
