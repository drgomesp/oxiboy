use std::fmt;

use super::ops::Ops;
use super::registers::Reg16;
use super::super::bus::Bus;

pub trait InstructionDecoding {
    fn decode<B: Bus>(&mut self, opcode: u8, b: &mut B) -> Instruction;
}

#[derive(Copy, Clone, Default)]
pub struct InstructionInfo {
    pub opcode: u8,
    pub string: &'static str,
    pub byte_length: usize,
    pub cycle_duration: usize,
}

pub enum Instruction {
    Nop,
    Load16 {
        info: InstructionInfo,
        reg: Reg16,
        val: u16,
    },
}

impl Instruction {
    pub fn execute<O: Ops>(self, ops: O) -> Instruction {
        use self::Instruction::*;

        match self {
            Nop => ops.nop(),
            Load16 { info: _, reg, val } => ops.load16_imm(reg, val),
        };

        self
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;

        match *self {
            Nop => Ok(()),
            Load16 { info, reg, val } => write!(f, "{:02x} LD {:?},${:4x}", info.opcode, reg, val),
        }
    }
}
