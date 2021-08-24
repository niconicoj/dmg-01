pub struct MMU {
    memory: Vec<u8>,
}

impl Default for MMU {
    fn default() -> Self {
        Self {
            memory: vec![0; 0xFFFF],
        }
    }
}

impl MMU {
    /// read a byte in memory
    pub fn rb(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    /// write a byte in memory
    pub fn wb(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    /// read a 16bit word in memory
    pub fn rw(&self, addr: u16) -> u16 {
        u16::from_be_bytes([self.memory[addr as usize], self.memory[(addr + 1) as usize]])
    }

    /// write a 16bit word in memory
    pub fn ww(&mut self, addr: u16, value: u16) {
        self.memory[addr as usize] = value.to_be_bytes()[0];
        self.memory[(addr + 1) as usize] = value.to_be_bytes()[1];
    }
}
