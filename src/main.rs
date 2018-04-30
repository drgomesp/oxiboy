mod gameboy;

use gameboy::GameBoy;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let bootrom_file_name = env::args().nth(1).unwrap();
    let bootrom = read_bin(bootrom_file_name);

    let mut gb = GameBoy::new(bootrom);
    gb.run();
}

fn read_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    buffer.into_boxed_slice()
}
