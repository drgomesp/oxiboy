use std::str::FromStr;

pub enum Command {
    Breakpoint,
    Continue,
    Step,
    DumpReg,
    DumpMem,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Command::*;
        match s {
            "m" => Ok(DumpMem),
            "b" | "bp" | "break" | "breakpoint" => Ok(Breakpoint),
            "c" | "continue" => Ok(Continue),
            "s" | "step" => Ok(Step),
            "r" | "reg" | "registers" => Ok(DumpReg),
            _ => Err(()),
        }
    }
}
