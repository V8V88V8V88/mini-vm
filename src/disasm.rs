use crate::vm::Opcode;

pub struct Disassembler {
    data: Vec<u8>,
    pos: usize,
}

impl Disassembler {
    pub fn new(data: Vec<u8>) -> Self {
        Disassembler { data, pos: 0 }
    }

    pub fn disassemble(&mut self) -> Vec<(usize, String)> {
        let mut output = Vec::new();
        
        while self.pos < self.data.len() {
            let addr = self.pos;
            if let Some(line) = self.decode_instruction() {
                output.push((addr, line));
            } else {
                break;
            }
        }
        
        output
    }

    pub fn disassemble_at(&self, addr: usize) -> Option<(String, usize)> {
        if addr >= self.data.len() {
            return None;
        }

        let opcode_byte = self.data[addr];
        let opcode = Opcode::from_byte(opcode_byte)?;

        let (instruction, size) = match opcode {
            Opcode::Nop | Opcode::Halt | Opcode::Pop | Opcode::Dup | Opcode::Swap |
            Opcode::Load | Opcode::Store | Opcode::Add | Opcode::Sub | Opcode::Mul |
            Opcode::Div | Opcode::Mod | Opcode::And | Opcode::Or | Opcode::Not |
            Opcode::Eq | Opcode::Lt | Opcode::Gt | Opcode::Ret => (opcode.name().to_string(), 1),

            Opcode::Push => {
                let val = self.read_u64_at(addr + 1)?;
                (format!("push {}", val as i64), 9)
            }

            Opcode::Jmp => {
                let target = self.read_u64_at(addr + 1)?;
                (format!("jmp 0x{:04x}", target), 9)
            }

            Opcode::Jz => {
                let target = self.read_u64_at(addr + 1)?;
                (format!("jz 0x{:04x}", target), 9)
            }

            Opcode::Jnz => {
                let target = self.read_u64_at(addr + 1)?;
                (format!("jnz 0x{:04x}", target), 9)
            }

            Opcode::Call => {
                let target = self.read_u64_at(addr + 1)?;
                (format!("call 0x{:04x}", target), 9)
            }

            Opcode::Trap => {
                let trap_code = *self.data.get(addr + 1)?;
                let trap_name = match trap_code {
                    0x01 => "putc",
                    0x02 => "getc",
                    0x03 => "puts",
                    0x04 => "putu",
                    0x05 => "puti",
                    _ => "trap?",
                };
                (trap_name.to_string(), 2)
            }
        };

        Some((instruction, size))
    }

    fn decode_instruction(&mut self) -> Option<String> {
        let addr = self.pos;
        let (instruction, size) = self.disassemble_at(addr)?;
        self.pos += size;
        Some(instruction)
    }

    fn read_u64_at(&self, addr: usize) -> Option<u64> {
        if addr + 8 > self.data.len() {
            return None;
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.data[addr..addr + 8]);
        Some(u64::from_le_bytes(bytes))
    }
}

pub fn disassemble_program(data: &[u8]) -> String {
    let mut disasm = Disassembler::new(data.to_vec());
    let instructions = disasm.disassemble();
    
    let mut output = String::new();
    for (addr, instruction) in instructions {
        output.push_str(&format!("{:04x}:  {}\n", addr, instruction));
    }
    output
}
