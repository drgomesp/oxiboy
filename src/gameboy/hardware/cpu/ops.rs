use super::registers::{Reg16, Reg8};

pub trait Ops {
    fn nop(self);
    fn load16_imm(self, reg: Reg16, val: u16);
    fn xor(self, Reg8);
}
