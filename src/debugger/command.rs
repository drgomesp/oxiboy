use std::str::FromStr;

pub enum Command {
    Continue,
    Step,
    DumpRegisters,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" | "continue" => Ok(Command::Continue),
            "s" | "step" => Ok(Command::Step),
            "r" | "reg" | "registers" => Ok(Command::DumpRegisters),
            _ => Err(()),
        }
    }
}
