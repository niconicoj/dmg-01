mod load_16bit;
mod load_8bit;

struct Register;
impl Register {
    const A: usize = 0b111;
    const B: usize = 0b000;
    const C: usize = 0b001;
    const D: usize = 0b010;
    const E: usize = 0b011;
    const F: usize = 0b110;
    const H: usize = 0b100;
    const L: usize = 0b101;
}

pub struct CPU {
    /// the code for each register is as follows :
    ///
    /// | Register | code |
    /// | A        | 111  |
    /// | B        | 000  |
    /// | C        | 001  |
    /// | D        | 010  |
    /// | E        | 011  |
    /// | F[^1]    | 110  |
    /// | H        | 100  |
    /// | L        | 101  |
    ///
    /// [^1] : F is not a regular register. Opcode where you might expect 110 to refer to this
    /// register usually refer to a complety different instruction.
    registers: [u8; 8],
    program_counter: u16,
    stack_pointer: u16,
    memory: Vec<u8>,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            registers: [0; 8],
            program_counter: 0,
            stack_pointer: 0,
            memory: vec![0; 0xFFFF],
        }
    }
}

impl CPU {
    pub fn run(&mut self) {
        // very naive main loop
        loop {
            let current = self.program_counter as usize;
            self.program_counter += 1;
            self.execute(self.memory[current]);
        }
    }

    fn execute(&mut self, opcode: u8) {
        let op = (opcode & 0b11000000) >> 6;
        let x = (opcode & 0b00111000) >> 3;
        let y = (opcode & 0b00000111) >> 0;

        #[rustfmt::skip]
        match (op, x, y) {
            (0b00, 0b000, 0b000) => self.nop(),
            (0b00, 0b110, 0b110) => self.ld_hl_n(),
            (0b00, 0b001, 0b010) => self.ld_a_ptr(Register::B, Register::C),
            (0b00, 0b011, 0b010) => self.ld_a_ptr(Register::D, Register::E),
            (0b00, 0b000, 0b010) => self.ld_ptr_a(Register::B, Register::C),
            (0b00, 0b010, 0b010) => self.ld_ptr_a(Register::D, Register::E),
            (0b00, 0b101, 0b010) => self.ld_a_hli(),
            (0b00, 0b111, 0b010) => self.ld_a_hld(),
            (0b00, 0b100, 0b010) => self.ld_hli_a(),
            (0b00, 0b110, 0b010) => self.ld_hld_a(),
            (0b00, 0b001, 0b000) => self.ld_nn_sp(),
            (0b00, dd   , 0b001) if (dd % 2 == 0) => self.ld_dd_nn(dd >> 1),
            (0b00, _    , 0b110) => self.ld_r_n(x),
            (0b01, 0b110, 0b110) => self.halt(),
            (0b01, _    , 0b110) => self.ld_r_hl(x),
            (0b01, 0b110, _    ) => self.ld_hl_r(y),
            (0b01, _    , _    ) => self.ld_rr(x, y),
            // (0b11, 0b110, 0b010) => self.ld_a_c(),
            (0b11, 0b100, 0b010) => self.ld_c_a(),
            (0b11, 0b110, 0b000) => self.ld_a_n(),
            (0b11, 0b100, 0b000) => self.ld_n_a(),
            (0b11, 0b111, 0b010) => self.ld_a_nn(),
            (0b11, 0b101, 0b010) => self.ld_nn_a(),
            (0b11, 0b111, 0b000) => self.ldhl_sp_e(),
            (0b11, 0b111, 0b001) => self.ld_sp_hl(),
            (0b11, qq   , 0b101) if (qq % 2 == 0) => self.push_qq(qq >> 1),
            (0b11, qq   , 0b001) if (qq % 2 == 0) => self.pop_qq(qq >> 1),
            _ => todo!("instruction {:08b} not yet supported", opcode)
        };
    }

    fn nop(&self) {}

    fn halt(&mut self) {
        todo!("halt")
    }

    fn half_carry(a: u8, b: u8, result: u8) -> bool {
        (a ^ b ^ result) & 0x10 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_carry_tests() {
        assert_eq!(CPU::half_carry(0x25, 0x48, 0x6D), false);
        assert_eq!(CPU::half_carry(0x39, 0x48, 0x81), true);
        assert_eq!(CPU::half_carry(0x72, 0x73, 0xE5), false);
    }
}
