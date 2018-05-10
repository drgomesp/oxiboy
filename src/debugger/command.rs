use std::str::FromStr;

pub enum Command {
    Breakpoint,
    Continue,
    Step,
    DumpRegisters,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Command::*;
        match s {
            "b" | "bp" | "break" | "breakpoint" => Ok(Breakpoint),
            "c" | "continue" => Ok(Continue),
            "s" | "step" => Ok(Step),
            "r" | "reg" | "registers" => Ok(DumpRegisters),
            _ => Err(()),
        }
    }
}
