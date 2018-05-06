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

#[derive(Copy, Clone, Debug)]
pub enum Src {
    D8(u8),
    D16(u16),
    Reg8(Reg8),
    Reg16(Reg16),
}

#[derive(Copy, Clone, Debug)]
pub enum Dst {
    A8(u8),
    Reg8(Reg8),
    Reg16(Reg16),
    Reg16Dec(Reg16),
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
    Load(InstructionInfo, Dst, Src),
    Xor(InstructionInfo, Reg8),
    Call(InstructionInfo, u16),
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
            Xor(_, reg) => ops.xor(reg),
            Call(_, addr) => ops.call(addr),
            JumpOn(_, cond, offset) => ops.jr_c(cond, offset),

            PrefixCB => return ops.prefix_cb(),

            Nop(_) => ops.nop(),
        };

        self
    }
}

impl fmt::Display for Src {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Src::*;

        match *self {
            D8(val) => write!(f, "${:#04X}", val),
            D16(val) => write!(f, "${:#04X}", val),
            Reg8(reg) => write!(f, "{:?}", reg),
            Reg16(reg) => write!(f, "({:?})", reg),
            _ => unimplemented!("display src:{:?}", *self),
        }
    }
}

impl fmt::Display for Dst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Dst::*;

        match *self {
            A8(val) => write!(f, "($FF00+${:#04X})", val),
            Reg8(reg) => write!(f, "{:?}", reg),
            Reg16(reg) => write!(f, "{:?}", reg),
            Reg16Dec(reg) => write!(f, "({:?}-)", reg),
            _ => unimplemented!("display dst:{:?}", *self),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;

        match *self {
            Nop(_) => Ok(()),
            Bit(info, bit, reg) => write!(f, "{:02X} BIT {:?},{:?}", info.opcode, bit, reg),
            Inc(info, reg) => write!(f, "{:02X} INC {:?}", info.opcode, reg),
            Load(info, dst, src) => write!(f, "{:02X} LD {:},{:}", info.opcode, dst, src),
            Xor(info, reg) => write!(f, "{:02X} XOR {:?}", info.opcode, reg),
            Call(info, addr) => write!(f, "{:02X} CALL ${:#06X}", info.opcode, addr),
            JumpOn(info, cond, addr) => {
                write!(f, "{:02X} JR {:?},${:#04X}", info.opcode, cond, addr)
            }

            PrefixCB => Ok(()),
        }
    }
}
