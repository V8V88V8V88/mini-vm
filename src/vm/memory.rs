pub const MEMORY_SIZE: usize = 65536;

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: vec![0; MEMORY_SIZE],
        }
    }

    pub fn load_program(&mut self, program: &[u8], offset: usize) {
        let end = (offset + program.len()).min(self.data.len());
        let len = end - offset;
        self.data[offset..end].copy_from_slice(&program[..len]);
    }

    pub fn read_byte(&self, addr: usize) -> u8 {
        self.data.get(addr).copied().unwrap_or(0)
    }

    pub fn write_byte(&mut self, addr: usize, val: u8) {
        if addr < self.data.len() {
            self.data[addr] = val;
        }
    }

    pub fn read_u64(&self, addr: usize) -> u64 {
        let mut bytes = [0u8; 8];
        for i in 0..8 {
            bytes[i] = self.read_byte(addr + i);
        }
        u64::from_le_bytes(bytes)
    }

    pub fn write_u64(&mut self, addr: usize, val: u64) {
        let bytes = val.to_le_bytes();
        for i in 0..8 {
            self.write_byte(addr + i, bytes[i]);
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
