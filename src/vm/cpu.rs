use super::instruction::Instruction;
use super::memory::Memory;
use super::stack::Stack;
pub enum Flag {
    Zero,
    Negative,
    Positive,
}

#[derive(Debug)]
pub struct CPU {
    registers: [u32; 8],
    pc: usize,
    flag: Flag,
    memory: Memory,
    stack: Stack,
}

impl CPU {
    pub fn new(memory_size: usize) -> Self {
        CPU {
            registers: [0; 8],
            pc: 0,
            flag: Flag::Zero,
            memory: Memory::new(memory_size),
            stack: Stack::new(),
        }
    }

    pub fn fetch(&mut self) -> u32 {
        let instruction = self.memory.read(self.pc);
        self.pc += 1;
        instruction
    }

    pub fn decode(&self, instruction: u32) -> Instruction {
        Instruction::from(instruction)
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Load(reg, value) => self.registers[reg] = value,
            Instruction::Store(reg, address) => self.memory.write(address, self.registers[reg]),
            Instruction::Add(dest, src1, src2) => {
                self.registers[dest] = self.registers[src1].wrapping_add(self.registers[src2]);
                self.update_flag(self.registers[dest]);
            }
            Instruction::Sub(dest, src1, src2) => {
                self.registers[dest] = self.registers[src1].wrapping_sub(self.registers[src2]);
                self.update_flag(self.registers[dest]);
            }
            Instruction::Mul(dest, src1, src2) => {
                self.registers[dest] = self.registers[src1].wrapping_mul(self.registers[src2]);
                self.update_flag(self.registers[dest]);
            }
            Instruction::Div(dest, src1, src2) => {
                if self.registers[src2] != 0 {
                    self.registers[dest] = self.registers[src1] / self.registers[src2];
                    self.update_flag(self.registers[dest]);
                } else {
                    panic!("Division by zero");
                }
            }
            Instruction::Jump(address) => self.pc = address as usize,
            Instruction::JumpIfZero(address) => {
                if matches!(self.flag, Flag::Zero) {
                    self.pc = address as usize;
                }
            }
            Instruction::JumpIfNegative(address) => {
                if matches!(self.flag, Flag::Negative) {
                    self.pc = address as usize;
                }
            }
            Instruction::Push(reg) => self.stack.push(self.registers[reg]),
            Instruction::Pop(reg) => self.registers[reg] = self.stack.pop(),
            Instruction::Call(address) => {
                self.stack.push(self.pc as u32);
                self.pc = address as usize;
            }
            Instruction::Return => {
                self.pc = self.stack.pop() as usize;
            }
            Instruction::Halt => self.pc = usize::MAX,
        }
    }

    fn update_flag(&mut self, value: u32) {
        self.flag = if value == 0 {
            Flag::Zero
        } else if (value as i32) < 0 {
            Flag::Negative
        } else {
            Flag::Positive
        };
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch();
            let decoded = self.decode(instruction);
            self.execute(decoded);
            if self.pc == usize::MAX {
                break;
            }
        }
    }

    pub fn get_registers(&self) -> &[u32; 8] {
        &self.registers
    }

    pub fn get_pc(&self) -> usize {
        self.pc
    }

    pub fn get_flag(&self) -> &Flag {
        &self.flag
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }

    pub fn get_stack(&self) -> &Stack {
        &self.stack
    }
}
