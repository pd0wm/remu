use super::instruction_types::{*};
use crate::riscv::register::Register::{*};
use crate::common::{Emulate, Disassemble, Instruction, Machine, VmExit};
use crate::instr;


instr!(LUI,    Utype,      _i, _m, Err(VmExit::NotImpl));
instr!(AUIPC,  Utype,      i, m, m.set_r(i.rd, (i.imm as i64 as u64).wrapping_add(m.get_r(Pc))));
instr!(JAL,    Jtype,      i, m, {
    let pc = m.get_r(Pc);
    m.set_r(i.rd, pc.wrapping_add(4))?;
    m.set_r(Pc, pc.wrapping_add(i.imm as i64 as u64).wrapping_sub(4))});
instr!(JALR,   Itype,      i, m, {
    let pc = m.get_r(Pc);
    let target = m.get_r(i.rs1).wrapping_add(i.imm as i64 as u64).wrapping_sub(4);
    m.set_r(i.rd, pc.wrapping_add(4))?;
    m.set_r(Pc, target)});
instr!(BEQ,    Btype,      i, m, {
    let t = m.get_r(Pc).wrapping_add(i.imm as i64 as u64).wrapping_sub(4);
    if m.get_r(i.rs1) == m.get_r(i.rs1) { m.set_r(Pc, t) } else { Ok(())}});
instr!(BNE,    Btype,      i, m, {
    let t = m.get_r(Pc).wrapping_add(i.imm as i64 as u64).wrapping_sub(4);
    if m.get_r(i.rs1) != m.get_r(i.rs1) { m.set_r(Pc, t) } else { Ok(())}});
instr!(BLT,    Btype,      i, m, {
    let t = m.get_r(Pc).wrapping_add(i.imm as i64 as u64).wrapping_sub(4);
    if (m.get_r(i.rs1) as i64) < (m.get_r(i.rs1) as i64) { m.set_r(Pc, t) } else { Ok(())}});
instr!(BGE,    Btype,      i, m, {
    let t = m.get_r(Pc).wrapping_add(i.imm as i64 as u64).wrapping_sub(4);
    if (m.get_r(i.rs1) as i64) >= (m.get_r(i.rs1) as i64) { m.set_r(Pc, t) } else { Ok(())}});
instr!(BLTU,   Btype,      i, m, {
    let t = m.get_r(Pc).wrapping_add(i.imm as i64 as u64).wrapping_sub(4);
    if m.get_r(i.rs1) < m.get_r(i.rs1) { m.set_r(Pc, t) } else { Ok(())}});
instr!(BGEU,   Btype,      i, m, {
    let t = m.get_r(Pc).wrapping_add(i.imm as i64 as u64).wrapping_sub(4);
    if m.get_r(i.rs1) >= m.get_r(i.rs1) { m.set_r(Pc, t) } else { Ok(())}});
instr!(LB,     Itype,      _i, _m, Err(VmExit::NotImpl));
instr!(LH,     Itype,      _i, _m, Err(VmExit::NotImpl));
instr!(LW,     Itype,      _i, _m, Err(VmExit::NotImpl));
instr!(LBU,    Itype,      _i, _m, Err(VmExit::NotImpl));
instr!(LHU,    Itype,      _i, _m, Err(VmExit::NotImpl));
instr!(SB,     Stype,      _i, _m, Err(VmExit::NotImpl));
instr!(SH,     Stype,      _i, _m, Err(VmExit::NotImpl));
instr!(SW,     Stype,      _i, _m, Err(VmExit::NotImpl));
instr!(ADDI,   Itype,      i, m, m.set_r(i.rd, m.get_r(i.rs1).wrapping_add(i.imm as i64 as u64)));
instr!(SLTI,   Itype,      _i, _m, Err(VmExit::NotImpl));
instr!(SLTIU,  Itype,      _i, _m, Err(VmExit::NotImpl));
instr!(XORI,   Itype,      _i, _m, Err(VmExit::NotImpl));
instr!(ORI,    Itype,      _i, _m, Err(VmExit::NotImpl));
instr!(ANDI,   Itype,      _i, _m, Err(VmExit::NotImpl));
instr!(SLLI,   ItypeShift, i, m, m.set_r(i.rd, m.get_r(i.rs1) << i.shamt));
instr!(SRLI,   ItypeShift, i, m, m.set_r(i.rd, m.get_r(i.rs1) >> i.shamt));
instr!(SRAI,   ItypeShift, i, m, m.set_r(i.rd, ((m.get_r(i.rs1) as i64) >> i.shamt) as u64));
instr!(ADD,    ItypeOp,    i, m, m.set_r(i.rd, m.get_r(i.rs1).wrapping_add(m.get_r(i.rs2))));
instr!(SUB,    ItypeOp,    i, m, m.set_r(i.rd, m.get_r(i.rs1).wrapping_sub(m.get_r(i.rs2))));
instr!(SLL,    ItypeOp,    _i, _m, Err(VmExit::NotImpl));
instr!(SLT,    ItypeOp,    _i, _m, Err(VmExit::NotImpl));
instr!(SLTU,   ItypeOp,    _i, _m, Err(VmExit::NotImpl));
instr!(XOR,    ItypeOp,    _i, _m, Err(VmExit::NotImpl));
instr!(SRL,    ItypeOp,    _i, _m, Err(VmExit::NotImpl));
instr!(SRA,    ItypeOp,    _i, _m, Err(VmExit::NotImpl));
instr!(OR,     ItypeOp,    _i, _m, Err(VmExit::NotImpl));
instr!(AND,    ItypeOp,    _i, _m, Err(VmExit::NotImpl));
// TODO: FENCE
instr!(ECALL,  Ntype,      _i, _m, Err(VmExit::Syscall));
instr!(EBREAK, Ntype,      _i, _m, Err(VmExit::NotImpl));
// TODO: CSRR

// RV64I
instr!(LWU,   Itype,       _i, _m, Err(VmExit::NotImpl));
instr!(LD,    Itype,       _i, _m, Err(VmExit::NotImpl));
instr!(SD,    Stype,       _i, _m, Err(VmExit::NotImpl));
// TODO: SSLI
// TODO: SRLI
// TODO: SRAI
instr!(ADDIW, Itype,       _i, _m, Err(VmExit::NotImpl));
instr!(SLLIW, ItypeShift,  _i, _m, Err(VmExit::NotImpl));
instr!(SRLIW, ItypeShift,  _i, _m, Err(VmExit::NotImpl));
instr!(SRAIW, ItypeShift,  _i, _m, Err(VmExit::NotImpl));
instr!(ADDW,  ItypeOp,     _i, _m, Err(VmExit::NotImpl));
instr!(SUBW,  ItypeOp,     _i, _m, Err(VmExit::NotImpl));
instr!(SLLW,  ItypeOp,     _i, _m, Err(VmExit::NotImpl));
instr!(SRLW,  ItypeOp,     _i, _m, Err(VmExit::NotImpl));
instr!(SRAW,  ItypeOp,     _i, _m, Err(VmExit::NotImpl));


pub fn parse_instruction(i: u32) -> Result<Box<dyn Instruction>, VmExit> {
    let opcode = i & 0b1111111;

    match opcode {
        0b0010111 => {Ok(Box::new(AUIPC::new(Utype::from(i))))},
        0b0110111 => {Ok(Box::new(LUI::new(Utype::from(i))))},
        0b1101111 => {Ok(Box::new(JAL::new(Jtype::from(i))))},
        0b1100111 => {Ok(Box::new(JALR::new(Itype::from(i))))},
        0b1100011 => {
            let inst = Btype::from(i);
            match inst.funct3 {
                0b000 => {Ok(Box::new(BEQ::new(inst)))},
                0b001 => {Ok(Box::new(BNE::new(inst)))},
                0b100 => {Ok(Box::new(BLT::new(inst)))},
                0b101 => {Ok(Box::new(BGE::new(inst)))},
                0b110 => {Ok(Box::new(BLTU::new(inst)))},
                0b111 => {Ok(Box::new(BGEU::new(inst)))},
                _ => Err(VmExit::InvalidOpcode(i)),
            }
        },
        0b0000011 => {
            let inst = Itype::from(i);
            match inst.funct3 {
                0b000 => {Ok(Box::new(LB::new(inst)))},
                0b001 => {Ok(Box::new(LH::new(inst)))},
                0b010 => {Ok(Box::new(LW::new(inst)))},
                0b100 => {Ok(Box::new(LBU::new(inst)))},
                0b101 => {Ok(Box::new(LHU::new(inst)))},
                // RV64I
                0b110 => {Ok(Box::new(LWU::new(inst)))},
                0b011 => {Ok(Box::new(LD::new(inst)))},
                _ => Err(VmExit::InvalidOpcode(i)),
            }
        },
        0b0100011 => {
            let inst = Stype::from(i);
            match inst.funct3 {
                0b000 => {Ok(Box::new(SB::new(inst)))},
                0b001 => {Ok(Box::new(SH::new(inst)))},
                0b010 => {Ok(Box::new(SW::new(inst)))},
                // RV64I
                0b011 => {Ok(Box::new(SD::new(inst)))},
                _ => Err(VmExit::InvalidOpcode(i)),
            }

        },
        0b0010011 => {
            let inst = Itype::from(i);
            match inst.funct3 {
                0b000 => {Ok(Box::new(ADDI::new(inst)))},
                0b010 => {Ok(Box::new(SLTI::new(inst)))},
                0b011 => {Ok(Box::new(SLTIU::new(inst)))},
                0b100 => {Ok(Box::new(XORI::new(inst)))},
                0b110 => {Ok(Box::new(ORI::new(inst)))},
                0b111 => {Ok(Box::new(ANDI::new(inst)))},
                // RV64I
                0b001 => {Ok(Box::new(SLLI::new(ItypeShift::from(i))))},
                0b101 => {
                    let mode = i >> 26;
                    match mode {
                        0b000000 => {Ok(Box::new(SRLI::new(ItypeShift::from(i))))}
                        0b010000 => {Ok(Box::new(SRAI::new(ItypeShift::from(i))))}
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
                (0b000,  0b0000000) => {Ok(Box::new(ADD::new(inst)))},
                (0b000,  0b0100000) => {Ok(Box::new(SUB::new(inst)))},
                (0b001,  0b0000000) => {Ok(Box::new(SLL::new(inst)))},
                (0b010,  0b0000000) => {Ok(Box::new(SLT::new(inst)))},
                (0b011,  0b0000000) => {Ok(Box::new(SLTU::new(inst)))},
                (0b100,  0b0000000) => {Ok(Box::new(XOR::new(inst)))},
                (0b101,  0b0000000) => {Ok(Box::new(SRL::new(inst)))},
                (0b101,  0b0100000) => {Ok(Box::new(SRA::new(inst)))},
                (0b110,  0b0000000) => {Ok(Box::new(OR::new(inst)))},
                (0b111,  0b0000000) => {Ok(Box::new(AND::new(inst)))},
                _ => Err(VmExit::InvalidOpcode(i)),
            }
        },
        0b1110011 => {
            let inst = ItypeOp::from(i);
            match inst.funct3 {
                0b000 => {
                    let mode = i >> 20;
                    match mode {
                        0b000000000000 => {Ok(Box::new(ECALL::new(Ntype::from(i))))},
                        0b000000000001 => {Ok(Box::new(EBREAK::new(Ntype::from(i))))},
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
                0b000 => {Ok(Box::new(ADDIW::new(inst)))},
                0b001 => {Ok(Box::new(SLLIW::new(ItypeShift::from(i))))},
                0b101 => {
                    match mode {
                        0b0000000 => {Ok(Box::new(SRLIW::new(ItypeShift::from(i))))},
                        0b0100000 => {Ok(Box::new(SRAIW::new(ItypeShift::from(i))))},
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
                (0b000,  0b0000000) => {Ok(Box::new(ADDW::new(inst)))},
                (0b000,  0b0100000) => {Ok(Box::new(SUBW::new(inst)))},
                (0b001,  0b0000000) => {Ok(Box::new(SLLW::new(inst)))},
                (0b101,  0b0000000) => {Ok(Box::new(SRLW::new(inst)))},
                (0b101,  0b0100000) => {Ok(Box::new(SRAW::new(inst)))},
                _ => Err(VmExit::InvalidOpcode(i)),
            }
        }
        _ => Err(VmExit::InvalidOpcode(i)),
    }
}
