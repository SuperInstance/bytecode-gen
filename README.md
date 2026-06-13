# Bytecode Gen

**A Rust library for bytecode generation and execution** — provides a stack-based virtual machine with a complete instruction set, disassembler, and REPL-ready execution model.

## Why It Matters

Every programming language needs a target — assembly, bytecode, or machine code. Stack-based bytecode is the simplest practical target: it's what the JVM, Python (CPython), WebAssembly, and the Ethereum Virtual Machine (EVM) all use.

Understanding bytecode generation teaches you:
- **How compilers work** — lowering ASTs to linear instruction sequences
- **How VMs work** — fetch-decode-execute loops, operand stacks, variable environments
- **How debuggers work** — disassembly, stepping, breakpoints

A stack-based VM uses a Last-In-First-Out (LIFO) operand stack for computation. `Push(10); Push(20); Add` is the bytecode equivalent of `10 + 20` — push operands, then the operation pops them and pushes the result.

## How It Works

**Instruction set** (`Op` enum): The VM supports 12 opcodes:
- **Stack ops**: `Push(i64)`, `Pop`
- **Arithmetic**: `Add`, `Sub`, `Mul`, `Div` (integer, with divide-by-zero detection)
- **I/O**: `Print` (prints top of stack)
- **Variables**: `Load(name)`, `Store(name)` using a HashMap environment
- **Control flow**: `Jump(addr)`, `JumpIfZero(addr)` (conditional branch on zero)
- **Terminator**: `Halt`

**Execution model** (`Vm`): The virtual machine maintains two data structures:
1. **Operand stack** (`Vec<i64>`) — temporary values for computation
2. **Variable environment** (`HashMap<String, i64>`) — named storage

The `run()` method implements a fetch-decode-execute loop: read the instruction at the instruction pointer (IP), execute it, increment the IP (unless it was a jump). Arithmetic pops two operands, computes, and pushes the result. Stack underflow returns an error.

**Chunk** (`BytecodeChunk`): A named collection of instructions with a constant pool. The `disassemble()` method produces a human-readable listing showing instruction offsets and opcodes — similar to `javap` or `python -m dis`.

## Quick Start

```rust
use bytecode_gen::{BytecodeChunk, Op, Vm};

// Compute (10 + 20) * 3 = 90
let mut chunk = BytecodeChunk::new("math");
chunk.emit(Op::Push(10));
chunk.emit(Op::Push(20));
chunk.emit(Op::Add);
chunk.emit(Op::Push(3));
chunk.emit(Op::Mul);
chunk.emit(Op::Halt);

let mut vm = Vm::new();
let result = vm.run(&chunk);
assert_eq!(result.unwrap(), Some(90));

// Using variables
let mut chunk2 = BytecodeChunk::new("vars");
chunk2.emit(Op::Push(42));
chunk2.emit(Op::Store("x".into()));
chunk2.emit(Op::Load("x".into()));
chunk2.emit(Op::Push(8));
chunk2.emit(Op::Add);
chunk2.emit(Op::Halt);

let mut vm2 = Vm::new();
assert_eq!(vm2.run(&chunk2).unwrap(), Some(50));

// View disassembly
println!("{}", chunk.disassemble());
```

## API

- **`Op`** — Enum: `Push`, `Pop`, `Add`, `Sub`, `Mul`, `Div`, `Print`, `Load`, `Store`, `Jump`, `JumpIfZero`, `Halt`
- **`BytecodeChunk`** — Named instruction block: `emit(op)`, `add_constant(val)`, `disassemble()`
- **`Vm`** — Stack machine: `new()`, `run(chunk)` → `Result<Option<i64>, String>`

## Architecture Notes

Provides the compilation target for SuperInstance language toolchain experiments. The stack-based design mirrors the JVM and WebAssembly execution models, making it a natural stepping stone toward a register-based VM or LLVM IR lowering. See the [architecture overview](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
