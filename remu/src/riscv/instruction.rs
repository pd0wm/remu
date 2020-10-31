use super::instruction_types::{*};
use crate::common::{Emulate, Disassemble, Instruction, Machine, VmExit};
use crate::instr;


instr!(LUI,    Utype,      "lui",    |_i: &Utype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(AUIPC,  Utype,      "auipc",  |_i: &Utype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(JAL,    Jtype,      "jal",    |_i: &Jtype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(JALR,   Itype,      "jalr",   |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(BEQ,    Btype,      "beq",    |_i: &Btype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(BNE,    Btype,      "bne",    |_i: &Btype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(BLT,    Btype,      "blt",    |_i: &Btype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(BGE,    Btype,      "bge",    |_i: &Btype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(BLTU,   Btype,      "bltu",   |_i: &Btype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(BGEU,   Btype,      "bgeu",   |_i: &Btype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(LB,     Itype,      "lb",     |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(LH,     Itype,      "lh",     |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(LW,     Itype,      "lw",     |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(LBU,    Itype,      "lbu",    |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(LHU,    Itype,      "lhu",    |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SB,     Stype,      "sb",     |_i: &Stype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SH,     Stype,      "sh",     |_i: &Stype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SW,     Stype,      "sw",     |_i: &Stype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(ADDI,   Itype,      "addi",   |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SLTI,   Itype,      "slti",   |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SLTIU,  Itype,      "sltiu",  |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(XORI,   Itype,      "xori",   |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(ORI,    Itype,      "ori",    |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(ANDI,   Itype,      "andi",   |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SLLI,   ItypeShift, "slli",   |_i: &ItypeShift, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SRLI,   ItypeShift, "srli",   |_i: &ItypeShift, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SRAI,   ItypeShift, "srai",   |_i: &ItypeShift, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(ADD,    ItypeOp,    "add",    |i: &ItypeOp, m: &mut Machine| -> Result<(), VmExit> {m.set_r(i.rd, m.get_r(i.rs1) + m.get_r(i.rs2)); Ok(())});
instr!(SUB,    ItypeOp,    "sub",    |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SLL,    ItypeOp,    "sll",    |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SLT,    ItypeOp,    "slt",    |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SLTU,   ItypeOp,    "sltu",   |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(XOR,    ItypeOp,    "xor",    |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SRL,    ItypeOp,    "srl",    |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SRA,    ItypeOp,    "sra",    |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(OR,     ItypeOp,    "or",     |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(AND,    ItypeOp,    "and",    |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
// TODO: FENCE
instr!(ECALL,  Ntype,      "ecall",  |_i: &Ntype, _m: &mut Machine| -> Result<(), VmExit> {Err(VmExit::Syscall)});
instr!(EBREAK, Ntype,      "ebreak", |_i: &Ntype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
// TODO: CSRR

// RV64I
instr!(LWU,   Itype,       "lwu",    |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(LD,    Itype,       "ld",     |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SD,    Stype,       "sd",     |_i: &Stype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
// TODO: SSLI
// TODO: SRLI
// TODO: SRAI
instr!(ADDIW, Itype,       "addiw", |_i: &Itype, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SLLIW, ItypeShift,  "slliw", |_i: &ItypeShift, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SRLIW, ItypeShift,  "srliw", |_i: &ItypeShift, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SRAIW, ItypeShift,  "sraiw", |_i: &ItypeShift, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(ADDW,  ItypeOp,     "addw",  |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SUBW,  ItypeOp,     "subw",  |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SLLW,  ItypeOp,     "sllw",  |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SRLW,  ItypeOp,     "srlw",  |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});
instr!(SRAW,  ItypeOp,     "sraw",  |_i: &ItypeOp, _m: &mut Machine| -> Result<(), VmExit> {Ok(())});

fn pack_ok(i : impl Instruction + 'static) -> Result<Box<dyn Instruction>, VmExit> {
    Ok(Box::new(i))
}


pub fn parse_instruction(i: u32) -> Result<Box<dyn Instruction>, VmExit> {
    let opcode = i & 0b1111111;

    match opcode {
        0b0010111 => {pack_ok(AUIPC::new(Utype::from(i)))},
        0b0110111 => {pack_ok(LUI::new(Utype::from(i)))},
        0b1101111 => {pack_ok(JAL::new(Jtype::from(i)))},
        0b1100111 => {pack_ok(JALR::new(Itype::from(i)))},
        0b1100011 => {
            let inst = Btype::from(i);
            match inst.funct3 {
                0b000 => {pack_ok(BEQ::new(inst))},
                0b001 => {pack_ok(BNE::new(inst))},
                0b100 => {pack_ok(BLT::new(inst))},
                0b101 => {pack_ok(BGE::new(inst))},
                0b110 => {pack_ok(BLTU::new(inst))},
                0b111 => {pack_ok(BGEU::new(inst))},
                _ => Err(VmExit::InvalidOpcode(i)),
            }
        },
        0b0000011 => {
            let inst = Itype::from(i);
            match inst.funct3 {
                0b000 => {pack_ok(LB::new(inst))},
                0b001 => {pack_ok(LH::new(inst))},
                0b010 => {pack_ok(LW::new(inst))},
                0b100 => {pack_ok(LBU::new(inst))},
                0b101 => {pack_ok(LHU::new(inst))},
                // RV64I
                0b110 => {pack_ok(LWU::new(inst))},
                0b011 => {pack_ok(LD::new(inst))},
                _ => Err(VmExit::InvalidOpcode(i)),
            }
        },
        0b0100011 => {
            let inst = Stype::from(i);
            match inst.funct3 {
                0b000 => {pack_ok(SB::new(inst))},
                0b001 => {pack_ok(SH::new(inst))},
                0b010 => {pack_ok(SW::new(inst))},
                // RV64I
                0b011 => {pack_ok(SD::new(inst))},
                _ => Err(VmExit::InvalidOpcode(i)),
            }

        },
        0b0010011 => {
            let inst = Itype::from(i);
            match inst.funct3 {
                0b000 => {pack_ok(ADDI::new(inst))},
                0b010 => {pack_ok(SLTI::new(inst))},
                0b011 => {pack_ok(SLTIU::new(inst))},
                0b100 => {pack_ok(XORI::new(inst))},
                0b110 => {pack_ok(ORI::new(inst))},
                0b111 => {pack_ok(ANDI::new(inst))},
                // RV64I
                0b001 => {pack_ok(SLLI::new(ItypeShift::from(i)))},
                0b101 => {
                    let mode = i >> 26;
                    match mode {
                        0b000000 => {pack_ok(SRLI::new(ItypeShift::from(i)))}
                        0b010000 => {pack_ok(SRAI::new(ItypeShift::from(i)))}
                        _ => Err(VmExit::InvalidOpcode(i)),
                    }
                },
                _ => Err(VmExit::InvalidOpcode(i)),

            }
        },
        0b0110011 => {
            let inst = ItypeOp::from(i);
            let mode = i >> 25;
            match (inst.funct3, mode) {
                (0b000,  0b0000000) => {pack_ok(ADD::new(inst))},
                (0b000,  0b0100000) => {pack_ok(SUB::new(inst))},
                (0b001,  0b0000000) => {pack_ok(SLL::new(inst))},
                (0b010,  0b0000000) => {pack_ok(SLT::new(inst))},
                (0b011,  0b0000000) => {pack_ok(SLTU::new(inst))},
                (0b100,  0b0000000) => {pack_ok(XOR::new(inst))},
                (0b101,  0b0000000) => {pack_ok(SRL::new(inst))},
                (0b101,  0b0100000) => {pack_ok(SRA::new(inst))},
                (0b110,  0b0000000) => {pack_ok(OR::new(inst))},
                (0b111,  0b0000000) => {pack_ok(AND::new(inst))},
                _ => Err(VmExit::InvalidOpcode(i)),
            }
        },
        0b1110011 => {
            let inst = ItypeOp::from(i);
            match inst.funct3 {
                0b000 => {
                    let mode = i >> 20;
                    match mode {
                        0b000000000000 => {pack_ok(ECALL::new(Ntype::from(i)))},
                        0b000000000001 => {pack_ok(EBREAK::new(Ntype::from(i)))},
                        _ => Err(VmExit::InvalidOpcode(i)),
                    }
                },
                _ => Err(VmExit::InvalidOpcode(i)),
            }
        },
        // RV64I
        0b0011011 => {
            let inst = Itype::from(i);
            let mode = i >> 25;
            match inst.funct3 {
                0b000 => {pack_ok(ADDIW::new(inst))},
                0b001 => {pack_ok(SLLIW::new(ItypeShift::from(i)))},
                0b101 => {
                    match mode {
                        0b0000000 => {pack_ok(SRLIW::new(ItypeShift::from(i)))},
                        0b0100000 => {pack_ok(SRAIW::new(ItypeShift::from(i)))},
                        _ => Err(VmExit::InvalidOpcode(i)),
                    }
                },
                _ => Err(VmExit::InvalidOpcode(i)),
            }
        },
        0b0111011 => {
            let inst = ItypeOp::from(i);
            let mode = i >> 25;
            match (inst.funct3, mode) {
                (0b000,  0b0000000) => {pack_ok(ADDW::new(inst))},
                (0b000,  0b0100000) => {pack_ok(SUBW::new(inst))},
                (0b001,  0b0000000) => {pack_ok(SLLW::new(inst))},
                (0b101,  0b0000000) => {pack_ok(SRLW::new(inst))},
                (0b101,  0b0100000) => {pack_ok(SRAW::new(inst))},
                _ => Err(VmExit::InvalidOpcode(i)),
            }
        }
        _ => Err(VmExit::InvalidOpcode(i)),
    }
}
