use super::bus::MemoryBus;
use super::ppu::PPU;
use failure::Error;

pub struct Interconnect {
    bootrom: Box<[u8]>,

    ppu: PPU,

    io: Box<[u8]>,
    wram: Box<[u8]>,
    hram: Box<[u8]>,
}

impl Interconnect {
    pub fn new(bootrom: Box<[u8]>) -> Self {
        Self {
            bootrom: bootrom,
            ppu: PPU::new(),

            io: vec![0xFF; 127].into_boxed_slice(),
            wram: vec![0xFF; 8192].into_boxed_slice(),
            hram: vec![0xFF; 127].into_boxed_slice(),
        }
    }

    pub fn read_internal(&self, addr: u16) -> u8 {
        match addr {
            0x0...0xFF => self.bootrom[addr as usize],
            0x0100...0x7FFF => panic!("time to map the cartdrige data! o/"),
            0x8000...0x9FFF => self.ppu.read(addr - 0x8000),
            0xC000...0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xFF00...0xFF7F => match addr {
                0xFF40 => self.ppu.get_control(),
                0xFF44 => self.ppu.get_current_line(),
                _ => self.io[(addr - 0xFF00) as usize],
            },
            0xFF80...0xFFFE => self.hram[(addr - 0xFF80) as usize],

            _ => panic!("Unrecognized read address ${:#X}", addr),
        }
    }

    fn write_internal(&mut self, addr: u16, val: u8) {
        match addr {
            0x0100...0x7FFF => panic!("-- writing cartdrige mem val:{:#04X} addr: ${:#06X}", val, addr - 0x0100),
            0x8000...0x9FFF => self.ppu.write(addr - 0x8000, val),
            0xC000...0xDFFF => {
                self.wram[(addr - 0xC000) as usize] = val;
            },
            0xFF00...0xFF7F => {
                match addr {
                    0xFF40 => self.ppu.set_control(val),
                    0xFF42 => self.ppu.set_scroll_y(val),
                    0xFF44 => panic!("kurwa!"),
                    _ => self.io[(addr - 0xFF00) as usize] = val
                }
            },
            0xFF80...0xFFFE => self.hram[(addr - 0xFF80) as usize] = val,
            _ => {
                panic!("!!TODO: write_internal(${:#04X}, {:#02X}): not implemented yet: missing relative adddr to physical addr mapping", addr, val)
            }
        }
    }
}

impl MemoryBus for Interconnect {
    fn read(&self, addr: u16) -> Result<u8, Error> {
        Ok(self.read_internal(addr))
    }

    fn write(&mut self, addr: u16, val: u8) -> Result<(), Error> {
        Ok(self.write_internal(addr, val))
    }
}
