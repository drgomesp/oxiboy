use std::fmt;

use super::super::bus::Bus;
use super::ops::Ops;
use super::registers::{Flags, Reg16, Reg8};

pub trait InstructionDecoding {
    fn decode<B: Bus>(&mut self, opcode: u8, b: &mut B) -> Instruction;
    fn decode_cb<B: Bus>(&mut self, opcode: u8, b: &mut B) -> Instruction;
}

#[derive(Copy, Clone, Default)]
pub struct Info {
    pub opcode: u8,
    pub byte_length: usize,
    pub cycle_duration: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum Src {
    PagedA8(u8),
    D8(u8),
    D16(u16),
    Reg8(Reg8),
    Reg16(Reg16),
}

#[derive(Copy, Clone, Debug)]
pub enum Dst {
    A8(u8),
    Reg8(Reg8),
    PagedReg8(Reg8),
    Reg16(Reg16),
    Reg16Inc(Reg16),
    Reg16Dec(Reg16),
    Addr(u16),
}

#[derive(Copy, Clone, Debug)]
pub enum JumpCondition {
    NZ,
    Z,
}

impl JumpCondition {
    pub fn check(&self, flags: Flags) -> bool {
        use self::JumpCondition::*;

        match *self {
            NZ => !flags.contains(Flags::ZERO),
            Z => flags.contains(Flags::ZERO),
        }
    }
}

pub enum Instruction {
    Nop(Info),
    Bit(Info, usize, Reg8),
    Dec(Info, Reg8),
    Inc(Info, Reg8),
    Load(Info, Dst, Src),
    Xor(Info, Reg8),
    Call(Info, u16),
    JumpOn(Info, JumpCondition, i8),
    Jump(Info, i8),
    Ret(Info, u16),
    Push16(Info, Reg16),
    Pop16(Info, Reg16),
    RotateLeft(Info, Reg8, bool),
    RotateLeftAkku(Info, bool),
    Inc16(Info, Reg16),
    Compare(Info, u8),

    PrefixCB,
}

impl Instruction {
    pub fn execute<O: Ops>(self, ops: O) -> Self {
        use self::Instruction::*;

        match self {
            Bit(_, bit, reg) => ops.bit(bit, reg),
            Dec(_, reg) => ops.dec(reg),
            Inc(_, reg) => ops.inc(reg),
            Load(_, addr, reg) => ops.load(addr, reg),
            Xor(_, reg) => ops.xor(reg),
            Call(_, addr) => ops.call(addr),
            JumpOn(_, cond, offset) => ops.jr_c(cond, offset),
            Jump(_, offset) => ops.jr(offset),
            Ret(_, addr) => ops.ret(addr),
            Push16(_, reg) => ops.push16(reg),
            Pop16(_, reg) => ops.pop16(reg),
            RotateLeft(_, reg, set_zero) => ops.rl(reg, set_zero),
            RotateLeftAkku(_, set_zero) => ops.rl(Reg8::A, set_zero),
            Inc16(_, reg) => ops.inc16(reg),
            Compare(_, val) => ops.sub(val),

            PrefixCB => return ops.prefix_cb(),

            Nop(_) => ops.nop(),
        }

        self
    }
}

impl fmt::Display for Src {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Src::*;

        match *self {
            PagedA8(val) => write!(f, "($FF00+${:#04X})", val),
            D8(val) => write!(f, "${:#04X}", val),
            D16(val) => write!(f, "${:#04X}", val),
            Reg8(reg) => write!(f, "{:?}", reg),
            Reg16(reg) => write!(f, "({:?})", reg),
        }
    }
}

impl fmt::Display for Dst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Dst::*;

        match *self {
            A8(val) => write!(f, "($FF00+${:#04X})", val),
            Reg8(reg) => write!(f, "{:?}", reg),
            PagedReg8(reg) => write!(f, "($FF00+{:?})", reg),
            Reg16(reg) => write!(f, "{:?}", reg),
            Reg16Inc(reg) => write!(f, "({:?}+)", reg),
            Reg16Dec(reg) => write!(f, "({:?}-)", reg),
            Addr(addr) => write!(f, "(${:#04X})", addr),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;

        match *self {
            Nop(info) => write!(f, "[{:02X}] -> NOP", info.opcode),
            Bit(info, bit, reg) => write!(f, "[{:02X}] -> BIT {:?},{:?}", info.opcode, bit, reg),
            Dec(info, reg) => write!(f, "[{:02X}] -> DEC {:?}", info.opcode, reg),
            Inc(info, reg) => write!(f, "[{:02X}] -> INC {:?}", info.opcode, reg),
            Inc16(info, reg) => write!(f, "[{:02X}] -> INC {:?}", info.opcode, reg),
            Compare(info, val) => write!(f, "[{:02X}] -> CP ${:#04X}", info.opcode, val),
            Load(info, dst, src) => write!(f, "[{:02X}] -> LD {:},{:}", info.opcode, dst, src),
            Xor(info, reg) => write!(f, "[{:02X}] -> XOR {:?}", info.opcode, reg),
            Call(info, addr) => write!(f, "[{:02X}] -> CALL ${:#06X}", info.opcode, addr),
            JumpOn(info, cd, addr) => {
                write!(f, "[{:02X}] -> JR {:?},${:#04X}", info.opcode, cd, addr)
            }
            Jump(info, addr) => write!(f, "[{:02X}] -> JR ${:#04X}", info.opcode, addr),
            Ret(info, addr) => write!(f, "[{:02X}] -> RET ${:#06X}", info.opcode, addr),
            Push16(info, reg) => write!(f, "[{:02X}] -> PUSH {:?}", info.opcode, reg),
            Pop16(info, reg) => write!(f, "[{:02X}] -> POP {:?}", info.opcode, reg),
            RotateLeft(info, reg, _) => write!(f, "[{:02X}] -> RL {:?}", info.opcode, reg),
            RotateLeftAkku(info, _) => write!(f, "[{:02X}] -> RLA", info.opcode),

            PrefixCB => Ok(()),
        }
    }
}
