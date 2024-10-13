pub struct Stack {
    data: Vec<u32>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, value: u32) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> u32 {
        self.data.pop().expect("Stack underflow")
    }

    pub fn peek(&self) -> Option<&u32> {
        self.data.last()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<u32> {
        self.data.iter()
    }
}
