use std::collections::HashSet;
use std::io::{self, Write};

use crate::disasm::Disassembler;
use crate::vm::CPU;

pub struct Debugger {
    cpu: CPU,
    breakpoints: HashSet<usize>,
    disasm: Disassembler,
    watches: Vec<usize>,
    history: Vec<String>,
}

impl Debugger {
    pub fn new(program: Vec<u8>) -> Self {
        let mut cpu = CPU::new();
        cpu.load_program(&program);
        let disasm = Disassembler::new(program);
        
        Debugger {
            cpu,
            breakpoints: HashSet::new(),
            disasm,
            watches: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        self.print_header();
        self.print_state();

        loop {
            print!("\n> ");
            io::stdout().flush().ok();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }

            let input = input.trim();
            if input.is_empty() {
                if let Some(last) = self.history.last().cloned() {
                    self.execute_command(&last);
                }
                continue;
            }

            self.history.push(input.to_string());
            
            if !self.execute_command(input) {
                break;
            }
        }
    }

    fn execute_command(&mut self, input: &str) -> bool {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return true;
        }

        match parts[0] {
            "s" | "step" => self.cmd_step(),
            "n" | "next" => self.cmd_step_n(parts.get(1)),
            "r" | "run" | "c" | "continue" => self.cmd_run(),
            "b" | "break" => self.cmd_breakpoint(parts.get(1)),
            "d" | "delete" => self.cmd_delete_breakpoint(parts.get(1)),
            "bl" | "breaklist" => self.cmd_list_breakpoints(),
            "m" | "mem" | "memory" => self.cmd_memory(parts.get(1), parts.get(2)),
            "w" | "watch" => self.cmd_watch(parts.get(1)),
            "uw" | "unwatch" => self.cmd_unwatch(parts.get(1)),
            "x" | "examine" => self.cmd_examine(parts.get(1)),
            "dis" | "disasm" => self.cmd_disasm(parts.get(1), parts.get(2)),
            "stack" | "st" => self.cmd_stack(),
            "reg" | "state" => self.print_state(),
            "reset" => self.cmd_reset(),
            "h" | "help" | "?" => self.cmd_help(),
            "q" | "quit" | "exit" => return false,
            _ => println!("unknown command: {}. type 'help' for commands.", parts[0]),
        }

        true
    }

    fn cmd_step(&mut self) {
        if self.cpu.halted {
            println!("program halted");
            return;
        }

        self.cpu.step();
        self.print_state();
        self.print_watches();
    }

    fn cmd_step_n(&mut self, count: Option<&&str>) {
        let n: usize = count.and_then(|s| s.parse().ok()).unwrap_or(10);
        
        for _ in 0..n {
            if self.cpu.halted {
                println!("program halted");
                break;
            }
            self.cpu.step();
        }
        
        self.print_state();
        self.print_watches();
    }

    fn cmd_run(&mut self) {
        loop {
            if self.cpu.halted {
                println!("program halted");
                break;
            }

            if self.breakpoints.contains(&self.cpu.ip) {
                println!("breakpoint at 0x{:04x}", self.cpu.ip);
                self.print_state();
                break;
            }

            self.cpu.step();
        }
        
        self.print_watches();
    }

    fn cmd_breakpoint(&mut self, addr: Option<&&str>) {
        if let Some(addr_str) = addr {
            if let Some(addr) = self.parse_addr(addr_str) {
                self.breakpoints.insert(addr);
                println!("breakpoint set at 0x{:04x}", addr);
            } else {
                println!("invalid address: {}", addr_str);
            }
        } else {
            println!("usage: break <address>");
        }
    }

    fn cmd_delete_breakpoint(&mut self, addr: Option<&&str>) {
        if let Some(addr_str) = addr {
            if let Some(addr) = self.parse_addr(addr_str) {
                if self.breakpoints.remove(&addr) {
                    println!("breakpoint removed at 0x{:04x}", addr);
                } else {
                    println!("no breakpoint at 0x{:04x}", addr);
                }
            }
        } else {
            self.breakpoints.clear();
            println!("all breakpoints cleared");
        }
    }

    fn cmd_list_breakpoints(&self) {
        if self.breakpoints.is_empty() {
            println!("no breakpoints set");
        } else {
            println!("breakpoints:");
            let mut bps: Vec<_> = self.breakpoints.iter().collect();
            bps.sort();
            for bp in bps {
                print!("  0x{:04x}", bp);
                if let Some((instr, _)) = self.disasm.disassemble_at(*bp) {
                    print!("  {}", instr);
                }
                println!();
            }
        }
    }

    fn cmd_memory(&self, start: Option<&&str>, len: Option<&&str>) {
        let start_addr = start.and_then(|s| self.parse_addr(s)).unwrap_or(0);
        let length: usize = len.and_then(|s| s.parse().ok()).unwrap_or(64);

        println!("memory at 0x{:04x}:", start_addr);
        for row in 0..(length / 16).max(1) {
            let addr = start_addr + row * 16;
            print!("  {:04x}: ", addr);
            
            for col in 0..16 {
                if row * 16 + col < length {
                    print!("{:02x} ", self.cpu.memory.read_byte(addr + col));
                }
            }
            
            print!(" |");
            for col in 0..16 {
                if row * 16 + col < length {
                    let b = self.cpu.memory.read_byte(addr + col);
                    let c = if b >= 0x20 && b < 0x7f { b as char } else { '.' };
                    print!("{}", c);
                }
            }
            println!("|");
        }
    }

    fn cmd_watch(&mut self, addr: Option<&&str>) {
        if let Some(addr_str) = addr {
            if let Some(addr) = self.parse_addr(addr_str) {
                if !self.watches.contains(&addr) {
                    self.watches.push(addr);
                }
                println!("watching 0x{:04x}", addr);
            }
        } else {
            println!("usage: watch <address>");
        }
    }

    fn cmd_unwatch(&mut self, addr: Option<&&str>) {
        if let Some(addr_str) = addr {
            if let Some(addr) = self.parse_addr(addr_str) {
                self.watches.retain(|&a| a != addr);
                println!("unwatched 0x{:04x}", addr);
            }
        } else {
            self.watches.clear();
            println!("all watches cleared");
        }
    }

    fn cmd_examine(&self, addr: Option<&&str>) {
        if let Some(addr_str) = addr {
            if let Some(addr) = self.parse_addr(addr_str) {
                let val = self.cpu.memory.read_u64(addr);
                println!("0x{:04x}: {} (0x{:016x})", addr, val as i64, val);
            }
        } else {
            println!("usage: examine <address>");
        }
    }

    fn cmd_disasm(&self, start: Option<&&str>, count: Option<&&str>) {
        let start_addr = start.and_then(|s| self.parse_addr(s)).unwrap_or(self.cpu.ip);
        let n: usize = count.and_then(|s| s.parse().ok()).unwrap_or(10);

        let mut addr = start_addr;
        for _ in 0..n {
            if let Some((instr, size)) = self.disasm.disassemble_at(addr) {
                let marker = if addr == self.cpu.ip { ">" } else { " " };
                let bp = if self.breakpoints.contains(&addr) { "*" } else { " " };
                println!("{}{} {:04x}:  {}", marker, bp, addr, instr);
                addr += size;
            } else {
                break;
            }
        }
    }

    fn cmd_stack(&self) {
        if self.cpu.stack.is_empty() {
            println!("stack: (empty)");
        } else {
            println!("stack ({} items):", self.cpu.stack.len());
            for (i, val) in self.cpu.stack.iter().rev().enumerate() {
                let marker = if i == 0 { " <- top" } else { "" };
                println!("  [{}] {} (0x{:x}){}", 
                    self.cpu.stack.len() - 1 - i, val, *val as u64, marker);
            }
        }
        
        if !self.cpu.call_stack.is_empty() {
            println!("call stack:");
            for (i, addr) in self.cpu.call_stack.iter().rev().enumerate() {
                println!("  [{}] 0x{:04x}", self.cpu.call_stack.len() - 1 - i, addr);
            }
        }
    }

    fn cmd_reset(&mut self) {
        let program: Vec<u8> = (0..self.cpu.memory.as_slice().len())
            .map(|i| self.cpu.memory.read_byte(i))
            .take_while(|&b| b != 0 || true)
            .collect();
        
        self.cpu = CPU::new();
        self.cpu.load_program(&program);
        println!("reset to initial state");
        self.print_state();
    }

    fn cmd_help(&self) {
        println!("debugger commands:");
        println!("  s, step          step one instruction");
        println!("  n, next [N]      step N instructions (default 10)");
        println!("  r, run, c        run until breakpoint or halt");
        println!("  b, break ADDR    set breakpoint at address");
        println!("  d, delete [ADDR] delete breakpoint (all if no addr)");
        println!("  bl, breaklist    list all breakpoints");
        println!("  m, memory [ADDR] [LEN]  show memory");
        println!("  w, watch ADDR    watch memory address");
        println!("  uw, unwatch ADDR unwatch address");
        println!("  x, examine ADDR  examine 64-bit value at address");
        println!("  dis [ADDR] [N]   disassemble N instructions");
        println!("  st, stack        show stack");
        println!("  reg, state       show current state");
        println!("  reset            reset VM to initial state");
        println!("  q, quit          exit debugger");
        println!();
        println!("addresses can be decimal or hex (0x prefix)");
        println!("press enter to repeat last command");
    }

    fn print_header(&self) {
        println!("mini-vm debugger");
        println!("type 'help' for commands\n");
    }

    fn print_state(&self) {
        let status = if self.cpu.halted { "HALTED" } else { "running" };
        println!("ip: 0x{:04x}  stack: {}  status: {}", 
            self.cpu.ip, self.cpu.stack.len(), status);

        if !self.cpu.halted {
            if let Some((instr, _)) = self.disasm.disassemble_at(self.cpu.ip) {
                println!("next: {}", instr);
            }
        }
    }

    fn print_watches(&self) {
        if self.watches.is_empty() {
            return;
        }
        
        println!("watches:");
        for &addr in &self.watches {
            let val = self.cpu.memory.read_u64(addr);
            println!("  0x{:04x} = {} (0x{:x})", addr, val as i64, val);
        }
    }

    fn parse_addr(&self, s: &str) -> Option<usize> {
        if s.starts_with("0x") || s.starts_with("0X") {
            usize::from_str_radix(&s[2..], 16).ok()
        } else {
            s.parse().ok()
        }
    }
}
