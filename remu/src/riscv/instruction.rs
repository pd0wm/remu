use super::instruction_types::{*};
use crate::common::{Emulate, Disassemble, Instruction, Machine};

macro_rules! ins {
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
            fn emulate(&self, m : &mut Machine) {
                $ev(&self.i, m)
            }
        }

    };
}

ins!(LUI,    Utype,      "lui",    |_i: &Utype, _m: &mut Machine| {});
ins!(AUIPC,  Utype,      "auipc",  |_i: &Utype, _m: &mut Machine| {});
ins!(JAL,    Jtype,      "jal",    |_i: &Jtype, _m: &mut Machine| {});
ins!(JALR,   Itype,      "jalr",   |_i: &Itype, _m: &mut Machine| {});
ins!(BEQ,    Btype,      "beq",    |_i: &Btype, _m: &mut Machine| {});
ins!(BNE,    Btype,      "bne",    |_i: &Btype, _m: &mut Machine| {});
ins!(BLT,    Btype,      "blt",    |_i: &Btype, _m: &mut Machine| {});
ins!(BGE,    Btype,      "bge",    |_i: &Btype, _m: &mut Machine| {});
ins!(BLTU,   Btype,      "bltu",   |_i: &Btype, _m: &mut Machine| {});
ins!(BGEU,   Btype,      "bgeu",   |_i: &Btype, _m: &mut Machine| {});
ins!(LB,     Itype,      "lb",     |_i: &Itype, _m: &mut Machine| {});
ins!(LH,     Itype,      "lh",     |_i: &Itype, _m: &mut Machine| {});
ins!(LW,     Itype,      "lw",     |_i: &Itype, _m: &mut Machine| {});
ins!(LBU,    Itype,      "lbu",    |_i: &Itype, _m: &mut Machine| {});
ins!(LHU,    Itype,      "lhu",    |_i: &Itype, _m: &mut Machine| {});
ins!(SB,     Stype,      "sb",     |_i: &Stype, _m: &mut Machine| {});
ins!(SH,     Stype,      "sh",     |_i: &Stype, _m: &mut Machine| {});
ins!(SW,     Stype,      "sw",     |_i: &Stype, _m: &mut Machine| {});
ins!(ADDI,   Itype,      "addi",   |_i: &Itype, _m: &mut Machine| {});
ins!(SLTI,   Itype,      "slti",   |_i: &Itype, _m: &mut Machine| {});
ins!(SLTIU,  Itype,      "sltiu",  |_i: &Itype, _m: &mut Machine| {});
ins!(XORI,   Itype,      "xori",   |_i: &Itype, _m: &mut Machine| {});
ins!(ORI,    Itype,      "ori",    |_i: &Itype, _m: &mut Machine| {});
ins!(ANDI,   Itype,      "andi",   |_i: &Itype, _m: &mut Machine| {});
ins!(SLLI,   ItypeShift, "slli",   |_i: &ItypeShift, _m: &mut Machine| {});
ins!(SRLI,   ItypeShift, "srli",   |_i: &ItypeShift, _m: &mut Machine| {});
ins!(SRAI,   ItypeShift, "srai",   |_i: &ItypeShift, _m: &mut Machine| {});
ins!(ADD,    ItypeOp,    "add",    |i: &ItypeOp, m: &mut Machine| {m.set_r(i.rd, 0);});
ins!(SUB,    ItypeOp,    "sub",    |_i: &ItypeOp, _m: &mut Machine| {});
ins!(SLL,    ItypeOp,    "sll",    |_i: &ItypeOp, _m: &mut Machine| {});
ins!(SLT,    ItypeOp,    "slt",    |_i: &ItypeOp, _m: &mut Machine| {});
ins!(SLTU,   ItypeOp,    "sltu",   |_i: &ItypeOp, _m: &mut Machine| {});
ins!(XOR,    ItypeOp,    "xor",    |_i: &ItypeOp, _m: &mut Machine| {});
ins!(SRL,    ItypeOp,    "srl",    |_i: &ItypeOp, _m: &mut Machine| {});
ins!(SRA,    ItypeOp,    "sra",    |_i: &ItypeOp, _m: &mut Machine| {});
ins!(OR,     ItypeOp,    "or",     |_i: &ItypeOp, _m: &mut Machine| {});
ins!(AND,    ItypeOp,    "and",    |_i: &ItypeOp, _m: &mut Machine| {});
// TODO: FENCE
ins!(ECALL,  Ntype,      "ecall",  |_i: &Ntype, _m: &mut Machine| {});
ins!(EBREAK, Ntype,      "ebreak", |_i: &Ntype, _m: &mut Machine| {});
// TODO: CSRR

// RV64I
ins!(LWU,   Itype,       "lwu",    |_i: &Itype, _m: &mut Machine| {});
ins!(LD,    Itype,       "ld",     |_i: &Itype, _m: &mut Machine| {});
ins!(SD,    Stype,       "sd",     |_i: &Stype, _m: &mut Machine| {});
// TODO: SSLI
// TODO: SRLI
// TODO: SRAI
ins!(ADDIW, Itype,       "addiw", |_i: &Itype, _m: &mut Machine| {});
ins!(SLLIW, ItypeShift,  "slliw", |_i: &ItypeShift, _m: &mut Machine| {});
ins!(SRLIW, ItypeShift,  "srliw", |_i: &ItypeShift, _m: &mut Machine| {});
ins!(SRAIW, ItypeShift,  "sraiw", |_i: &ItypeShift, _m: &mut Machine| {});
ins!(ADDW,  ItypeOp,     "addw",  |_i: &ItypeOp, _m: &mut Machine| {});
ins!(SUBW,  ItypeOp,     "subw",  |_i: &ItypeOp, _m: &mut Machine| {});
ins!(SLLW,  ItypeOp,     "sllw",  |_i: &ItypeOp, _m: &mut Machine| {});
ins!(SRLW,  ItypeOp,     "srlw",  |_i: &ItypeOp, _m: &mut Machine| {});
ins!(SRAW,  ItypeOp,     "sraw",  |_i: &ItypeOp, _m: &mut Machine| {});


pub fn parse_instruction(i: u32) -> Box<dyn Instruction> {
    let opcode = i & 0b1111111;

    match opcode {
        0b0010111 => {Box::new(AUIPC::new(Utype::from(i)))},
        0b0110111 => {Box::new(LUI::new(Utype::from(i)))},
        0b1101111 => {Box::new(JAL::new(Jtype::from(i)))},
        0b1100111 => {Box::new(JALR::new(Itype::from(i)))},
        0b1100011 => {
            let inst = Btype::from(i);
            match inst.funct3 {
                0b000 => {Box::new(BEQ::new(inst))},
                0b001 => {Box::new(BNE::new(inst))},
                0b100 => {Box::new(BLT::new(inst))},
                0b101 => {Box::new(BGE::new(inst))},
                0b110 => {Box::new(BLTU::new(inst))},
                0b111 => {Box::new(BGEU::new(inst))},
                _ => unreachable!(),
            }
        },
        0b0000011 => {
            let inst = Itype::from(i);
            match inst.funct3 {
                0b000 => {Box::new(LB::new(inst))},
                0b001 => {Box::new(LH::new(inst))},
                0b010 => {Box::new(LW::new(inst))},
                0b100 => {Box::new(LBU::new(inst))},
                0b101 => {Box::new(LHU::new(inst))},
                // RV64I
                0b110 => {Box::new(LWU::new(inst))},
                0b011 => {Box::new(LD::new(inst))},
                _ => unreachable!(),
            }
        },
        0b0100011 => {
            let inst = Stype::from(i);
            match inst.funct3 {
                0b000 => {Box::new(SB::new(inst))},
                0b001 => {Box::new(SH::new(inst))},
                0b010 => {Box::new(SW::new(inst))},
                // RV64I
                0b011 => {Box::new(SD::new(inst))},
                _ => unreachable!(),
            }

        },
        0b0010011 => {
            let inst = Itype::from(i);
            match inst.funct3 {
                0b000 => {Box::new(ADDI::new(inst))},
                0b010 => {Box::new(SLTI::new(inst))},
                0b011 => {Box::new(SLTIU::new(inst))},
                0b100 => {Box::new(XORI::new(inst))},
                0b110 => {Box::new(ORI::new(inst))},
                0b111 => {Box::new(ANDI::new(inst))},
                // RV64I
                0b001 => {Box::new(SLLI::new(ItypeShift::from(i)))},
                0b101 => {
                    let mode = i >> 26;
                    match mode {
                        0b000000 => {Box::new(SRLI::new(ItypeShift::from(i)))}
                        0b010000 => {Box::new(SRAI::new(ItypeShift::from(i)))}
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!(),

            }
        },
        0b0110011 => {
            let inst = ItypeOp::from(i);
            let mode = i >> 25;
            match (inst.funct3, mode) {
                (0b000,  0b0000000) => {Box::new(ADD::new(inst))},
                (0b000,  0b0100000) => {Box::new(SUB::new(inst))},
                (0b001,  0b0000000) => {Box::new(SLL::new(inst))},
                (0b010,  0b0000000) => {Box::new(SLT::new(inst))},
                (0b011,  0b0000000) => {Box::new(SLTU::new(inst))},
                (0b100,  0b0000000) => {Box::new(XOR::new(inst))},
                (0b101,  0b0000000) => {Box::new(SRL::new(inst))},
                (0b101,  0b0100000) => {Box::new(SRA::new(inst))},
                (0b110,  0b0000000) => {Box::new(OR::new(inst))},
                (0b111,  0b0000000) => {Box::new(AND::new(inst))},
                _ => unreachable!(),
            }
        },
        0b1110011 => {
            let inst = ItypeOp::from(i);
            match inst.funct3 {
                0b000 => {
                    let mode = i >> 20;
                    match mode {
                        0b000000000000 => {Box::new(ECALL::new(Ntype::from(i)))},
                        0b000000000001 => {Box::new(EBREAK::new(Ntype::from(i)))},
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!(),
            }
        },
        // RV64I
        0b0011011 => {
            let inst = Itype::from(i);
            let mode = i >> 25;
            match inst.funct3 {
                0b000 => {Box::new(ADDIW::new(inst))},
                0b001 => {Box::new(SLLIW::new(ItypeShift::from(i)))},
                0b101 => {
                    match mode {
                        0b0000000 => {Box::new(SRLIW::new(ItypeShift::from(i)))},
                        0b0100000 => {Box::new(SRAIW::new(ItypeShift::from(i)))},
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!(),
            }
        },
        0b0111011 => {
            let inst = ItypeOp::from(i);
            let mode = i >> 25;
            match (inst.funct3, mode) {
                (0b000,  0b0000000) => {Box::new(ADDW::new(inst))},
                (0b000,  0b0100000) => {Box::new(SUBW::new(inst))},
                (0b001,  0b0000000) => {Box::new(SLLW::new(inst))},
                (0b101,  0b0000000) => {Box::new(SRLW::new(inst))},
                (0b101,  0b0100000) => {Box::new(SRAW::new(inst))},
                _ => unreachable!(),
            }
        }
        _ => unimplemented!("Unhandled opcode {:#09b}\n", opcode),
    }
}
