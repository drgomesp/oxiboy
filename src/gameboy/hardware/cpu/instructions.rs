use std::fmt;

use super::ops::Ops;
use super::registers::{Reg16, Reg8};
use super::super::bus::Bus;

pub trait InstructionDecoding {
    fn decode<B: Bus>(&mut self, opcode: u8, b: &mut B) -> Instruction;
}

#[derive(Copy, Clone, Default)]
pub struct InstructionInfo {
    pub opcode: u8,
    pub byte_length: usize,
    pub cycle_duration: usize,
}

#[derive(Copy, Clone)]
pub enum Addr {
    HLD,
}

pub enum Instruction {
    Nop,
    Load(InstructionInfo, Addr, Reg8),
    Load16(InstructionInfo, Reg16, u16),
    Xor(InstructionInfo, Reg8),
}

impl Instruction {
    pub fn execute<O: Ops>(self, ops: O) -> Instruction {
        use self::Instruction::*;

        match self {
            Nop => ops.nop(),
            Load(_, addr, reg) => ops.load(addr, reg),
            Load16(_, reg, val) => ops.load16_imm(reg, val),
            Xor(_, reg) => ops.xor(reg),
        };

        self
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;

        match *self {
            Nop => Ok(()),
            Load(_, addr, reg) => write!(f, ""),
            Load16(info, reg, val) => write!(f, "{:#2x} LD {:?},${:4x}", info.opcode, reg, val),
            Xor(info, reg) => write!(f, "{:#2x} XOR {:?}", info.opcode, reg),
        }
    }
}
