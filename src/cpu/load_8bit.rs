use crate::cpu::Register;
use crate::cpu::CPU;

impl CPU {
    pub fn ld_hl_n(&mut self) {
        let memory_pointer =
            u16::from_be_bytes([self.registers[Register::H], self.registers[Register::L]]);
        self.mmu
            .wb(memory_pointer, self.mmu.rb(self.program_counter));
        self.program_counter = self.program_counter.overflowing_add(1).0;
    }

    pub fn ld_a_ptr(&mut self, upper: usize, lower: usize) {
        let memory_pointer = u16::from_be_bytes([self.registers[upper], self.registers[lower]]);
        self.registers[Register::A] = self.mmu.rb(memory_pointer);
    }

    pub fn ld_ptr_a(&mut self, upper: usize, lower: usize) {
        let memory_pointer = u16::from_be_bytes([self.registers[upper], self.registers[lower]]);
        self.mmu.wb(memory_pointer, self.registers[Register::A])
    }

    pub fn ld_a_hli(&mut self) {
        let mut memory_pointer =
            u16::from_be_bytes([self.registers[Register::H], self.registers[Register::L]]);
        self.registers[Register::A] = self.mmu.rb(memory_pointer);
        memory_pointer = memory_pointer.overflowing_add(1).0;
        let pointer_bytes = memory_pointer.to_be_bytes();
        self.registers[Register::H] = pointer_bytes[0];
        self.registers[Register::L] = pointer_bytes[1];
    }

    pub fn ld_a_hld(&mut self) {
        let mut memory_pointer =
            u16::from_be_bytes([self.registers[Register::H], self.registers[Register::L]]);
        self.registers[Register::A] = self.mmu.rb(memory_pointer);
        memory_pointer = memory_pointer.overflowing_sub(1).0;
        let pointer_bytes = memory_pointer.to_be_bytes();
        self.registers[Register::H] = pointer_bytes[0];
        self.registers[Register::L] = pointer_bytes[1];
    }

    pub fn ld_hli_a(&mut self) {
        let mut memory_pointer =
            u16::from_be_bytes([self.registers[Register::H], self.registers[Register::L]]);
        self.mmu.wb(memory_pointer, self.registers[Register::A]);
        memory_pointer = memory_pointer.overflowing_add(1).0;
        let pointer_bytes = memory_pointer.to_be_bytes();
        self.registers[Register::H] = pointer_bytes[0];
        self.registers[Register::L] = pointer_bytes[1];
    }

    pub fn ld_hld_a(&mut self) {
        let mut memory_pointer =
            u16::from_be_bytes([self.registers[Register::H], self.registers[Register::L]]);
        self.mmu.wb(memory_pointer, self.registers[Register::A]);
        memory_pointer = memory_pointer.overflowing_sub(1).0;
        let pointer_bytes = memory_pointer.to_be_bytes();
        self.registers[Register::H] = pointer_bytes[0];
        self.registers[Register::L] = pointer_bytes[1];
    }

    pub fn ld_r_n(&mut self, x: u8) {
        self.registers[x as usize] = self.mmu.rb(self.program_counter);
        self.program_counter = self.program_counter.overflowing_add(1).0;
    }

    pub fn ld_r_hl(&mut self, x: u8) {
        let memory_pointer =
            u16::from_be_bytes([self.registers[Register::H], self.registers[Register::L]]);
        self.registers[x as usize] = self.mmu.rb(memory_pointer);
    }

    pub fn ld_hl_r(&mut self, x: u8) {
        let memory_pointer =
            u16::from_be_bytes([self.registers[Register::H], self.registers[Register::L]]);
        self.mmu.wb(memory_pointer, self.registers[x as usize]);
    }

    pub fn ld_rr(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    /*
        fn ld_a_c(&mut self) {
            let memory_pointer = u16::from_be_bytes([0xFF, self.registers[Register::C]]);
            self.registers[Register::A] = self.memory[memory_pointer as usize];
        }
    */

    pub fn ld_c_a(&mut self) {
        let memory_pointer = u16::from_be_bytes([0xFF, self.registers[Register::C]]);
        self.mmu.wb(memory_pointer, self.registers[Register::A])
    }

    pub fn ld_a_n(&mut self) {
        let memory_pointer = u16::from_be_bytes([0xFF, self.mmu.rb(self.program_counter)]);
        self.registers[Register::A] = self.mmu.rb(memory_pointer);
        self.program_counter = self.program_counter.overflowing_add(1).0;
    }

    pub fn ld_n_a(&mut self) {
        let memory_pointer = u16::from_be_bytes([0xFF, self.mmu.rb(self.program_counter)]);
        self.mmu.wb(memory_pointer, self.registers[Register::A]);
        self.program_counter = self.program_counter.overflowing_add(1).0;
    }

    pub fn ld_a_nn(&mut self) {
        let memory_pointer = u16::from_be_bytes([
            self.mmu.rb(self.program_counter),
            self.mmu.rb(self.program_counter + 1),
        ]);
        self.registers[Register::A] = self.mmu.rb(memory_pointer);
        self.program_counter = self.program_counter.overflowing_add(2).0;
    }

    pub fn ld_nn_a(&mut self) {
        let memory_pointer = u16::from_be_bytes([
            self.mmu.rb(self.program_counter),
            self.mmu.rb(self.program_counter + 1),
        ]);
        self.mmu.wb(memory_pointer, self.registers[Register::A]);
        self.program_counter = self.program_counter.overflowing_add(2).0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ld_hl_n_tests() {
        let mut cpu = CPU::default();
        cpu.mmu.wb(0, 0x34);
        cpu.registers[Register::H] = 0xA6;
        cpu.registers[Register::L] = 0xB7;

        let instruction = 0b00110110;
        cpu.execute(instruction);

        assert_eq!(cpu.mmu.rb(0xA6B7), 0x34);
        assert_eq!(cpu.program_counter, 1);
    }

    #[test]
    fn ld_a_ptr_tests() {
        let mut cpu = CPU::default();
        cpu.mmu.wb(0xA6B7, 0x34);
        cpu.mmu.wb(0x56BA, 0x56);
        cpu.registers[Register::B] = 0xA6;
        cpu.registers[Register::C] = 0xB7;
        cpu.registers[Register::D] = 0x56;
        cpu.registers[Register::E] = 0xBA;

        let instruction = 0b00001010;
        cpu.execute(instruction);
        assert_eq!(cpu.registers[Register::A], 0x34);

        let instruction = 0b00011010;
        cpu.execute(instruction);
        assert_eq!(cpu.registers[Register::A], 0x56);
    }

    #[test]
    fn ld_ptr_a_tests() {
        let mut cpu = CPU::default();
        cpu.registers[Register::A] = 0xF3;
        cpu.registers[Register::B] = 0xA6;
        cpu.registers[Register::C] = 0xB7;
        cpu.registers[Register::D] = 0x56;
        cpu.registers[Register::E] = 0xBA;

        let instruction = 0b00_000_010;
        cpu.execute(instruction);

        assert_eq!(cpu.mmu.rb(0xA6B7), 0xF3);

        let instruction = 0b00_010_010;
        cpu.execute(instruction);
        assert_eq!(cpu.mmu.rb(0x56BA), 0xF3);
    }

    #[test]
    fn ld_a_hli_tests() {
        let mut cpu = CPU::default();

        cpu.registers[Register::H] = 0x47;
        cpu.registers[Register::L] = 0x34;
        cpu.mmu.wb(0x4734, 0x7B);

        let instruction = 0b00_101_010;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::A], 0x7B);
        assert_eq!(cpu.registers[Register::H], 0x47);
        assert_eq!(cpu.registers[Register::L], 0x35);
    }

    #[test]
    fn ld_a_hld_tests() {
        let mut cpu = CPU::default();

        cpu.registers[Register::H] = 0x47;
        cpu.registers[Register::L] = 0x34;
        cpu.mmu.wb(0x4734, 0x7B);

        let instruction = 0b00_111_010;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::A], 0x7B);
        assert_eq!(cpu.registers[Register::H], 0x47);
        assert_eq!(cpu.registers[Register::L], 0x33);
    }

    #[test]
    fn ld_hli_a_tests() {
        let mut cpu = CPU::default();

        cpu.registers[Register::A] = 0xF3;
        cpu.registers[Register::H] = 0xD0;
        cpu.registers[Register::L] = 0x5B;

        let instruction = 0b00_100_010;
        cpu.execute(instruction);

        assert_eq!(cpu.mmu.rb(0xD05B), 0xF3);
        assert_eq!(cpu.registers[Register::H], 0xD0);
        assert_eq!(cpu.registers[Register::L], 0x5C);
    }

    #[test]
    fn ld_hld_a_tests() {
        let mut cpu = CPU::default();

        cpu.registers[Register::A] = 0xF3;
        cpu.registers[Register::H] = 0xD0;
        cpu.registers[Register::L] = 0x5B;

        let instruction = 0b00_110_010;
        cpu.execute(instruction);

        assert_eq!(cpu.mmu.rb(0xD05B), 0xF3);
        assert_eq!(cpu.registers[Register::H], 0xD0);
        assert_eq!(cpu.registers[Register::L], 0x5A);
    }

    #[test]
    fn ld_r_n_tests() {
        let mut cpu = CPU::default();
        cpu.mmu.wb(0x0, 0x43);

        let instruction = 0b00_010_110;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[0b010], 0x43);
    }

    #[test]
    fn ld_r_hl_tests() {
        let mut cpu = CPU::default();
        cpu.mmu.wb(0xA6B7, 0x34);
        cpu.registers[Register::H] = 0xA6;
        cpu.registers[Register::L] = 0xB7;

        let instruction = 0b01_010_110;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[0b010], 0x34);
    }

    #[test]
    fn ld_hl_r_tests() {
        let mut cpu = CPU::default();
        cpu.registers[0b010] = 0x34;
        cpu.registers[Register::H] = 0xA6;
        cpu.registers[Register::L] = 0xB7;

        let instruction = 0b01_110_010;
        cpu.execute(instruction);

        assert_eq!(cpu.mmu.rb(0xA6B7), 0x34);
    }

    #[test]
    fn ld_rr_tests() {
        let mut cpu = CPU::default();
        cpu.registers[0b011] = 0xAC;

        let instruction = 0b01_010_011;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[0b010], 0xAC);
    }

    /*
        #[test]
        fn ld_a_c_tests() {
            let mut cpu = CPU::default();
            cpu.registers[Register::C] = 0xF1;
            cpu.memory[0xFFF1] = 0x5B;

            let instruction = 0b11110010;
            cpu.execute(instruction);

            assert_eq!(cpu.registers[Register::A], 0x5B);
        }
    */

    #[test]
    fn ld_c_a_tests() {
        let mut cpu = CPU::default();
        cpu.registers[Register::C] = 0xF1;
        cpu.registers[Register::A] = 0xB5;

        let instruction = 0b11100010;
        cpu.execute(instruction);

        assert_eq!(cpu.mmu.rb(0xFFF1), 0xB5);
    }

    #[test]
    fn ld_a_n_tests() {
        let mut cpu = CPU::default();
        cpu.mmu.wb(0x0, 0x5B);
        cpu.mmu.wb(0xFF5B, 0x11);

        let instruction = 0b11110000;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::A], 0x11);
        assert_eq!(cpu.program_counter, 1);
    }

    #[test]
    fn ld_n_a_tests() {
        let mut cpu = CPU::default();
        cpu.registers[Register::A] = 0xB5;
        cpu.mmu.wb(0x0, 0x12);

        let instruction = 0b11100000;
        cpu.execute(instruction);

        assert_eq!(cpu.mmu.rb(0xFF12), 0xB5);
        assert_eq!(cpu.program_counter, 1);
    }

    #[test]
    fn ld_a_nn_tests() {
        let mut cpu = CPU::default();
        cpu.mmu.wb(0x0, 0x34);
        cpu.mmu.wb(0x1, 0xF5);
        cpu.mmu.wb(0x34F5, 0x78);

        let instruction = 0b11_111_010;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::A], 0x78);
        assert_eq!(cpu.program_counter, 2);
    }

    #[test]
    fn ld_nn_a_tests() {
        let mut cpu = CPU::default();
        cpu.mmu.wb(0x0, 0x34);
        cpu.mmu.wb(0x1, 0xF5);
        cpu.registers[Register::A] = 0x78;

        let instruction = 0b11_101_010;
        cpu.execute(instruction);

        assert_eq!(cpu.mmu.rb(0x34F5), 0x78);
        assert_eq!(cpu.program_counter, 2);
    }
}
