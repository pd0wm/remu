use super::instruction_types::{Utype, Itype};

pub trait Instruction {
    fn disassemble(&self) -> String;
}

struct AUIPC {
    inst: Utype
}


impl AUIPC {
    fn new(inst: Utype) -> Self {
        Self {inst: inst}
    }
}


impl Instruction for AUIPC {
    fn disassemble(&self) -> String {
        format!("auipc {:?},{:#02x?}", self.inst.rd, self.inst.imm)
    }
}

struct LUI {
    inst: Utype
}

impl LUI {
    fn new(inst: Utype) -> Self {
        Self {inst: inst}
    }
}


impl Instruction for LUI {
    fn disassemble(&self) -> String {
        format!("auipc {:?},{:#02x?}", self.inst.rd, self.inst.imm)
    }
}

struct ADDI {
    inst: Itype
}

impl ADDI {
    fn new(inst: Itype) -> Self {
        Self {inst: inst}
    }
}


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
