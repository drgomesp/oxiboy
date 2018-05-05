mod instructions;
mod ops;
pub mod registers;

use self::instructions::*;
use self::registers::{Flags, Reg16, Reg8, Registers};

use self::ops::Ops;
use super::bus::Bus;

pub struct LR35902 {
    pub registers: Registers,
}

impl LR35902 {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B) -> Instruction {
        let pc = self.registers.read16(Reg16::PC);
        let opcode = bus.read(pc);

        self.registers.write16(Reg16::PC, pc.wrapping_add(1));
        let instr = self.decode(opcode, bus);

        instr.execute((self, bus))
    }

    fn next_u8(&mut self, bus: &mut Bus) -> u8 {
        let pc = self.registers.read16(Reg16::PC);
        self.registers.write16(Reg16::PC, pc.wrapping_add(1));
        bus.read(pc)
    }

    fn next_u16(&mut self, bus: &mut Bus) -> u16 {
        let l = self.next_u8(bus);
        let h = self.next_u8(bus);
        ((h as u16) << 8) | (l as u16)
    }
}

impl InstructionDecoding for LR35902 {
    fn decode<B: Bus>(&mut self, opcode: u8, bus: &mut B) -> Instruction {
        use self::Instruction::*;

        match opcode {
            0x0E => Load8(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Reg8::C,
                self.next_u8(bus),
            ),
            0x11 => Load16(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 3,
                    cycle_duration: 12,
                },
                Reg16::DE,
                self.next_u16(bus),
            ),
            0x1A => Load8Reg16(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Reg8::A,
                Reg16::DE,
            ),
            0x20 => JumpOn(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 12,
                },
                JumpCondition::NZ,
                self.next_u8(bus) as i8,
            ),
            0x21 => Load16(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 3,
                    cycle_duration: 12,
                },
                Reg16::HL,
                self.next_u16(bus),
            ),
            0x31 => Load16(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 3,
                    cycle_duration: 12,
                },
                Reg16::SP,
                self.next_u16(bus),
            ),
            0x32 => Load(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Addr::HLD,
                Reg8::A,
            ),
            0x3E => Load8(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Reg8::A,
                self.next_u8(bus),
            ),
            0xAF => Xor(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Reg8::A,
            ),
            0xE2 => Load(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Addr::C,
                Reg8::A,
            ),
            0x0C => Inc(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Reg8::C,
            ),
            0x77 => Load(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 8,
                },
                Addr::HL,
                Reg8::A,
            ),
            0xE0 => Load(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 2,
                    cycle_duration: 12,
                },
                Addr::a8,
                Reg8::A,
            ),
            0xCB => PrefixCB,
            0x00 => Nop(InstructionInfo {
                opcode: 0x00,
                byte_length: 1,
                cycle_duration: 4,
            }),
            _ => panic!("unrecognized opcode: {:#04X}", opcode),
        }
    }

    fn decode_cb<B: Bus>(&mut self, opcode: u8, _: &mut B) -> Instruction {
        use self::Instruction::*;

        match opcode {
            0x7C => Bit(
                InstructionInfo {
                    opcode: 0x7C,
                    byte_length: 2,
                    cycle_duration: 8,
                },
                7,
                Reg8::H,
            ),
            _ => panic!("Unrecognized cb opcode: {:#04X}", opcode),
        }
    }
}

impl<'a, B> Ops for (&'a mut LR35902, &'a mut B)
where
    B: Bus,
{
    fn nop(self) {}

    fn bit(self, bit: usize, reg: Reg8) {
        let (cpu, _) = self;
        let val = cpu.registers.read8(reg) & (1 << bit);

        cpu.registers.f = if val == 0 {
            Flags::ZERO
        } else {
            Flags::empty()
        } | Flags::HALF_CARRY & cpu.registers.f;
    }

    fn inc(self, reg: Reg8) {
        let (cpu, _) = self;
        let val = cpu.registers.read8(reg);

        cpu.registers.write8(reg, val.wrapping_add(1))
    }

    fn load(self, addr: Addr, reg: Reg8) {
        let (cpu, bus) = self;
        use self::instructions::Addr::*;

        let indirect_addr = match addr {
            a8 => (0xFF00u16 | cpu.next_u8(bus) as u16) as u16,
            C => (0xFF00u16 | cpu.registers.read8(Reg8::C) as u16) as u16,
            HL => cpu.registers.read16(Reg16::HL),
            HLD => {
                let addr = cpu.registers.read16(Reg16::HL);
                cpu.registers.write16(Reg16::HL, addr - 1);
                addr
            }
            _ => unimplemented!(),
        };

        bus.write(indirect_addr, cpu.registers.read8(reg));
    }

    fn load8_imm(self, reg: Reg8, val: u8) {
        let (cpu, _) = self;
        cpu.registers.write8(reg, val);
    }

    fn load16_reg(self, reg8: Reg8, reg16: Reg16) {
        println!("load16_reg({:?},{:?})", reg8, reg16);
    }

    fn load16_imm(self, reg: Reg16, val: u16) {
        let (cpu, _) = self;
        cpu.registers.write16(reg, val);
    }

    fn xor(self, reg: Reg8) {
        let (cpu, _) = self;
        let v = cpu.registers.read8(reg);
        cpu.registers.write8(reg, v ^ v)
    }

    fn jr_c(self, cond: JumpCondition, offset: i8) {
        let (cpu, _) = self;

        if cond.check(cpu.registers.f) {
            let pc = cpu.registers.read16(Reg16::PC);
            cpu.registers
                .write16(Reg16::PC, pc.wrapping_add(offset as u16));
        }
    }

    fn prefix_cb(self) -> Instruction {
        let (cpu, bus) = self;
        let opcode = cpu.next_u8(bus);

        cpu.decode_cb(opcode, bus).execute((cpu, bus))
    }
}
