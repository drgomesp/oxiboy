use super::bus::Bus;

pub struct Interconnect {
    bootrom: Box<[u8]>,
    io: Box<[u8]>,
    vram: Box<[u8]>,
    wram: Box<[u8]>,
    hram: Box<[u8]>,
}

impl Interconnect {
    pub fn new(bootrom: Box<[u8]>) -> Self {
        Self {
            bootrom: bootrom,
            io: vec![0xFF; 127].into_boxed_slice(),
            vram: vec![0xFF; 8192].into_boxed_slice(),
            wram: vec![0xFF; 8192].into_boxed_slice(),
            hram: vec![0xFF; 127].into_boxed_slice(),
        }
    }

    fn read_internal(&self, addr: u16) -> u8 {
        match addr {
            0x0...0xFF => self.bootrom[addr as usize],
            0x0100...0x7FFF => panic!("time to map the cartdrige data! o/"),
            0x8000...0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xC000...0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xFF00...0xFF7F => self.io[(addr - 0xFF00) as usize],
            0xFF80...0xFFFE => self.hram[(addr - 0xFF80) as usize],

            _ => panic!("Unrecognized read address ${:#X}", addr),
        }
    }

    fn write_internal(&mut self, addr: u16, val: u8) {
        match addr {
            0x0100...0x7FFF => println!("-- writing cartdrige mem val:{:#04X} addr: ${:#06X}", val, addr - 0x0100),
            0x8000...0x9FFF => {
                self.vram[(addr - 0x8000) as usize] = val;
            },
            0xC000...0xDFFF => {
                self.wram[(addr - 0xC000) as usize] = val;
            },
            0xFF00...0xFF7F => self.io[(addr - 0xFF00) as usize] = val,
            0xFF80...0xFFFE => self.hram[(addr - 0xFF80) as usize] = val,
            _ => {
                panic!("!!TODO: write_internal(${:#04X}, {:#02X}): not implemented yet: missing relative adddr to physical addr mapping", addr, val)
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
