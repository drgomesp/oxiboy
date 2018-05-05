use super::instructions::{Addr, Instruction, JumpCondition};
use super::registers::{Reg16, Reg8};

pub trait Ops {
    fn nop(self);
    fn bit(self, usize, Reg8);
    fn inc(self, Reg8);
    fn load(self, Addr, Reg8);
    fn load8_imm(self, Reg8, u8);
    fn load16_imm(self, Reg16, u16);

    fn xor(self, Reg8);
    fn jr_c(self, JumpCondition, i8);

    fn prefix_cb(self) -> Instruction;
}
