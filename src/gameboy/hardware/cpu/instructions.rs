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
    C,
    HL,
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
    Nop(InstructionInfo),
    Bit(InstructionInfo, usize, Reg8),
    Inc(InstructionInfo, Reg8),
    Load(InstructionInfo, Addr, Reg8),
    Load8(InstructionInfo, Reg8, u8),
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
            Inc(_, reg) => ops.inc(reg),
            Load(_, addr, reg) => ops.load(addr, reg),
            Load8(_, reg, val) => ops.load8_imm(reg, val),
            Load16(_, reg, val) => ops.load16_imm(reg, val),
            Xor(_, reg) => ops.xor(reg),
            JumpOn(_, cond, offset) => ops.jr_c(cond, offset),

            PrefixCB => return ops.prefix_cb(),

            Nop(_) => ops.nop(),
        };

        self
    }
}

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Addr::*;

        match *self {
            C => write!(f, "($FF00+C)"),
            HL => write!(f, "(HL)"),
            HLD => write!(f, "(HL-)"),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;

        match *self {
            Nop(_) => Ok(()),
            Bit(info, bit, reg) => write!(f, "{:#04X} BIT {:?},{:?}", info.opcode, bit, reg),
            Inc(info, reg) => write!(f, "{:#04X} INC {:?}", info.opcode, reg),
            Load(info, addr, reg) => match addr {
                _ => write!(f, "{:#04X} LD {:},{:?}", info.opcode, addr, reg),
            },
            Load8(info, reg, val) => write!(f, "{:#04X} LD {:?},${:#04x}", info.opcode, reg, val),
            Load16(info, reg, val) => write!(f, "{:#04X} LD {:?},${:#04x}", info.opcode, reg, val),
            Xor(info, reg) => write!(f, "{:#04X} XOR {:?}", info.opcode, reg),
            JumpOn(info, cond, addr) => {
                write!(f, "{:#04X} JR {:?},${:#04x}", info.opcode, cond, addr)
            }

            PrefixCB => Ok(()),
        }
    }
}
