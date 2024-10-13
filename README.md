# Mini Virtual Machine

This project implements a mini virtual machine (VM) written in Rust. The VM supports basic arithmetic operations, branching, stack manipulation, and function calls. It also includes a colorful Terminal User Interface (TUI) to display the VM's state in real-time.

## Features

- Basic instruction set including load, store, arithmetic operations, and branching
- 8 general-purpose registers
- 1024 bytes of simulated memory
- Stack support for function calls and local storage
- Colorful TUI displaying registers, memory, stack, and current instruction
- Example program demonstrating VM capabilities

## Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager

## Building and Running

1. Clone the repository:
   ```
   git clone https://github.com/v8v88v8v88/mini-vm.git
   cd mini-vm
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the VM with the example program:
   ```
   cargo run --release
   ```

## Instruction Set

The VM supports the following instructions:

- `Load(reg, value)`: Load a value into a register
- `Store(reg, address)`: Store a value from a register into memory
- `Add(dest, src1, src2)`: Add values from two registers and store the result
- `Sub(dest, src1, src2)`: Subtract values from two registers and store the result
- `Mul(dest, src1, src2)`: Multiply values from two registers and store the result
- `Div(dest, src1, src2)`: Divide values from two registers and store the result
- `Jump(address)`: Jump to a specific address
- `JumpIfZero(address)`: Jump to an address if the flag is zero
- `JumpIfNegative(address)`: Jump to an address if the flag is negative
- `Push(reg)`: Push a value from a register onto the stack
- `Pop(reg)`: Pop a value from the stack into a register
- `Call(address)`: Call a function at the specified address
- `Return`: Return from a function call
- `Halt`: Stop the VM execution

## Writing Programs for the VM

To write a program for the VM, create an array of 32-bit unsigned integers representing the instructions. Each instruction is encoded as follows:

- Bits 31-28: Opcode
- Bits 27-24: Register 1
- Bits 23-20: Register 2
- Bits 19-16: Register 3
- Bits 15-0: Immediate value or address

For example, to load the value 5 into register 0, you would use the instruction:
```rust
0x00000005 // Load R0, 5
```

Refer to the `Instruction` enum in `src/vm/instruction.rs` for the complete list of opcodes and their encodings.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
