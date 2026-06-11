/// A bytecode generation library with a virtual machine.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Push(i64),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Load(String),
    Store(String),
    Jump(usize),
    JumpIfZero(usize),
    Halt,
}

#[derive(Debug, Clone)]
pub struct BytecodeChunk {
    pub ops: Vec<Op>,
    pub constants: Vec<i64>,
    pub name: String,
}

impl BytecodeChunk {
    pub fn new(name: &str) -> Self {
        Self {
            ops: Vec::new(),
            constants: Vec::new(),
            name: name.to_string(),
        }
    }

    pub fn emit(&mut self, op: Op) {
        self.ops.push(op);
    }

    pub fn add_constant(&mut self, val: i64) -> usize {
        let idx = self.constants.len();
        self.constants.push(val);
        idx
    }

    pub fn disassemble(&self) -> String {
        let mut out = format!("=== {} ===\n", self.name);
        for (i, op) in self.ops.iter().enumerate() {
            out.push_str(&format!("{:04} {:?}\n", i, op));
        }
        out
    }
}

#[derive(Debug)]
pub struct Vm {
    stack: Vec<i64>,
    vars: HashMap<String, i64>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            vars: HashMap::new(),
        }
    }

    pub fn run(&mut self, chunk: &BytecodeChunk) -> Result<Option<i64>, String> {
        let mut ip = 0;
        while ip < chunk.ops.len() {
            match &chunk.ops[ip] {
                Op::Push(v) => self.stack.push(*v),
                Op::Pop => { self.stack.pop(); }
                Op::Add => {
                    let b = self.stack.pop().ok_or("stack underflow")?;
                    let a = self.stack.pop().ok_or("stack underflow")?;
                    self.stack.push(a + b);
                }
                Op::Sub => {
                    let b = self.stack.pop().ok_or("stack underflow")?;
                    let a = self.stack.pop().ok_or("stack underflow")?;
                    self.stack.push(a - b);
                }
                Op::Mul => {
                    let b = self.stack.pop().ok_or("stack underflow")?;
                    let a = self.stack.pop().ok_or("stack underflow")?;
                    self.stack.push(a * b);
                }
                Op::Div => {
                    let b = self.stack.pop().ok_or("stack underflow")?;
                    let a = self.stack.pop().ok_or("stack underflow")?;
                    if b == 0 { return Err("division by zero".into()); }
                    self.stack.push(a / b);
                }
                Op::Print => {
                    if let Some(v) = self.stack.last() {
                        print!("{}", v);
                    }
                }
                Op::Load(name) => {
                    let val = self.vars.get(name).copied().unwrap_or(0);
                    self.stack.push(val);
                }
                Op::Store(name) => {
                    let val = self.stack.pop().ok_or("stack underflow")?;
                    self.vars.insert(name.clone(), val);
                }
                Op::Jump(addr) => { ip = *addr; continue; }
                Op::JumpIfZero(addr) => {
                    let v = self.stack.pop().ok_or("stack underflow")?;
                    if v == 0 { ip = *addr; continue; }
                }
                Op::Halt => break,
            }
            ip += 1;
        }
        Ok(self.stack.last().copied())
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        let mut chunk = BytecodeChunk::new("test");
        chunk.emit(Op::Push(10));
        chunk.emit(Op::Push(20));
        chunk.emit(Op::Add);
        chunk.emit(Op::Halt);

        let mut vm = Vm::new();
        assert_eq!(vm.run(&chunk), Ok(Some(30)));
    }

    #[test]
    fn test_variables() {
        let mut chunk = BytecodeChunk::new("vars");
        chunk.emit(Op::Push(42));
        chunk.emit(Op::Store("x".into()));
        chunk.emit(Op::Load("x".into()));
        chunk.emit(Op::Push(8));
        chunk.emit(Op::Add);
        chunk.emit(Op::Halt);

        let mut vm = Vm::new();
        assert_eq!(vm.run(&chunk), Ok(Some(50)));
    }
}
