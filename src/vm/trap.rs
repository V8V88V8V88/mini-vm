use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Trap {
    Putc = 0x01,
    Getc = 0x02,
    Puts = 0x03,
    Putu = 0x04,
    Puti = 0x05,
}

impl Trap {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(Trap::Putc),
            0x02 => Some(Trap::Getc),
            0x03 => Some(Trap::Puts),
            0x04 => Some(Trap::Putu),
            0x05 => Some(Trap::Puti),
            _ => None,
        }
    }
}

pub fn execute_trap(trap: Trap, stack: &mut Vec<i64>, memory: &[u8]) {
    match trap {
        Trap::Putc => {
            if let Some(val) = stack.pop() {
                print!("{}", val as u8 as char);
                io::stdout().flush().ok();
            }
        }
        Trap::Getc => {
            let mut buf = [0u8; 1];
            if io::stdin().read_exact(&mut buf).is_ok() {
                stack.push(buf[0] as i64);
            } else {
                stack.push(-1);
            }
        }
        Trap::Puts => {
            if let Some(addr) = stack.pop() {
                let mut i = addr as usize;
                while i < memory.len() && memory[i] != 0 {
                    print!("{}", memory[i] as char);
                    i += 1;
                }
                io::stdout().flush().ok();
            }
        }
        Trap::Putu => {
            if let Some(val) = stack.pop() {
                print!("{}", val as u64);
                io::stdout().flush().ok();
            }
        }
        Trap::Puti => {
            if let Some(val) = stack.pop() {
                print!("{}", val);
                io::stdout().flush().ok();
            }
        }
    }
}
