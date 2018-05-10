use std::fmt;

bitflags!(
  pub struct Flags: u8 {
    const ZERO       = 0b_1000_0000; 
    const ADD_SUB    = 0b_0100_0000;
    const HALF_CARRY = 0b_0010_0000;
    const CARRY      = 0b_0001_0000;
  }
);

impl Flags {
    pub fn self_or_empty(&self, test: bool) -> Flags {
        if test {
            *self
        } else {
            Flags::empty()
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Copy, Clone, Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

pub struct Registers {
    pc: u16,
    sp: u16,

    a: u8,
    pub f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            pc: 0,
            sp: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: Flags::empty(),
            h: 0,
            l: 0,
        }
    }
}

impl Registers {
    pub fn read8(&self, reg: Reg8) -> u8 {
        use self::Reg8::*;

        match reg {
            A => self.a,
            B => self.b,
            C => self.c,
            D => self.d,
            E => self.e,
            H => self.h,
            L => self.l,
        }
    }

    pub fn write8(&mut self, reg: Reg8, val: u8) {
        use self::Reg8::*;

        match reg {
            A => self.a = val,
            B => self.b = val,
            C => self.c = val,
            D => self.d = val,
            E => self.e = val,
            H => self.h = val,
            L => self.l = val,
        }
    }

    pub fn read16(&self, reg: Reg16) -> u16 {
        use self::Reg16::*;

        match reg {
            AF => ((self.a as u16) << 8) | (self.f.bits() as u16),
            BC => ((self.b as u16) << 8) | (self.c as u16),
            DE => ((self.d as u16) << 8) | (self.e as u16),
            HL => ((self.h as u16) << 8) | (self.l as u16),
            PC => self.pc,
            SP => self.sp,
        }
    }

    pub fn write16(&mut self, reg: Reg16, val: u16) {
        use self::Reg16::*;

        match reg {
            AF => unimplemented!(),
            BC => {
                (self.b = (val >> 8) as u8);
                self.c = val as u8;
            }
            DE => {
                (self.d = (val >> 8) as u8);
                self.e = val as u8;
            }
            HL => {
                (self.h = (val >> 8) as u8);
                self.l = val as u8;
            }
            SP => self.sp = val,
            PC => self.pc = val,
        }
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PC:{:04X} SP:{:04X} \
             A:{:02X} F:{:#08b} B:{:02X} C:{:02X} \
             D:{:02X} E:{:02X} H:{:02X} L:{:02X}",
            self.pc, self.sp, self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l
        )
    }
}
