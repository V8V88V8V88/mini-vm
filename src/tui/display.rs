use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

use crate::vm::{Flag, CPU};

pub struct TUI;

impl TUI {
    pub fn display(cpu: &CPU) -> std::io::Result<()> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        Self::display_registers(cpu, &mut stdout)?;
        Self::display_memory(cpu, &mut stdout)?;
        Self::display_stack(cpu, &mut stdout)?;
        Self::display_current_instruction(cpu, &mut stdout)?;

        stdout.flush()?;
        Ok(())
    }

    fn display_registers(cpu: &CPU, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        execute!(
            stdout,
            SetForegroundColor(Color::Blue),
            Print("Registers:\n"),
            ResetColor
        )?;

        for (i, reg) in cpu.get_registers().iter().enumerate() {
            execute!(stdout, Print(format!("R{}: {:#010x}\n", i, reg)))?;
        }

        execute!(
            stdout,
            Print(format!("PC: {:#010x}\n", cpu.get_pc())),
            Print(format!("Flag: {:?}\n", cpu.get_flag())),
            Print("\n")
        )?;

        Ok(())
    }

    fn display_memory(cpu: &CPU, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        execute!(
            stdout,
            SetForegroundColor(Color::Green),
            Print("Memory:\n"),
            ResetColor
        )?;

        for i in 0..16 {
            let address = i * 4;
            execute!(
                stdout,
                Print(format!(
                    "{:#06x}: {:#010x}\n",
                    address,
                    cpu.get_memory().read(address)
                ))
            )?;
        }

        execute!(stdout, Print("\n"))?;
        Ok(())
    }

    fn display_stack(cpu: &CPU, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print("Stack:\n"),
            ResetColor
        )?;

        for (i, value) in cpu.get_stack().iter().rev().take(8).enumerate() {
            execute!(
                stdout,
                Print(format!(
                    "{}: {:#010x}\n",
                    cpu.get_stack().size() - i - 1,
                    value
                ))
            )?;
        }

        execute!(stdout, Print("\n"))?;
        Ok(())
    }

    fn display_current_instruction(cpu: &CPU, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        execute!(
            stdout,
            SetForegroundColor(Color::Red),
            Print("Current Instruction:\n"),
            ResetColor
        )?;

        let instruction = cpu.get_memory().read(cpu.get_pc());
        execute!(stdout, Print(format!("{:#010x}\n", instruction)))?;

        Ok(())
    }
}
