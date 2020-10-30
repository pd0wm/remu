// https://github.com/gamozolabs/fuzz_with_emus/blob/master/src/emulator.rs
use super::register::Register;
use super::instruction::Disassemble;

#[derive(Debug, Copy, Clone)]
pub struct Rtype {
    pub funct7: u32,
    pub rs2:    Register,
    pub rs1:    Register,
    pub funct3: u32,
    pub rd:     Register,
}

impl From<u32> for Rtype {
    fn from(inst: u32) -> Self {
        Rtype {
            funct7: (inst >> 25) & 0b1111111,
            rs2:    Register::from((inst >> 20) & 0b11111),
            rs1:    Register::from((inst >> 15) & 0b11111),
            funct3: (inst >> 12) & 0b111,
            rd:     Register::from((inst >>  7) & 0b11111),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Stype {
    pub imm:    i32,
    pub rs2:    Register,
    pub rs1:    Register,
    pub funct3: u32,
}

impl From<u32> for Stype {
    fn from(inst: u32) -> Self {
        let imm115 = (inst >> 25) & 0b1111111;
        let imm40  = (inst >>  7) & 0b11111;

        let imm = (imm115 << 5) | imm40;
        let imm = ((imm as i32) << 20) >> 20;

        Stype {
            imm:    imm,
            rs2:    Register::from((inst >> 20) & 0b11111),
            rs1:    Register::from((inst >> 15) & 0b11111),
            funct3: (inst >> 12) & 0b111,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Jtype {
    pub imm: i32,
    pub rd:  Register,
}

impl From<u32> for Jtype {
    fn from(inst: u32) -> Self {
        let imm20   = (inst >> 31) & 1;
        let imm101  = (inst >> 21) & 0b1111111111;
        let imm11   = (inst >> 20) & 1;
        let imm1912 = (inst >> 12) & 0b11111111;

        let imm = (imm20 << 20) | (imm1912 << 12) | (imm11 << 11) |
            (imm101 << 1);
        let imm = ((imm as i32) << 11) >> 11;

        Jtype {
            imm: imm,
            rd:  Register::from((inst >> 7) & 0b11111),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Btype {
    pub imm:    i32,
    pub rs2:    Register,
    pub rs1:    Register,
    pub funct3: u32,
}

impl From<u32> for Btype {
    fn from(inst: u32) -> Self {
        let imm12  = (inst >> 31) & 1;
        let imm105 = (inst >> 25) & 0b111111;
        let imm41  = (inst >>  8) & 0b1111;
        let imm11  = (inst >>  7) & 1;

        let imm = (imm12 << 12) | (imm11 << 11) |(imm105 << 5) | (imm41 << 1);
        let imm = ((imm as i32) << 19) >> 19;

        Btype {
            imm:    imm,
            rs2:    Register::from((inst >> 20) & 0b11111),
            rs1:    Register::from((inst >> 15) & 0b11111),
            funct3: (inst >> 12) & 0b111,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Itype {
    pub imm:    i32,
    pub rs1:    Register,
    pub funct3: u32,
    pub rd:     Register,
}

impl From<u32> for Itype {
    fn from(inst: u32) -> Self {
        let imm = (inst as i32) >> 20;
        Itype {
            imm:    imm,
            rs1:    Register::from((inst >> 15) & 0b11111),
            funct3: (inst >> 12) & 0b111,
            rd:     Register::from((inst >>  7) & 0b11111),
        }
    }
}

impl Disassemble for Itype {
    fn disassemble(&self) -> String {
        format!("{:?},{:?},{:}", self.rd, self.rs1, self.imm)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Utype {
    pub imm: i32,
    pub rd:  Register,
}

impl From<u32> for Utype {
    fn from(inst: u32) -> Self {
        Utype {
            imm: ((inst & !0xfff) >> 12) as i32,
            rd:  Register::from((inst >> 7) & 0b11111),
        }
    }
}

impl Disassemble for Utype {
    fn disassemble(&self) -> String {
        format!("{:?},{:#02x?}", self.rd, self.imm)
    }
}
