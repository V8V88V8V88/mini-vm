#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Load(usize, u32),
    Store(usize, usize),
    Add(usize, usize, usize),
    Sub(usize, usize, usize),
    Mul(usize, usize, usize),
    Div(usize, usize, usize),
    Jump(u32),
    JumpIfZero(u32),
    JumpIfNegative(u32),
    Push(usize),
    Pop(usize),
    Call(u32),
    Return,
    Halt,
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let opcode = (value >> 28) & 0xF;
        let reg1 = ((value >> 24) & 0xF) as usize;
        let reg2 = ((value >> 20) & 0xF) as usize;
        let reg3 = ((value >> 16) & 0xF) as usize;
        let immediate = value & 0xFFFF;

        match opcode {
            0 => Instruction::Load(reg1, immediate),
            1 => Instruction::Store(reg1, immediate as usize),
            2 => Instruction::Add(reg1, reg2, reg3),
            3 => Instruction::Sub(reg1, reg2, reg3),
            4 => Instruction::Mul(reg1, reg2, reg3),
            5 => Instruction::Div(reg1, reg2, reg3),
            6 => Instruction::Jump(immediate),
            7 => Instruction::JumpIfZero(immediate),
            8 => Instruction::JumpIfNegative(immediate),
            9 => Instruction::Push(reg1),
            10 => Instruction::Pop(reg1),
            11 => Instruction::Call(immediate),
            12 => Instruction::Return,
            15 => Instruction::Halt,
            _ => panic!("Invalid instruction"),
        }
    }
}
