mod instructions;

use self::instructions::*;
use self::instructions::Instruction::*;

pub struct Cpu {}

impl Cpu {
    pub fn new() -> Self {
        Self {}
    }
}

impl InstructionCycle for Cpu {
    fn fetch(&self, _addr: u16) -> u8 {
        0
    }

    fn decode(&self) -> Instruction {
        NOP {
            info: InstructionInfo::default(),
        }
    }

    fn execute(&self) -> usize {
        0
    }
}
