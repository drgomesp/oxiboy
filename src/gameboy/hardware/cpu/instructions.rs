use std::fmt;

use super::ops::Ops;
use super::registers::{Flags, Reg16, Reg8};
use super::super::bus::Bus;

pub trait InstructionDecoding {
    fn decode<B: Bus>(&mut self, opcode: u8, b: &mut B) -> Instruction;
    fn decode_cb<B: Bus>(&mut self, opcode: u8, b: &mut B) -> Instruction;
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

#[derive(Copy, Clone, Debug)]
pub enum JumpCondition {
    NZ,
}

impl JumpCondition {
    pub fn check(&self, flags: Flags) -> bool {
        use self::JumpCondition::*;

        match *self {
            NZ => !flags.contains(Flags::ZERO),
        }
    }
}

pub enum Instruction {
    Nop,
    Bit(InstructionInfo, usize, Reg8),
    Load(InstructionInfo, Addr, Reg8),
    Load16(InstructionInfo, Reg16, u16),
    Xor(InstructionInfo, Reg8),
    JumpOn(InstructionInfo, JumpCondition, i8),

    PrefixCB,
}

impl Instruction {
    pub fn execute<O: Ops>(self, ops: O) -> Instruction {
        use self::Instruction::*;

        match self {
            Bit(_, bit, reg) => ops.bit(bit, reg),
            Nop => ops.nop(),
            Load(_, addr, reg) => ops.load(addr, reg),
            Load16(_, reg, val) => ops.load16_imm(reg, val),
            Xor(_, reg) => ops.xor(reg),
            JumpOn(_, cond, offset) => ops.jr_c(cond, offset),

            PrefixCB => return ops.prefix_cb(),
        };

        self
    }
}

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Addr::*;

        match *self {
            HLD => write!(f, "(HL-)"),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;

        match *self {
            Nop => Ok(()),
            Bit(info, bit, reg) => write!(f, "{:#2x} BIT {:?},{:?}", info.opcode, bit, reg),
            Load(info, addr, reg) => match addr {
                Addr::HLD => write!(f, "{:#2x} LD {:},{:?}", info.opcode, addr, reg),
            },
            Load16(info, reg, val) => write!(f, "{:#2x} LD {:?},${:4x}", info.opcode, reg, val),
            Xor(info, reg) => write!(f, "{:#2x} XOR {:?}", info.opcode, reg),
            JumpOn(info, cond, addr) => write!(f, "{:#2x} JR {:?},{:#2x}", info.opcode, cond, addr),

            PrefixCB => Ok(()),
        }
    }
}
