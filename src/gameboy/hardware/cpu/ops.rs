use super::instructions::Addr;
use super::registers::{Reg16, Reg8};

pub trait Ops {
    fn nop(self);
    fn load(self, Addr, Reg8);
    fn load16_imm(self, Reg16, u16);
    fn xor(self, Reg8);
}
