pub struct GameBoy {}

impl GameBoy {
    pub fn new(_bootrom: Box<[u8]>) -> Self {
        Self {}
    }

    pub fn run(&self) {
        loop {}
    }
}
