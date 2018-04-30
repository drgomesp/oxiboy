pub trait InstructionCycle {
    fn fetch(&self, addr: u16) -> u8;
    fn decode(&self) -> Instruction;
    fn execute(&self) -> usize;
}

#[derive(Default)]
pub struct InstructionInfo {
    opcode: u8,
    string: &'static str,
    byte_length: usize,
    cycle_duration: usize,
}

pub enum Instruction {
    NOP { info: InstructionInfo },
}
