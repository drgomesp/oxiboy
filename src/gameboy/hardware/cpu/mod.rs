mod instructions;
mod ops;
mod registers;

use self::instructions::*;
use self::instructions::Instruction::*;
use self::registers::Registers;

use self::ops::Ops;
use super::bus::Bus;

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

impl<'a, B> Ops for (&'a mut Cpu, &'a mut B)
where
    B: Bus,
{
    type T = ();

    fn nop(self) {}
}
