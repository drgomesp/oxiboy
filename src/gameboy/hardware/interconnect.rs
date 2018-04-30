pub struct Interconnect {
    bootrom: Box<[u8]>,
}

impl Interconnect {
    pub fn new(bootrom: Box<[u8]>) -> Self {
        Self { bootrom }
    }
}
