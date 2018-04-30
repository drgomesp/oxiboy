pub enum Reg16 {
    SP,
}

#[derive(Default)]
pub struct Registers {
    sp: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers::default()
    }
}
