#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate bitflags;
extern crate failure;
extern crate log;
extern crate simplelog;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use log::LevelFilter;
use simplelog::{CombinedLogger, Config, TermLogger};
use debugger::Debugger;

use emulation::Emulator;
use gameboy::GameBoy;

mod debugger;

mod gameboy;
mod emulation;

// use debugger::Debugger;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, Config::default()).unwrap()
    ])
        .unwrap();

    let bootrom_file_name = env::args().nth(1).unwrap();
    let bootrom = read_bin(bootrom_file_name);

    let rom_file_name = env::args().nth(2).unwrap();
    let rom = read_bin(rom_file_name);

    let gb = GameBoy::new(bootrom, rom);

    // let mut emu = Emulator::new(gb);
    // emu.run();

    let mut dbg = Debugger::new(gb);
    dbg.run();
}

fn read_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    buffer.into_boxed_slice()
}
