mod tui;
mod vm;

use std::{thread, time::Duration};
use tui::TUI;
use vm::{Instruction, CPU};

fn main() -> std::io::Result<()> {
    let mut cpu = CPU::new(1024);

    let program = [
        0x00000001, // Load R0, 1
        0x01000002, // Load R1, 2
        0x20000001, // Add R0, R0, R1
        0x30100000, // Sub R1, R1, R0
        0x70000009, // JumpIfZero 9
        0x40000001, // Mul R0, R0, R1
        0x50100000, // Div R1, R1, R0
        0x60000004, // Jump 4
        0x90000000, // Push R0
        0xA0000001, // Pop R1
        0xB000000E, // Call 14
        0xF0000000, // Halt
        0x00000000, // Nop (padding)
        0x00000000, // Nop (padding)
        0x00000005, // Load R0, 5
        0xC0000000, // Return
    ];

    cpu.get_memory().load_program(&program);

    loop {
        TUI::display(&cpu)?;

        if cpu.get_pc() >= program.len() {
            break;
        }

        let instruction = cpu.fetch();
        let decoded = cpu.decode(instruction);
        cpu.execute(decoded);

        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}

pub fn get_memory(&self) -> &Memory {
    &mut self.memory
}
