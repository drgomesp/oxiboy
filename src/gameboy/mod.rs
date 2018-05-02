mod hardware;

use self::hardware::cpu::LR35902;
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

    pub fn step(&mut self) {
        let addr = self.cpu.registers.pc;
        println!("${:04x} {:}", addr, self.cpu.step(&mut self.interconnect));
    }
}
