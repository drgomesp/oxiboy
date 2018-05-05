use super::bus::Bus;

pub struct Interconnect {
    bootrom: Box<[u8]>,
    vram: Box<[u8]>,
}

impl Interconnect {
    pub fn new(bootrom: Box<[u8]>) -> Self {
        Self {
            bootrom: bootrom,
            vram: vec![0; 8192].into_boxed_slice(),
        }
    }

    fn read_internal(&self, addr: u16) -> u8 {
        match addr {
            0x00...0xFF => self.bootrom[addr as usize],
            _ => panic!("Unrecognized read address ${:#x}", addr),
        }
    }

    fn write_internal(&mut self, addr: u16, val: u8) {
        match addr & 0xF000 {
            0x8000...0x9FFF => {
                self.vram[(addr & 0x1FFF) as usize] = val;
            },
            _ => {
                println!("write_internal(${:#4x}, {:#02X}): not implemented yet: missing relative adddr to physical addr mapping", addr, val)
            }
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
