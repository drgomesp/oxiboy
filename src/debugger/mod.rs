mod command;

use self::command::Command;
use std::io::{stdin, stdout};
use std::io::Write;
use super::gameboy::GameBoy;

pub struct Debugger {
    gb: GameBoy,
}

impl Debugger {
    pub fn new(gb: GameBoy) -> Self {
        Self { gb: gb }
    }

    pub fn run(&mut self) {
        loop {
            print!("oxiboy> ");
            stdout().flush().unwrap();

            match read_stdin().parse() {
                Ok(Command::Step) => self.gb.step(),
                _ => println!("invalid input"),
            };
        }
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}
