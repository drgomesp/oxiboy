mod instructions;
mod registers;

use self::instructions::*;
use self::instructions::Instruction::*;
use self::registers::Registers;

pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    pub fn step(&self) -> usize {
        // TODO
        0
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
