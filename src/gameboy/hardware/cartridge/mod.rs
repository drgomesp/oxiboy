pub struct Cartridge {
    rom: Box<[u8]>,
}

impl Cartridge {
    pub fn new(rom: Box<[u8]>) -> Self {
        Self {
            rom: rom,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let actual_addr = addr - 0x0100;
        return self.rom[actual_addr as usize];
    }
}
