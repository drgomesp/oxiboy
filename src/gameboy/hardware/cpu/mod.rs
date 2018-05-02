mod instructions;
mod ops;
mod registers;

use self::instructions::*;
use self::registers::{Reg16, Reg8, Registers};

use self::ops::Ops;
use super::bus::Bus;

pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B) -> Instruction {
        let opcode = bus.read(self.registers.pc);
        let instr = self.decode(opcode, bus);

        self.registers.pc += 1;

        instr.execute((self, bus))
    }

    fn next_u8(&mut self, bus: &mut Bus) -> u8 {
        self.registers.pc += 1;
        bus.read(self.registers.pc)
    }

    fn next_u16(&mut self, bus: &mut Bus) -> u16 {
        let l = self.next_u8(bus);
        let h = self.next_u8(bus);
        ((h as u16) << 8) | (l as u16)
    }
}

impl InstructionDecoding for Cpu {
    fn decode<B: Bus>(&mut self, opcode: u8, bus: &mut B) -> Instruction {
        use self::Instruction::*;

        match opcode {
            0x0 => Nop,
            0x31 => Load16(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 3,
                    cycle_duration: 12,
                },
                Reg16::SP,
                self.next_u16(bus),
            ),
            0xAF => Xor(
                InstructionInfo {
                    opcode: opcode,
                    byte_length: 1,
                    cycle_duration: 4,
                },
                Reg8::A,
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
            _ => panic!("unrecognized opcode: {:#2x}", opcode),
        }
    }
}

impl<'a, B> Ops for (&'a mut Cpu, &'a mut B)
where
    B: Bus,
{
    fn nop(self) {}

    fn load16_imm(self, reg: Reg16, val: u16) {
        let (cpu, _) = self;
        cpu.registers.write16(reg, val)
    }

    fn xor(self, reg: Reg8) {
        let (cpu, _) = self;

        use self::Reg8::*;

        match reg {
            A => cpu.registers.a ^= cpu.registers.a,
            // _ => unreachable!("not implemented yet"),
        }
    }
}
