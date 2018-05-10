#![deny(
//    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate bitflags;
extern crate failure;

mod debugger;
mod emulation;
mod gameboy;

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
