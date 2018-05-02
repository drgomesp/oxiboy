mod hardware;

use self::hardware::cpu::Cpu;
use self::hardware::interconnect::Interconnect;

pub struct GameBoy {
    cpu: Cpu,
    interconnect: Interconnect,
}

impl GameBoy {
    pub fn new(bootrom: Box<[u8]>) -> Self {
        Self {
            cpu: Cpu::new(),
            interconnect: Interconnect::new(bootrom),
        }
    }

    pub fn run(&mut self) {
        loop {
            let addr = self.cpu.registers.pc;
            println!("${:04x} {:}", addr, self.cpu.step(&mut self.interconnect));
        }
    }
}
