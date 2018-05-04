mod command;

use self::command::Command;
use std::io::{stdin, stdout};
use std::io::Write;
use super::gameboy::GameBoy;

pub struct Debugger {
    debug: bool,
    gb: GameBoy,
}

impl Debugger {
    pub fn new(gb: GameBoy) -> Self {
        Self {
            debug: true,
            gb: gb,
        }
    }

    pub fn run(&mut self) {
        loop {
            if !self.debug {
                self.gb.step()
            } else {
                print!("oxiboy> ");
                stdout().flush().unwrap();

                match read_stdin().parse() {
                    Ok(Command::Continue) => {
                        self.debug = false;
                        self.gb.step()
                    }
                    Ok(Command::Step) => self.gb.step(),
                    Ok(Command::DumpRegisters) => println!("{}", self.gb.cpu.registers),
                    _ => println!("invalid input"),
                };
            }
        }
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}
