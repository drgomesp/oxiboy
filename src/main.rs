#[macro_use]
extern crate bitflags;

mod debugger;
mod gameboy;
mod emulation;

use gameboy::GameBoy;
// use emulation::Emulator;
use debugger::Debugger;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let bootrom_file_name = env::args().nth(1).unwrap();
    let bootrom = read_bin(bootrom_file_name);

    let gb = GameBoy::new(bootrom);
    // let mut emu = Emulator::new(gb);
    let mut dbg = Debugger::new(gb);
    dbg.run();
}

fn read_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    buffer.into_boxed_slice()
}
