use super::instructions::{Dst, Instruction, JumpCondition, Src};
use super::registers::{Reg16, Reg8};

pub trait Ops {
    fn nop(self);
    fn bit(self, usize, Reg8);
    fn dec(self, Reg8);
    fn inc(self, Reg8);
    fn load(self, Dst, Src);
    fn xor(self, Reg8);
    fn call(self, u16);
    fn jr_c(self, JumpCondition, i8);
    fn jr(self, i8);
    fn ret(self, u16);
    fn push16(self, Reg16);
    fn pop16(self, Reg16);
    fn rl(self, Reg8, bool);
    fn inc16(self, Reg16);
    fn sub(self, u8);

    fn prefix_cb(self) -> Instruction;
}
