# Mini VM

A stack-based virtual machine with assembler, disassembler, and debugger.

## Building

```
cargo build --release
```

## Usage

### Assemble

```
mini-vm asm program.asm -o program.bin
```

### Run

```
mini-vm run program.bin
```

### Disassemble

```
mini-vm disasm program.bin
```

### Debug

```
mini-vm debug program.bin
```

## Debugger Commands

| Command | Description |
|---------|-------------|
| s, step | step one instruction |
| n, next [N] | step N instructions |
| r, run, c | run until breakpoint or halt |
| b, break ADDR | set breakpoint |
| d, delete [ADDR] | delete breakpoint |
| bl, breaklist | list breakpoints |
| m, memory [ADDR] [LEN] | show memory hexdump |
| w, watch ADDR | watch memory address |
| uw, unwatch [ADDR] | remove watch |
| x, examine ADDR | show 64-bit value at address |
| dis [ADDR] [N] | disassemble N instructions |
| st, stack | show stack contents |
| reg, state | show current state |
| reset | reset VM |
| q, quit | exit |

Addresses can be decimal or hex (0x prefix). Press enter to repeat last command.

## Instruction Set

### Stack Operations
| Opcode | Instruction | Description |
|--------|-------------|-------------|
| 0x00 | nop | no operation |
| 0x01 | halt | stop execution |
| 0x02 | push N | push value onto stack |
| 0x03 | pop | discard top of stack |
| 0x04 | dup | duplicate top of stack |
| 0x05 | swap | swap top two values |

### Memory
| Opcode | Instruction | Description |
|--------|-------------|-------------|
| 0x10 | load | pop addr, push mem[addr] |
| 0x11 | store | pop addr, pop val, mem[addr] = val |

### Arithmetic
| Opcode | Instruction | Description |
|--------|-------------|-------------|
| 0x20 | add | pop b, pop a, push a+b |
| 0x21 | sub | pop b, pop a, push a-b |
| 0x22 | mul | pop b, pop a, push a*b |
| 0x23 | div | pop b, pop a, push a/b |
| 0x24 | mod | pop b, pop a, push a%b |
| 0x25 | and | bitwise and |
| 0x26 | or | bitwise or |
| 0x27 | not | bitwise not |

### Comparison
| Opcode | Instruction | Description |
|--------|-------------|-------------|
| 0x30 | eq | push 1 if a==b else 0 |
| 0x31 | lt | push 1 if a<b else 0 |
| 0x32 | gt | push 1 if a>b else 0 |

### Control Flow
| Opcode | Instruction | Description |
|--------|-------------|-------------|
| 0x40 | jmp ADDR | jump to address |
| 0x41 | jz ADDR | jump if top is zero |
| 0x42 | jnz ADDR | jump if top is not zero |
| 0x50 | call ADDR | call subroutine |
| 0x51 | ret | return from subroutine |

### I/O (Traps)
| Instruction | Description |
|-------------|-------------|
| putc | print top of stack as char |
| getc | read char, push to stack |
| puts | print string at address |
| putu | print as unsigned integer |
| puti | print as signed integer |

## Assembly Syntax

```asm
; comments start with semicolon

label:          ; labels end with colon
    push 10     ; instructions with operands
    add         ; instructions without operands
    jmp label   ; jump to label
    halt
```

## Examples

See the `examples/` directory:

- `hello.asm` - hello world
- `math.asm` - arithmetic operations
- `loop.asm` - counting loop
- `functions.asm` - function calls
