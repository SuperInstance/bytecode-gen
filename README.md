# bytecode-gen

A Rust library for **bytecode generation and virtual machine execution**, providing a stack-based instruction set, a disassembler, and a register-variable VM with jump control flow. Designed for compiler backends, DSL implementation, and language prototyping.

## Why It Matters

Bytecode VMs are the execution layer behind Python (CPython), Lua, the JVM, WebAssembly, and V8's Ignition interpreter. Understanding bytecode generation is essential for:

- **Language implementation** — compiling source to a portable IR
- **DSL engines** — query languages, rule engines, scripting
- **Sandboxing** — bytecode VMs provide controlled execution boundaries
- **JIT foundations** — bytecode is the starting point for trace/cranelift JIT compilation

This crate implements a clean, minimal bytecode IR that demonstrates the full compilation pipeline: source-level semantics → opcode emission → stack-machine execution.

## How It Works

### Instruction Set

The opcode enum maps directly to a stack machine:

| Opcode | Stack Effect | Semantics |
|--------|-------------|-----------|
| `Push(v)` | → v | Push constant |
| `Pop` | v → | Discard top |
| `Add` | a, b → a+b | Arithmetic |
| `Sub` | a, b → a−b | Arithmetic |
| `Mul` | a, b → a×b | Arithmetic |
| `Div` | a, b → a/b | Arithmetic (zero-check) |
| `Load(name)` | → vars[name] | Variable read |
| `Store(name)` | v → | Variable write |
| `Jump(addr)` | — | Unconditional branch |
| `JumpIfZero(addr)` | v → | Conditional branch |
| `Print` | v → | Output top |
| `Halt` | — | Terminate |

### VM Execution

The `Vm::run()` method is a **dispatch loop** — the canonical bytecode interpreter pattern:

```
ip = 0
while ip < len(ops):
    match ops[ip]:
        Push(v) → stack.push(v)
        Add     → b=pop(); a=pop(); push(a+b)
        ...
        Jump(a) → ip = a; continue
    ip += 1
```

### Complexity Analysis

| Operation | Time | Notes |
|-----------|------|-------|
| `emit(op)` | O(1) amortized | Vec push |
| `add_constant(v)` | O(1) amortized | Vec push |
| `Vm::run(chunk)` | O(n) | n = instruction count |
| `disassemble()` | O(n) | Linear pretty-print |

Each opcode dispatch is O(1), so total execution is linear in the bytecode length. Stack operations are O(1) amortized (Vec backing).

### Stack Safety

The VM returns `Result<Option<i64>, String>` — stack underflow and division-by-zero are caught and returned as errors rather than panicking. This makes the VM safe for untrusted bytecode.

## Quick Start

```rust
use bytecode_gen::{BytecodeChunk, Op, Vm};

let mut chunk = BytecodeChunk::new("expr");
chunk.emit(Op::Push(10));
chunk.emit(Op::Push(20));
chunk.emit(Op::Add);
chunk.emit(Op::Halt);

let mut vm = Vm::new();
assert_eq!(vm.run(&chunk), Ok(Some(30)));

// Variables and control flow
let mut chunk2 = BytecodeChunk::new("loop");
chunk2.emit(Op::Push(42));
chunk2.emit(Op::Store("x".into()));
chunk2.emit(Op::Load("x".into()));
chunk2.emit(Op::Push(8));
chunk2.emit(Op::Add);
chunk2.emit(Op::Halt);

assert_eq!(Vm::new().run(&chunk2), Ok(Some(50)));
```

## API

| Type | Method | Description |
|------|--------|-------------|
| `BytecodeChunk` | `new(name)` | Create named chunk |
| `BytecodeChunk` | `emit(Op)` | Append instruction |
| `BytecodeChunk` | `add_constant(i64) → usize` | Pool a constant |
| `BytecodeChunk` | `disassemble() → String` | Pretty-print |
| `Vm` | `new()` | Fresh VM state |
| `Vm` | `run(&chunk) → Result<Option<i64>, String>` | Execute |

## Architecture Notes

The **γ + η = C** link: the bytecode emitter (γ) generates the instruction stream, while the VM dispatch loop (η) interprets it. Together they conserve the semantic invariant C — for any well-formed chunk, the VM produces a deterministic result determined solely by the opcode sequence and constant pool. The stack underflow/division-by-zero checks form the safety boundary ensuring C is never violated by malformed input.

## References

- Ertl, M. A., & Gregg, D. (2003). *The Structure and Performance of Efficient Interpreters.* JILP, 5.
- Diehl, S., Hartel, P., & Sestoft, P. (2000). *Abstract Machines for Programming Language Implementation.* ERCIM News.
- Ierusalimschy, R., de Figueiredo, L. H., & Celes, W. (2005). *The Implementation of Lua 5.0.* JUCS.
- Haahr, M. (2021). *Crafting Interpreters.* Genever Benning. Chapters 15–18.
- bytecodealliance/wasmtime: *Cranelift IR.* <https://cranelift.dev/>

## License

MIT
