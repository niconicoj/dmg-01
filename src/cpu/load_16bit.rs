use super::Register;
use super::CPU;

impl CPU {
    pub fn ld_dd_nn(&mut self, dd: u8) {
        match dd {
            0b00 => {
                self.registers[Register::B] = self.memory[self.program_counter as usize];
                self.registers[Register::C] = self.memory[(self.program_counter as usize) + 1];
            }
            0b01 => {
                self.registers[Register::D] = self.memory[self.program_counter as usize];
                self.registers[Register::E] = self.memory[(self.program_counter as usize) + 1];
            }
            0b10 => {
                self.registers[Register::H] = self.memory[self.program_counter as usize];
                self.registers[Register::L] = self.memory[(self.program_counter as usize) + 1];
            }
            0b11 => {
                self.stack_pointer = u16::from_be_bytes([
                    self.memory[self.program_counter as usize],
                    self.memory[(self.program_counter as usize) + 1],
                ]);
            }
            _ => unreachable!(),
        }
        self.program_counter += 2;
    }

    pub fn ld_sp_hl(&mut self) {
        self.stack_pointer =
            u16::from_be_bytes([self.registers[Register::H], self.registers[Register::L]])
    }

    pub fn push_qq(&mut self, qq: u8) {
        match qq {
            0b00 => {
                self.memory[(self.stack_pointer - 1) as usize] = self.registers[Register::B];
                self.memory[(self.stack_pointer - 2) as usize] = self.registers[Register::C];
            }
            0b01 => {
                self.memory[(self.stack_pointer - 1) as usize] = self.registers[Register::D];
                self.memory[(self.stack_pointer - 2) as usize] = self.registers[Register::E];
            }
            0b10 => {
                self.memory[(self.stack_pointer - 1) as usize] = self.registers[Register::H];
                self.memory[(self.stack_pointer - 2) as usize] = self.registers[Register::L];
            }
            0b11 => {
                self.memory[(self.stack_pointer - 1) as usize] = self.registers[Register::A];
                self.memory[(self.stack_pointer - 2) as usize] = self.registers[Register::F];
            }
            _ => unreachable!(),
        }
        self.stack_pointer -= 2;
    }

    pub fn pop_qq(&mut self, qq: u8) {
        match qq {
            0b00 => {
                self.registers[Register::C] = self.memory[self.stack_pointer as usize];
                self.registers[Register::B] = self.memory[(self.stack_pointer + 1) as usize];
            }
            0b01 => {
                self.registers[Register::E] = self.memory[self.stack_pointer as usize];
                self.registers[Register::D] = self.memory[(self.stack_pointer + 1) as usize];
            }
            0b10 => {
                self.registers[Register::L] = self.memory[self.stack_pointer as usize];
                self.registers[Register::H] = self.memory[(self.stack_pointer + 1) as usize];
            }
            0b11 => {
                self.registers[Register::F] = self.memory[self.stack_pointer as usize];
                self.registers[Register::A] = self.memory[(self.stack_pointer + 1) as usize];
            }
            _ => unreachable!(),
        }
        self.stack_pointer += 2;
    }

    pub fn ldhl_sp_e(&mut self) {
        // types are strictly the same size so transmute seems just fine
        let operand =
            unsafe { std::mem::transmute::<u8, i8>(self.memory[self.program_counter as usize]) };

        let (result, carry) = match operand.is_negative() {
            true => self
                .stack_pointer
                .overflowing_sub(operand.unsigned_abs() as u16),
            false => self
                .stack_pointer
                .overflowing_add(operand.unsigned_abs() as u16),
        };

        self.registers[Register::H] = result.to_be_bytes()[0];
        self.registers[Register::L] = result.to_be_bytes()[1];
        self.registers[Register::F] &= 0b00001111;
        if carry {
            self.registers[Register::F] |= 1 << 5;
        }
        if Self::half_carry(
            self.stack_pointer.to_be_bytes()[0],
            operand.unsigned_abs(),
            result.to_be_bytes()[0],
        ) {
            self.registers[Register::F] |= 1 << 4;
        }
        self.program_counter += 1;
    }

    pub fn ld_nn_sp(&mut self) {
        self.stack_pointer = u16::from_be_bytes([
            self.memory[self.program_counter as usize],
            self.memory[(self.program_counter + 1) as usize],
        ]);
        self.program_counter += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ld_dd_nn_tests() {
        let mut cpu = CPU::default();

        cpu.memory[0x0] = 0xCD;
        cpu.memory[0x1] = 0x1F;

        let instruction = 0b00_100_001;

        // load into Register
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::H], 0xCD);
        assert_eq!(cpu.registers[Register::L], 0x1F);
        assert_eq!(cpu.program_counter, 2);

        cpu.memory[0x2] = 0x57;
        cpu.memory[0x3] = 0xD3;

        // load into SP
        let instruction = 0b00_110_001;

        cpu.execute(instruction);

        assert_eq!(cpu.stack_pointer, 0x57D3);
        assert_eq!(cpu.program_counter, 4);
    }

    #[test]
    fn ld_sp_hl_tests() {
        let mut cpu = CPU::default();

        cpu.registers[Register::H] = 0x34;
        cpu.registers[Register::L] = 0x71;

        let instruction = 0b11_111_001;
        cpu.execute(instruction);

        assert_eq!(cpu.stack_pointer, 0x3471);
    }

    #[test]
    fn push_qq_tests() {
        let mut cpu = CPU::default();
        cpu.stack_pointer = 7;
        cpu.registers[Register::B] = 0x47;
        cpu.registers[Register::C] = 0xA5;

        let instruction = 0b11_000_101;
        cpu.execute(instruction);

        assert_eq!(cpu.memory[0x6], 0x47);
        assert_eq!(cpu.memory[0x5], 0xA5);
        assert_eq!(cpu.stack_pointer, 5);
    }

    #[test]
    fn pop_qq_tests() {
        let mut cpu = CPU::default();

        cpu.stack_pointer = 0x45B2;
        cpu.memory[0x45B2] = 0x01;
        cpu.memory[0x45B3] = 0xD5;

        let instruction = 0b11_010_001;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::D], 0xD5);
        assert_eq!(cpu.registers[Register::E], 0x01);
        assert_eq!(cpu.stack_pointer, 0x45B4);
    }

    #[test]
    fn ldhl_sp_e() {
        let mut cpu = CPU::default();

        cpu.stack_pointer = 0x45B2;
        cpu.memory[0x0] = 0x45;

        let instruction = 0b11_111_000;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::H], 0x45);
        assert_eq!(cpu.registers[Register::L], 0xF7);
        assert_eq!(cpu.registers[Register::F], 0b00000000);
        assert_eq!(cpu.program_counter, 0x1);

        cpu.stack_pointer = 0x0F2A;
        cpu.memory[0x1] = 0xF5;

        let instruction = 0b11_111_000;
        cpu.execute(instruction);

        assert_eq!(cpu.registers[Register::H], 0x0F);
        assert_eq!(cpu.registers[Register::L], 0x1F);
        assert_eq!(cpu.registers[Register::F], 0b00000000);
        assert_eq!(cpu.program_counter, 0x2);
    }

    #[test]
    fn ld_nn_sp_tests() {
        let mut cpu = CPU::default();

        cpu.memory[0x0] = 0x73;
        cpu.memory[0x1] = 0xE1;

        let instruction = 0b00_001_000;
        cpu.execute(instruction);

        assert_eq!(cpu.stack_pointer, 0x73E1);
        assert_eq!(cpu.program_counter, 0x2);
    }
}
