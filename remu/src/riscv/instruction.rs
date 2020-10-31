use super::instruction_types::{*};
use crate::common::{Disassemble, Instruction};

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

ins!(LUI,    Utype, "lui");
ins!(AUIPC,  Utype, "auipc");
ins!(JAL,    Jtype, "jal");
ins!(JALR,   Itype, "jalr");
ins!(BEQ,    Btype, "beq");
ins!(BNE,    Btype, "bne");
ins!(BLT,    Btype, "blt");
ins!(BGE,    Btype, "bge");
ins!(BLTU,   Btype, "bltu");
ins!(BGEU,   Btype, "bgeu");
ins!(LB,     Itype, "lb");
ins!(LH,     Itype, "lh");
ins!(LW,     Itype, "lw");
ins!(LBU,    Itype, "lbu");
ins!(LHU,    Itype, "lhu");
ins!(SB,     Stype, "sb");
ins!(SH,     Stype, "sh");
ins!(SW,     Stype, "sw");
ins!(ADDI,   Itype, "addi");
ins!(SLTI,   Itype, "slti");
ins!(SLTIU,  Itype, "sltiu");
ins!(XORI,   Itype, "xori");
ins!(ORI,    Itype, "ori");
ins!(ANDI,   Itype, "andi");
ins!(SLLI,   ItypeShift, "slli");
ins!(SRLI,   ItypeShift, "srli");
ins!(SRAI,   ItypeShift, "srai");
ins!(ADD,    ItypeOp, "add");
ins!(SUB,    ItypeOp, "sub");
ins!(SLL,    ItypeOp, "sll");
ins!(SLT,    ItypeOp, "slt");
ins!(SLTU,   ItypeOp, "sltu");
ins!(XOR,    ItypeOp, "xor");
ins!(SRL,    ItypeOp, "srl");
ins!(SRA,    ItypeOp, "sra");
ins!(OR,     ItypeOp, "or");
ins!(AND,    ItypeOp, "and");
// TODO: FENCE
ins!(ECALL,  Ntype, "ecall");
ins!(EBREAK, Ntype, "ebreak");
// TODO: CSRR

// RV64I
ins!(LWU,   Itype, "lwu");
ins!(LD,    Itype, "ld");
ins!(SD,    Stype, "sd");
// TODO: SSLI
// TODO: SRLI
// TODO: SRAI
ins!(ADDIW, Itype, "addiw");
ins!(SLLIW, ItypeShift, "slliw");
ins!(SRLIW, ItypeShift, "srliw");
ins!(SRAIW, ItypeShift, "sraiw");
ins!(ADDW,  ItypeOp, "addw");
ins!(SUBW,  ItypeOp, "subw");
ins!(SLLW,  ItypeOp, "sllw");
ins!(SRLW,  ItypeOp, "srlw");
ins!(SRAW,  ItypeOp, "sraw");


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
