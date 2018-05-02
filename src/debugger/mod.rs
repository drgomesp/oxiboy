use super::gameboy::GameBoy;

pub struct Debugger {
    gb: GameBoy,
}

impl Debugger {
    pub fn new(gb: GameBoy) -> Self {
        Self { gb: gb }
    }

    pub fn run(&mut self) {
        loop {
            self.gb.step();
        }
    }
}
