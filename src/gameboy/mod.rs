mod hardware;

use self::hardware::cpu::LR35902;
use self::hardware::cpu::registers::Reg16;
use self::hardware::interconnect::Interconnect;

pub struct GameBoy {
    pub cpu: LR35902,
    interconnect: Interconnect,
}

impl GameBoy {
    pub fn new(bootrom: Box<[u8]>) -> Self {
        Self {
            cpu: LR35902::new(),
            interconnect: Interconnect::new(bootrom),
        }
    }

    pub fn pc(&self) -> u16 {
        self.cpu.registers.read16(Reg16::PC)
    }

    pub fn step(&mut self) {
        let addr = self.cpu.registers.read16(Reg16::PC);
        println!("${:04X} {:}", addr, self.cpu.step(&mut self.interconnect));
    }
}
