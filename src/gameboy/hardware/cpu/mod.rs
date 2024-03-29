use super::bus::MemoryBus;

use self::instructions::*;
use self::ops::Ops;
use self::registers::{Flags, Reg16, Reg8, Registers};

mod instructions;
mod ops;
pub mod registers;

pub struct LR35902 {
    pub registers: Registers,
}

impl LR35902 {
    pub fn new() -> Self {
        Self {
            registers: Default::default(),
        }
    }

    pub fn step<B: MemoryBus>(&mut self, bus: &mut B) -> Instruction {
        let pc = self.registers.read16(Reg16::PC);
        let opcode = bus.read(pc).unwrap();

        self.registers.write16(Reg16::PC, pc.wrapping_add(1));
        let instr = self.decode(opcode, bus);

        instr.execute((self, bus))
    }

    fn next_u8<B: MemoryBus>(&mut self, bus: &mut B) -> u8 {
        let pc = self.registers.read16(Reg16::PC);
        self.registers.write16(Reg16::PC, pc.wrapping_add(1));
        bus.read(pc).unwrap()
    }

    fn next_u16<B: MemoryBus>(&mut self, bus: &mut B) -> u16 {
        let l = self.next_u8(bus);
        let h = self.next_u8(bus);
        ((h as u16) << 8) | (l as u16)
    }

    fn push_u8<B: MemoryBus>(&mut self, bus: &mut B, val: u8) {
        let sp = self.registers.read16(Reg16::SP);
        self.registers.write16(Reg16::SP, sp.wrapping_sub(1));
        bus.write(self.registers.read16(Reg16::SP), val).unwrap()
    }

    fn push_u16<B: MemoryBus>(&mut self, bus: &mut B, val: u16) {
        self.push_u8(bus, (val >> 8) as u8);
        self.push_u8(bus, val as u8);
    }

    fn pop_u8<B: MemoryBus>(&mut self, bus: &mut B) -> u8 {
        let sp = self.registers.read16(Reg16::SP);
        let val = bus.read(sp).unwrap();
        self.registers.write16(Reg16::SP, sp.wrapping_add(1));
        val
    }

    fn pop_u16<B: MemoryBus>(&mut self, bus: &mut B) -> u16 {
        let l = self.pop_u8(bus);
        let h = self.pop_u8(bus);
        ((h as u16) << 8 | (l as u16))
    }
}

impl InstructionDecoding for LR35902 {
    fn decode<B: MemoryBus>(&mut self, opcode: u8, bus: &mut B) -> Instruction {
        use self::Instruction::*;

        match opcode {
            0x0E => Load(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Dst::Reg8(Reg8::C),
                Src::D8(self.next_u8(bus)),
            ),
            0x05 => Dec(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Reg8::B,
            ),
            0x11 => Load(
                Info {
                    opcode: opcode,
                    byte_length: 3,
                    cycle_duration: 12,
                },
                Dst::Reg16(Reg16::DE),
                Src::D16(self.next_u16(bus)),
            ),
            0x13 => Inc16(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Reg16::DE,
            ),
            0x18 => Jump(
                Info {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 12,
                },
                self.next_u8(bus) as i8,
            ),
            0x1A => Load(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Dst::Reg8(Reg8::A),
                Src::Reg16(Reg16::DE),
            ),
            0x57 => Load(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Dst::Reg8(Reg8::D),
                Src::Reg8(Reg8::A),
            ),
            0x67 => Load(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Dst::Reg8(Reg8::H),
                Src::Reg8(Reg8::A),
            ),
            0x20 => JumpOn(
                Info {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 12,
                },
                JumpCondition::NZ,
                self.next_u8(bus) as i8,
            ),
            0x21 => Load(
                Info {
                    opcode: opcode,
                    byte_length: 3,
                    cycle_duration: 12,
                },
                Dst::Reg16(Reg16::HL),
                Src::D16(self.next_u16(bus)),
            ),
            0x23 => Inc16(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Reg16::HL,
            ),
            0x31 => Load(
                Info {
                    opcode: opcode,
                    byte_length: 3,
                    cycle_duration: 12,
                },
                Dst::Reg16(Reg16::SP),
                Src::D16(self.next_u16(bus)),
            ),
            0x22 => Load(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Dst::Reg16Inc(Reg16::HL),
                Src::Reg8(Reg8::A),
            ),
            0x32 => Load(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Dst::Reg16Dec(Reg16::HL),
                Src::Reg8(Reg8::A),
            ),
            0x3D => Dec(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Reg8::A,
            ),
            0x3E => Load(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Dst::Reg8(Reg8::A),
                Src::D8(self.next_u8(bus)),
            ),
            0x4F => Load(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Dst::Reg8(Reg8::C),
                Src::Reg8(Reg8::A),
            ),
            0x06 => Load(
                Info {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 8,
                },
                Dst::Reg8(Reg8::B),
                Src::D8(self.next_u8(bus)),
            ),
            0xAF => Xor(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Reg8::A,
            ),
            0x1E => Load(
                Info {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 8,
                },
                Dst::Reg8(Reg8::E),
                Src::D8(self.next_u8(bus)),
            ),
            0xE2 => Load(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Dst::PagedReg8(Reg8::C),
                Src::Reg8(Reg8::A),
            ),
            0x04 => Inc(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Reg8::B,
            ),
            0x0C => Inc(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Reg8::C,
            ),
            0x0D => Dec(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Reg8::C,
            ),
            0x28 => JumpOn(
                Info {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 12,
                },
                JumpCondition::Z,
                self.next_u8(bus) as i8,
            ),
            0x3C => Inc(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Reg8::A,
            ),
            0x7B => Load(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Dst::Reg8(Reg8::A),
                Src::Reg8(Reg8::E),
            ),
            0x77 => Load(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Dst::Reg16(Reg16::HL),
                Src::Reg8(Reg8::A),
            ),
            0xC1 => Pop16(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 12,
                },
                Reg16::BC,
            ),
            0xCD => Call(
                Info {
                    opcode: opcode,
                    byte_length: 3,
                    cycle_duration: 24,
                },
                self.next_u16(bus),
            ),
            0xC5 => Push16(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 16,
                },
                Reg16::BC,
            ),
            0xC9 => Ret(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 16,
                },
                self.pop_u16(bus),
            ),
            0xEA => Load(
                Info {
                    opcode: opcode,
                    byte_length: 3,
                    cycle_duration: 16,
                },
                Dst::Addr(self.next_u16(bus)),
                Src::Reg8(Reg8::A),
            ),
            0x2E => Load(
                Info {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 12,
                },
                Dst::Reg8(Reg8::L),
                Src::D8(self.next_u8(bus)),
            ),
            0xE0 => Load(
                Info {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 12,
                },
                Dst::A8(self.next_u8(bus)),
                Src::Reg8(Reg8::A),
            ),
            0xF0 => Load(
                Info {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 12,
                },
                Dst::Reg8(Reg8::A),
                Src::PagedA8(self.next_u8(bus)),
            ),
            0xFE => Compare(
                Info {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 8,
                },
                self.next_u8(bus),
            ),
            0x17 => RotateLeftAkku(
                Info {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                false,
            ),
            0xCB => PrefixCB,
            0x00 => Nop(Info {
                opcode: 0x00,
                byte_length: 1,
                cycle_duration: 4,
            }),
            _ => panic!("unrecognized opcode: {:02X}", opcode),
        }
    }

    fn decode_cb<B: MemoryBus>(&mut self, opcode: u8, _: &mut B) -> Instruction {
        use self::Instruction::*;

        match opcode {
            0x11 => RotateLeft(
                Info {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 8,
                },
                Reg8::C,
                true,
            ),
            0x7C => Bit(
                Info {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 8,
                },
                7,
                Reg8::H,
            ),
            _ => panic!("Unrecognized cb opcode: {:02X}", opcode),
        }
    }
}

impl<'a, B> Ops for (&'a mut LR35902, &'a mut B)
    where
        B: MemoryBus,
{
    fn nop(self) {}

    fn bit(self, bit: usize, reg: Reg8) {
        let (cpu, _) = self;
        let val = cpu.registers.read8(reg) & (1 << bit);

        cpu.registers.f = Flags::ZERO.self_or_empty(val == 0)
            | Flags::HALF_CARRY
            | (Flags::CARRY & cpu.registers.f);
    }

    fn dec(self, reg: Reg8) {
        let (cpu, _) = self;
        let val = cpu.registers.read8(reg);
        let new_val = val.wrapping_sub(1);

        cpu.registers.f = Flags::ZERO.self_or_empty(new_val == 0)
            | Flags::ADD_SUB
            | Flags::HALF_CARRY.self_or_empty(val & 0xf == 0)
            | (Flags::CARRY & cpu.registers.f);

        cpu.registers.write8(reg, new_val);
    }

    fn inc(self, reg: Reg8) {
        let (cpu, _) = self;
        let val = cpu.registers.read8(reg);
        let new_val = val.wrapping_add(1);

        cpu.registers.f = Flags::ZERO.self_or_empty(new_val == 0)
            | Flags::HALF_CARRY.self_or_empty(val & 0xf == 0xf)
            | (Flags::CARRY & cpu.registers.f);

        cpu.registers.write8(reg, new_val);
    }

    fn load(self, dst: Dst, src: Src) {
        let (cpu, bus) = self;

        let val: u16 = match src {
            Src::PagedA8(val) => bus.read(0xFF00u16 | (val as u16)).unwrap() as u16,
            Src::D8(val) => val as u16,
            Src::D16(val) => val,
            Src::Reg8(reg) => cpu.registers.read8(reg) as u16,
            Src::Reg16(reg) => cpu.registers.read16(reg),
        };

        match dst {
            Dst::A8(val) => {
                let addr = 0xFF00u16 | val as u16;
                bus.write(addr, cpu.registers.read8(Reg8::A))
            }
            Dst::Reg8(reg) => Ok(cpu.registers.write8(reg, val as u8)),
            Dst::PagedReg8(reg) => {
                let addr = 0xFF00u16 | cpu.registers.read8(reg) as u16;
                bus.write(addr, cpu.registers.read8(Reg8::A))
            }
            Dst::Reg16(reg) => Ok(cpu.registers.write16(reg, val)),
            Dst::Reg16Inc(reg) => {
                let addr = cpu.registers.read16(reg);
                cpu.registers.write16(reg, addr.wrapping_add(1));
                bus.write(addr, val as u8)
            }
            Dst::Reg16Dec(reg) => {
                let addr = cpu.registers.read16(reg);
                cpu.registers.write16(reg, addr.wrapping_sub(1));
                bus.write(addr, val as u8)
            }
            Dst::Addr(addr) => bus.write(addr, val as u8),
        };
    }

    fn xor(self, reg: Reg8) {
        let (cpu, _) = self;
        let v = cpu.registers.read8(reg);
        cpu.registers.write8(reg, v ^ v);
        cpu.registers.f = Flags::ZERO.self_or_empty(cpu.registers.read8(Reg8::A) == 0)
    }

    fn call(self, addr: u16) {
        let (cpu, bus) = self;
        let pc = cpu.registers.read16(Reg16::PC);
        cpu.push_u16(bus, pc);
        cpu.registers.write16(Reg16::PC, addr);
    }

    fn jr_c(self, cond: JumpCondition, offset: i8) {
        let (cpu, _) = self;

        if cond.check(cpu.registers.f) {
            let addr = cpu.registers.read16(Reg16::PC).wrapping_add(offset as u16);
            cpu.registers.write16(Reg16::PC, addr);
        }
    }

    fn jr(self, offset: i8) {
        let (cpu, _) = self;
        let addr = cpu.registers.read16(Reg16::PC).wrapping_add(offset as u16);
        cpu.registers.write16(Reg16::PC, addr);
    }

    fn ret(self, addr: u16) {
        let (cpu, _) = self;
        cpu.registers.write16(Reg16::PC, addr);
    }

    fn push16(self, reg: Reg16) {
        let (cpu, bus) = self;
        let val = cpu.registers.read16(reg);
        cpu.push_u16(bus, val);
    }

    fn pop16(self, reg: Reg16) {
        let (cpu, bus) = self;
        let val = cpu.pop_u16(bus);
        cpu.registers.write16(reg, val);
    }

    fn rl(self, reg: Reg8, set_zero: bool) {
        let (cpu, _) = self;

        let reg_val = cpu.registers.read8(reg);
        let carry_val = if cpu.registers.f.contains(Flags::CARRY) {
            1
        } else {
            0
        };

        let new_carry_val = reg_val & 0x80;
        let new_reg_val = (reg_val << 1) | carry_val;

        cpu.registers.f = Flags::ZERO.self_or_empty(set_zero && new_reg_val == 0)
            | Flags::CARRY.self_or_empty(new_carry_val != 0);

        cpu.registers.write8(reg, new_reg_val);
    }

    fn inc16(self, reg: Reg16) {
        let (cpu, _) = self;
        let val = cpu.registers.read16(reg);

        cpu.registers.write16(reg, val.wrapping_add(1));
    }

    fn sub(self, val: u8) {
        let (cpu, _) = self;
        let carry_val = if false && cpu.registers.f.contains(Flags::CARRY) {
            1
        } else {
            0
        };

        let reg_val = cpu.registers.read8(Reg8::A);
        let sub_res = reg_val.wrapping_sub(val).wrapping_sub(carry_val);

        cpu.registers.f = Flags::ZERO.self_or_empty(sub_res == 0)
            | Flags::ADD_SUB
            | Flags::CARRY.self_or_empty((reg_val as u16) < (val as u16) + (carry_val as u16))
            | Flags::HALF_CARRY.self_or_empty((reg_val & 0xf) < (val & 0xf) + carry_val);
    }

    fn prefix_cb(self) -> Instruction {
        let (cpu, bus) = self;
        let opcode = cpu.next_u8(bus);

        cpu.decode_cb(opcode, bus).execute((cpu, bus))
    }
}
