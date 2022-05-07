use super::gameboy::GameBoy;

pub struct Emulator {
    gb: GameBoy,
}

impl Emulator {
    pub fn new(gb: GameBoy) -> Self {
        Self { gb: gb }
    }

    pub fn run(&mut self) {
        loop {
            self.gb.step();
        }
    }
}
