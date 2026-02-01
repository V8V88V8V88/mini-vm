use super::instruction::Opcode;
use super::memory::Memory;
use super::trap::{execute_trap, Trap};

pub struct CPU {
    pub ip: usize,
    pub stack: Vec<i64>,
    pub call_stack: Vec<usize>,
    pub memory: Memory,
    pub halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            ip: 0,
            stack: Vec::with_capacity(1024),
            call_stack: Vec::with_capacity(256),
            memory: Memory::new(),
            halted: false,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.load_program(program, 0);
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory.read_byte(self.ip);
        self.ip += 1;
        byte
    }

    fn fetch_u64(&mut self) -> u64 {
        let val = self.memory.read_u64(self.ip);
        self.ip += 8;
        val
    }

    fn pop(&mut self) -> i64 {
        self.stack.pop().unwrap_or(0)
    }

    fn push(&mut self, val: i64) {
        self.stack.push(val);
    }

    pub fn step(&mut self) -> bool {
        if self.halted {
            return false;
        }

        let opcode_byte = self.fetch_byte();
        let opcode = match Opcode::from_byte(opcode_byte) {
            Some(op) => op,
            None => {
                self.halted = true;
                return false;
            }
        };

        match opcode {
            Opcode::Nop => {}

            Opcode::Halt => {
                self.halted = true;
            }

            Opcode::Push => {
                let val = self.fetch_u64() as i64;
                self.push(val);
            }

            Opcode::Pop => {
                self.pop();
            }

            Opcode::Dup => {
                if let Some(&val) = self.stack.last() {
                    self.push(val);
                }
            }

            Opcode::Swap => {
                let len = self.stack.len();
                if len >= 2 {
                    self.stack.swap(len - 1, len - 2);
                }
            }

            Opcode::Load => {
                let addr = self.pop() as usize;
                let val = self.memory.read_u64(addr) as i64;
                self.push(val);
            }

            Opcode::Store => {
                let addr = self.pop() as usize;
                let val = self.pop();
                self.memory.write_u64(addr, val as u64);
            }

            Opcode::Add => {
                let b = self.pop();
                let a = self.pop();
                self.push(a.wrapping_add(b));
            }

            Opcode::Sub => {
                let b = self.pop();
                let a = self.pop();
                self.push(a.wrapping_sub(b));
            }

            Opcode::Mul => {
                let b = self.pop();
                let a = self.pop();
                self.push(a.wrapping_mul(b));
            }

            Opcode::Div => {
                let b = self.pop();
                let a = self.pop();
                if b != 0 {
                    self.push(a / b);
                } else {
                    self.push(0);
                }
            }

            Opcode::Mod => {
                let b = self.pop();
                let a = self.pop();
                if b != 0 {
                    self.push(a % b);
                } else {
                    self.push(0);
                }
            }

            Opcode::And => {
                let b = self.pop();
                let a = self.pop();
                self.push(a & b);
            }

            Opcode::Or => {
                let b = self.pop();
                let a = self.pop();
                self.push(a | b);
            }

            Opcode::Not => {
                let a = self.pop();
                self.push(!a);
            }

            Opcode::Eq => {
                let b = self.pop();
                let a = self.pop();
                self.push(if a == b { 1 } else { 0 });
            }

            Opcode::Lt => {
                let b = self.pop();
                let a = self.pop();
                self.push(if a < b { 1 } else { 0 });
            }

            Opcode::Gt => {
                let b = self.pop();
                let a = self.pop();
                self.push(if a > b { 1 } else { 0 });
            }

            Opcode::Jmp => {
                let addr = self.fetch_u64() as usize;
                self.ip = addr;
            }

            Opcode::Jz => {
                let addr = self.fetch_u64() as usize;
                let val = self.pop();
                if val == 0 {
                    self.ip = addr;
                }
            }

            Opcode::Jnz => {
                let addr = self.fetch_u64() as usize;
                let val = self.pop();
                if val != 0 {
                    self.ip = addr;
                }
            }

            Opcode::Call => {
                let addr = self.fetch_u64() as usize;
                self.call_stack.push(self.ip);
                self.ip = addr;
            }

            Opcode::Ret => {
                if let Some(addr) = self.call_stack.pop() {
                    self.ip = addr;
                } else {
                    self.halted = true;
                }
            }

            Opcode::Trap => {
                let trap_code = self.fetch_byte();
                if let Some(trap) = Trap::from_byte(trap_code) {
                    execute_trap(trap, &mut self.stack, self.memory.as_slice());
                }
            }
        }

        !self.halted
    }

    pub fn run(&mut self) {
        while self.step() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut cpu = CPU::new();
        cpu.load_program(&vec![Opcode::Push as u8, 10, 0, 0, 0, 0, 0, 0, 0, Opcode::Halt as u8]);
        cpu.run();
        assert_eq!(cpu.stack.len(), 1);
        assert_eq!(cpu.stack[0], 10);
    }

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        cpu.load_program(&vec![
            Opcode::Push as u8, 5, 0, 0, 0, 0, 0, 0, 0,
            Opcode::Push as u8, 3, 0, 0, 0, 0, 0, 0, 0,
            Opcode::Add as u8,
            Opcode::Halt as u8
        ]);
        cpu.run();
        assert_eq!(cpu.stack.last(), Some(&8));
    }

    #[test]
    fn test_jmp() {
        let mut cpu = CPU::new();
        cpu.load_program(&vec![
            Opcode::Jmp as u8, 18, 0, 0, 0, 0, 0, 0, 0, // jmp to halt at 18
            Opcode::Push as u8, 1, 0, 0, 0, 0, 0, 0, 0, // 9-17
            Opcode::Halt as u8                          // 18
        ]);
        cpu.run();
        assert_eq!(cpu.stack.len(), 0);
        assert_eq!(cpu.ip, 19);
    }

    #[test]
    fn test_call_ret() {
        let mut cpu = CPU::new();
        // 0: call 10
        // 9: halt
        // 10: push 42
        // 19: ret
        let mut program = vec![0; 32];
        program[0] = Opcode::Call as u8;
        let target = 10u64.to_le_bytes();
        program[1..9].copy_from_slice(&target);
        program[9] = Opcode::Halt as u8;
        program[10] = Opcode::Push as u8;
        let val = 42u64.to_le_bytes();
        program[11..19].copy_from_slice(&val);
        program[19] = Opcode::Ret as u8;

        cpu.load_program(&program);
        cpu.run();
        assert_eq!(cpu.stack.last(), Some(&42));
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}
