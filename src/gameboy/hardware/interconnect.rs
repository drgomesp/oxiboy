use super::bus::Bus;

pub struct Interconnect {
    bootrom: Box<[u8]>,
}

impl Interconnect {
    pub fn new(bootrom: Box<[u8]>) -> Self {
        Self { bootrom }
    }

    fn read_internal(&self, addr: u16) -> u8 {
        match addr {
            0x00...0xFF => self.bootrom[addr as usize],
            _ => panic!("Unrecognized read address ${:#x}", addr),
        }
    }
}

impl Bus for Interconnect {
    fn read(&self, addr: u16) -> u8 {
        self.read_internal(addr)
    }
}
