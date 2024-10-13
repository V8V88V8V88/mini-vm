pub struct Memory {
    data: Vec<u32>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
        }
    }

    pub fn read(&self, address: usize) -> u32 {
        self.data[address]
    }

    pub fn write(&mut self, address: usize, value: u32) {
        self.data[address] = value;
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn load_program(&mut self, program: &[u32]) {
        self.data[..program.len()].copy_from_slice(program);
    }
}
