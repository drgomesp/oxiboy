use super::registers::Reg16;

pub trait Ops {
    fn nop(self);
    fn load16_imm(self, reg: Reg16, val: u16);
}
