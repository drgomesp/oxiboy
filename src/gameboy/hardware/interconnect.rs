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

    fn write_internal(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000...0x9FFF => println!("Writing value {:#x} at GPU mem address ${:#x}", val, addr),
            _ => panic!(
                "Unrecognized write at address ${:#x} with value ${:#x}",
                addr, val
            ),
        }
    }
}

impl Bus for Interconnect {
    fn read(&self, addr: u16) -> u8 {
        self.read_internal(addr)
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.write_internal(addr, val)
    }
}
