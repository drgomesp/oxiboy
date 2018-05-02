use std::str::FromStr;

pub enum Command {
    Step,
    DumpRegisters,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" | "step" => Ok(Command::Step),
            "r" | "reg" | "registers" => Ok(Command::DumpRegisters),
            _ => Err(()),
        }
    }
}
