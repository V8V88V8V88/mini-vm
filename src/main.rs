mod asm;
mod debugger;
mod disasm;
mod vm;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use asm::Assembler;
use debugger::Debugger;
use disasm::disassemble_program;
use vm::CPU;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let result = match args[1].as_str() {
        "run" => cmd_run(&args[2..]),
        "asm" => cmd_asm(&args[2..]),
        "disasm" => cmd_disasm(&args[2..]),
        "debug" => cmd_debug(&args[2..]),
        "help" | "--help" | "-h" => {
            print_usage();
            Ok(())
        }
        _ => {
            eprintln!("unknown command: {}", args[1]);
            print_usage();
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}

fn print_usage() {
    eprintln!("mini-vm - a stack-based virtual machine\n");
    eprintln!("usage:");
    eprintln!("  mini-vm run <file.bin>       run a binary program");
    eprintln!("  mini-vm asm <file.asm> [-o out.bin]  assemble program");
    eprintln!("  mini-vm disasm <file.bin>    disassemble binary");
    eprintln!("  mini-vm debug <file.bin>     interactive debugger");
    eprintln!("  mini-vm help                 show this message");
}

fn cmd_run(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("usage: mini-vm run <file.bin>".into());
    }

    let program = fs::read(&args[0])
        .map_err(|e| format!("failed to read '{}': {}", args[0], e))?;

    let mut cpu = CPU::new();
    cpu.load_program(&program);
    cpu.run();

    Ok(())
}

fn cmd_asm(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("usage: mini-vm asm <file.asm> [-o output.bin]".into());
    }

    let source = fs::read_to_string(&args[0])
        .map_err(|e| format!("failed to read '{}': {}", args[0], e))?;

    let mut assembler = Assembler::new();
    let bytecode = assembler.assemble(&source)?;

    let output_file = if args.len() >= 3 && args[1] == "-o" {
        Some(args[2].as_str())
    } else {
        None
    };

    if let Some(path) = output_file {
        fs::write(path, &bytecode)
            .map_err(|e| format!("failed to write '{}': {}", path, e))?;
        eprintln!("assembled {} bytes -> {}", bytecode.len(), path);
    } else {
        io::stdout().write_all(&bytecode)
            .map_err(|e| format!("failed to write: {}", e))?;
    }

    Ok(())
}

fn cmd_disasm(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("usage: mini-vm disasm <file.bin>".into());
    }

    let program = fs::read(&args[0])
        .map_err(|e| format!("failed to read '{}': {}", args[0], e))?;

    let output = disassemble_program(&program);
    print!("{}", output);

    Ok(())
}

fn cmd_debug(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("usage: mini-vm debug <file.bin>".into());
    }

    let program = fs::read(&args[0])
        .map_err(|e| format!("failed to read '{}': {}", args[0], e))?;

    let mut debugger = Debugger::new(program);
    debugger.run();

    Ok(())
}
