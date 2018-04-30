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
            println!("{:}", self.cpu.registers);
            let _instr = self.cpu.step(&mut self.interconnect);
        }
    }
}
