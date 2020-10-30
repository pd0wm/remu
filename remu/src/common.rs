pub trait Disassemble {
    fn disassemble(&self) -> String;
}

pub trait Instruction : Disassemble {}
impl<T: Disassemble> Instruction for T {}
