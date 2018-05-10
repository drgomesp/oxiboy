use super::instructions::{Dst, Instruction, JumpCondition, Src};
use super::registers::{Reg16, Reg8};

pub trait Ops {
    fn nop(self);
    fn bit(self, bit: usize, reg: Reg8);
    fn dec(self, reg: Reg8);
    fn inc(self, reg: Reg8);
    fn load(self, dst: Dst, src: Src);
    fn xor(self, reg: Reg8);
    fn call(self, addr: u16);
    fn jr_c(self, cond: JumpCondition, offset: i8);
    fn jr(self, offset: i8);
    fn ret(self, addr: u16);
    fn push16(self, reg: Reg16);
    fn pop16(self, reg: Reg16);
    fn rl(self, reg: Reg8, set_zero: bool);
    fn inc16(self, reg: Reg16);
    fn sub(self, val: u8);

    fn prefix_cb(self) -> Instruction;
}
