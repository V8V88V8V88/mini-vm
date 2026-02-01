use std::collections::HashMap;
use crate::vm::Opcode;

pub struct Assembler {
    labels: HashMap<String, usize>,
    output: Vec<u8>,
    unresolved: Vec<(usize, String)>,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            labels: HashMap::new(),
            output: Vec::new(),
            unresolved: Vec::new(),
        }
    }

    pub fn assemble(&mut self, source: &str) -> Result<Vec<u8>, String> {
        self.labels.clear();
        self.output.clear();
        self.unresolved.clear();

        for (line_num, line) in source.lines().enumerate() {
            self.process_line(line, line_num + 1)?;
        }

        self.resolve_labels()?;
        Ok(self.output.clone())
    }

    fn process_line(&mut self, line: &str, line_num: usize) -> Result<(), String> {
        let line = line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            return Ok(());
        }

        if line.ends_with(':') {
            let label = line.trim_end_matches(':').to_string();
            self.labels.insert(label, self.output.len());
            return Ok(());
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        let mnemonic = parts[0].to_lowercase();
        let operand = parts.get(1).map(|s| *s);

        match mnemonic.as_str() {
            "nop" => self.emit_byte(Opcode::Nop as u8),
            "halt" => self.emit_byte(Opcode::Halt as u8),
            "pop" => self.emit_byte(Opcode::Pop as u8),
            "dup" => self.emit_byte(Opcode::Dup as u8),
            "swap" => self.emit_byte(Opcode::Swap as u8),
            "load" => self.emit_byte(Opcode::Load as u8),
            "store" => self.emit_byte(Opcode::Store as u8),
            "add" => self.emit_byte(Opcode::Add as u8),
            "sub" => self.emit_byte(Opcode::Sub as u8),
            "mul" => self.emit_byte(Opcode::Mul as u8),
            "div" => self.emit_byte(Opcode::Div as u8),
            "mod" => self.emit_byte(Opcode::Mod as u8),
            "and" => self.emit_byte(Opcode::And as u8),
            "or" => self.emit_byte(Opcode::Or as u8),
            "not" => self.emit_byte(Opcode::Not as u8),
            "eq" => self.emit_byte(Opcode::Eq as u8),
            "lt" => self.emit_byte(Opcode::Lt as u8),
            "gt" => self.emit_byte(Opcode::Gt as u8),
            "ret" => self.emit_byte(Opcode::Ret as u8),

            "push" => {
                let val = self.parse_operand(operand, line_num)?;
                self.emit_byte(Opcode::Push as u8);
                self.emit_u64(val);
            }

            "jmp" => {
                self.emit_byte(Opcode::Jmp as u8);
                self.emit_label_or_addr(operand, line_num)?;
            }

            "jz" => {
                self.emit_byte(Opcode::Jz as u8);
                self.emit_label_or_addr(operand, line_num)?;
            }

            "jnz" => {
                self.emit_byte(Opcode::Jnz as u8);
                self.emit_label_or_addr(operand, line_num)?;
            }

            "call" => {
                self.emit_byte(Opcode::Call as u8);
                self.emit_label_or_addr(operand, line_num)?;
            }

            "putc" => {
                self.emit_byte(Opcode::Trap as u8);
                self.emit_byte(0x01);
            }

            "getc" => {
                self.emit_byte(Opcode::Trap as u8);
                self.emit_byte(0x02);
            }

            "puts" => {
                self.emit_byte(Opcode::Trap as u8);
                self.emit_byte(0x03);
            }

            "putu" => {
                self.emit_byte(Opcode::Trap as u8);
                self.emit_byte(0x04);
            }

            "puti" => {
                self.emit_byte(Opcode::Trap as u8);
                self.emit_byte(0x05);
            }

            "db" => {
                if let Some(op) = operand {
                    if op.starts_with('"') && op.ends_with('"') {
                        let s = &op[1..op.len()-1];
                        for c in s.bytes() {
                            self.emit_byte(c);
                        }
                        self.emit_byte(0);
                    } else {
                        let val = self.parse_number(op, line_num)?;
                        self.emit_byte(val as u8);
                    }
                }
            }

            _ => return Err(format!("line {}: unknown instruction '{}'", line_num, mnemonic)),
        }

        Ok(())
    }

    fn emit_byte(&mut self, byte: u8) {
        self.output.push(byte);
    }

    fn emit_u64(&mut self, val: u64) {
        self.output.extend_from_slice(&val.to_le_bytes());
    }

    fn emit_label_or_addr(&mut self, operand: Option<&str>, line_num: usize) -> Result<(), String> {
        let op = operand.ok_or_else(|| format!("line {}: missing operand", line_num))?;

        if let Ok(addr) = self.parse_number(op, line_num) {
            self.emit_u64(addr);
        } else {
            self.unresolved.push((self.output.len(), op.to_string()));
            self.emit_u64(0);
        }
        Ok(())
    }

    fn parse_operand(&self, operand: Option<&str>, line_num: usize) -> Result<u64, String> {
        let op = operand.ok_or_else(|| format!("line {}: missing operand", line_num))?;
        self.parse_number(op, line_num)
    }

    fn parse_number(&self, s: &str, line_num: usize) -> Result<u64, String> {
        let s = s.trim();
        if s.starts_with("0x") || s.starts_with("0X") {
            u64::from_str_radix(&s[2..], 16)
                .map_err(|_| format!("line {}: invalid hex number '{}'", line_num, s))
        } else if s.starts_with('-') {
            s.parse::<i64>()
                .map(|v| v as u64)
                .map_err(|_| format!("line {}: invalid number '{}'", line_num, s))
        } else {
            s.parse::<u64>()
                .map_err(|_| format!("line {}: invalid number '{}'", line_num, s))
        }
    }

    fn resolve_labels(&mut self) -> Result<(), String> {
        for (pos, label) in &self.unresolved {
            let addr = self.labels.get(label)
                .ok_or_else(|| format!("undefined label '{}'", label))?;
            let bytes = (*addr as u64).to_le_bytes();
            for i in 0..8 {
                self.output[pos + i] = bytes[i];
            }
        }
        Ok(())
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_basic() {
        let mut asm = Assembler::new();
        let code = "push 10\nhalt";
        let bin = asm.assemble(code).unwrap();
        assert_eq!(bin[0], Opcode::Push as u8);
        assert_eq!(bin[9], Opcode::Halt as u8);
    }

    #[test]
    fn test_assemble_labels() {
        let mut asm = Assembler::new();
        let code = "start:\npush 1\njmp start\nhalt";
        let bin = asm.assemble(code).unwrap();
        // 0: label start
        // 0: push 1 (9 bytes)
        // 9: jmp 0 (9 bytes)
        // 18: halt (1 byte)
        assert_eq!(bin[0], Opcode::Push as u8);
        assert_eq!(bin[9], Opcode::Jmp as u8);
        let target = u64::from_le_bytes(bin[10..18].try_into().unwrap());
        assert_eq!(target, 0);
    }

    #[test]
    fn test_assemble_db() {
        let mut asm = Assembler::new();
        let code = "db \"Hello\"\ndb 0";
        let bin = asm.assemble(code).unwrap();
        assert_eq!(&bin[0..5], b"Hello");
        assert_eq!(bin[5], 0);
        assert_eq!(bin[6], 0);
    }
}
