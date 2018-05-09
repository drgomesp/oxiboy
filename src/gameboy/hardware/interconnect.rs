use super::bus::Bus;

pub struct Interconnect {
    bootrom: Box<[u8]>,
    vram: Box<[u8]>,
    wram: Box<[u8]>,
}

impl Interconnect {
    pub fn new(bootrom: Box<[u8]>) -> Self {
        Self {
            bootrom: bootrom,
            vram: vec![0xFF; 8192].into_boxed_slice(),
            wram: vec![0xFF; 8192].into_boxed_slice(),
        }
    }

    fn read_internal(&self, addr: u16) -> u8 {
        match addr & 0xF000 {
            0x00...0xFF => {
                println!("- reading at addr: ${:#06X}", addr);
                self.bootrom[addr as usize]
            }
            0x8000...0x9FFF => self.vram[(addr & 0x1FFF) as usize],
            0xC000...0xDFFF => self.wram[(addr & 0x1FFF) as usize], // TODO handle bank switching?
            0x0100...0x7FFF => {
                unimplemented!("time to map the cartdrige data! o/")
            }
            _ => panic!("Unrecognized read address ${:#X}", addr),
        }
    }

    fn write_internal(&mut self, addr: u16, val: u8) {
        match addr & 0xF000 {
            0x8000...0x9FFF => {
                self.vram[(addr & 0x1FFF) as usize] = val;
            },
            0xC000...0xDFFF => {
                self.wram[(addr & 0x1FFF) as usize] = val;
            },
            0x0100 ... 0x7FFF => println!("-- writing cartdrige mem val:{:#04X} addr: ${:#06X}", val, addr & 0x1FFF),
            _ => {
                println!("!!TODO: write_internal(${:#04X}, {:#02X}): not implemented yet: missing relative adddr to physical addr mapping", addr >> 8, val)
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
