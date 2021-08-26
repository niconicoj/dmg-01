use std::usize;

use crate::cpu::Register;
use crate::cpu::CPU;

impl CPU {
    pub fn add_a_r(&mut self, register: u8) {
        let (result, carry) =
            self.registers[Register::A].overflowing_add(self.registers[register as usize]);
        self.set_add_flags(
            self.registers[Register::A],
            self.registers[register as usize],
            result,
            carry,
        );
        self.registers[Register::A] = result;
    }

    pub fn add_a_n(&mut self) {
        let (result, carry) =
            self.registers[Register::A].overflowing_add(self.mmu.rb(self.program_counter));
        self.set_add_flags(
            self.registers[Register::A],
            self.mmu.rb(self.program_counter),
            result,
            carry,
        );
        self.registers[Register::A] = result;
        self.program_counter += 1;
    }

    pub fn add_a_hl(&mut self) {
        let memory_pointer =
            u16::from_be_bytes([self.registers[Register::H], self.registers[Register::L]]);
        let (result, carry) =
            self.registers[Register::A].overflowing_add(self.mmu.rb(memory_pointer));
        self.set_add_flags(
            self.registers[Register::A],
            self.mmu.rb(memory_pointer),
            result,
            carry,
        );
        self.registers[Register::A] = result;
        self.program_counter += 1;
    }

    fn set_add_flags(&mut self, a: u8, b: u8, result: u8, carry: bool) {
        self.registers[Register::F] = 0x0;
        if carry {
            self.registers[Register::F] |= 1 << 4;
        }
        if Self::half_carry(a, b, result) {
            self.registers[Register::F] |= 1 << 5;
        }
        if result == 0 {
            self.registers[Register::F] |= 1 << 7;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_a_r_tests() {
        let mut cpu = CPU::default();

        let instruction = 0b10000000;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::A], 0x0);
        assert_eq!(cpu.registers[Register::F], 0b10000000);

        cpu.registers[Register::B] = 0x45;

        let instruction = 0b10000000;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::A], 0x45);
        assert_eq!(cpu.registers[Register::F], 0b00000000);

        cpu.registers[Register::C] = 0xF4;

        let instruction = 0b10000001;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::A], 0x39);
        assert_eq!(cpu.registers[Register::F], 0b00010000);

        cpu.registers[Register::A] = 0x39;
        cpu.registers[Register::D] = 0x48;

        let instruction = 0b10000010;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::A], 0x81);
        assert_eq!(cpu.registers[Register::F], 0b00100000);
    }

    #[test]
    fn add_a_n_tests() {
        let mut cpu = CPU::default();

        cpu.mmu.wb(0x0, 0xF4);

        let instruction = 0b11000110;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::A], 0xF4);
        assert_eq!(cpu.registers[Register::F], 0b00000000);
    }

    #[test]
    fn add_a_hl_tests() {
        let mut cpu = CPU::default();

        cpu.registers[Register::H] = 0x27;
        cpu.registers[Register::L] = 0x1C;
        cpu.mmu.wb(0x271C, 0xF4);

        let instruction = 0b10_000_110;
        cpu.execute(instruction);

        assert_eq!(cpu.mmu.rb(0x271C), 0xF4);
    }
}
