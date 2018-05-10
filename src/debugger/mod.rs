mod command;

use self::command::Command;
use super::gameboy::GameBoy;
use std::io::Write;
use std::io::{stdin, stdout};

pub struct Debugger {
    debug: bool,
    gb: GameBoy,
}

impl Debugger {
    pub fn new(gb: GameBoy) -> Self {
        Self { debug: true, gb }
    }

    pub fn run(&mut self) {
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
                    Ok(DumpReg) => println!("{:?}", self.gb.cpu.registers),
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
