use std::io::{stdin, stdout};
use std::io::Write;

use super::gameboy::GameBoy;

use self::command::Command;

mod command;

pub struct Debugger {
    debug: bool,
    gb: GameBoy,
}

impl Debugger {
    pub fn new(gb: GameBoy) -> Self {
        Self { debug: true, gb }
    }

    pub fn run(&mut self) {
        // clear terminal screen and position at top-left
        print!("\x1B[2J\x1B[1;1H");

        print!("oxiboy> \
            [s] step \
            [c] continue \
            [bp] add breakpoint \
            [r] dump registers \
            [m] dump memory\
        \n");

        loop {
            if self.debug {
                print!("oxiboy> ");
                stdout().flush().unwrap();

                use self::Command::*;
                match read_stdin().parse() {
                    Ok(Breakpoint) => {
                        let addr = read_stdin().parse::<u16>().unwrap();

                        while self.gb.pc() != addr {
                            self.gb.step()
                        }
                    }
                    Ok(Continue) => {
                        self.debug = false;
                        self.gb.step()
                    }
                    Ok(Step) => self.gb.step(),
                    Ok(DumpMem) => {
                        let addr = read_stdin().parse::<u16>().unwrap();
                        println!("${:#04X}: {:#02X}", addr, self.gb.mem(addr));
                    }
                    Ok(DumpReg) => println!("\n{:?}", self.gb.cpu.registers),
                    _ => println!("invalid input"),
                };
            } else {
                self.gb.step()
            }
        }
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}
