use super::instructions::{Addr, Instruction};
use super::registers::{Reg16, Reg8};

pub trait Ops {
    fn nop(self);
    fn bit(self, usize, Reg8);
    fn load(self, Addr, Reg8);
    fn load16_imm(self, Reg16, u16);
    fn xor(self, Reg8);

    fn prefix_cb(self) -> Instruction;
}
