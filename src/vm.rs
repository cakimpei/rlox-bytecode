use std::rc::Rc;

use crate::{
    chunk::{Chunk, OpCode},
    value::Val,
};

pub(crate) struct VM {
    chunk: Option<Rc<Chunk>>,
    ip: usize, // index, not pointer
    stack: Vec<Val>,
}

impl VM {
    pub(crate) fn new() -> Self {
        VM {
            chunk: None,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub(crate) fn interpret(&mut self, chunk: Rc<Chunk>) -> Result<(), InterpretErr> {
        self.chunk = Some(chunk);
        self.ip = 0;

        self.run()
    }

    fn run(&mut self) -> Result<(), InterpretErr> {
        loop {
            #[cfg(debug_assertions)]
            if let Some(chunk_rc) = &mut self.chunk {
                let stack_display = self
                    .stack
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("[{}]", stack_display);
                chunk_rc
                    .disassemble_instruction(self.ip)
                    .map_err(|_e| InterpretErr::InterpretCompileErr)?;
            }
            let byte = self.read_byte().ok_or(InterpretErr::InterpretCompileErr)?;
            let instruction =
                OpCode::try_from(byte).map_err(|_e| InterpretErr::InterpretCompileErr)?;
            match instruction {
                OpCode::OpConstant => {
                    let constant = self.read_constant()?.clone();
                    self.stack.push(constant);
                }
                OpCode::OpAdd => {
                    self.binary_op(|a, b| a + b)?;
                }
                OpCode::OpSubtract => {
                    self.binary_op(|a, b| a - b)?;
                }
                OpCode::OpMultiply => {
                    self.binary_op(|a, b| a * b)?;
                }
                OpCode::OpDivide => {
                    self.binary_op(|a, b| a / b)?;
                }
                OpCode::OpNegate => {
                    let mut val = self.stack.pop().ok_or(InterpretErr::InterpretCompileErr)?;
                    *val.val_mut() = -val.val();
                    self.stack.push(val);
                }
                OpCode::OpReturn => {
                    let return_val = self.stack.pop();
                    if let Some(val) = return_val {
                        println!("{}", val);
                    }
                    return Ok(());
                }
            }
        }
    }

    fn read_byte(&mut self) -> Option<&u8> {
        if let Some(chunk) = &self.chunk {
            if let Some(byte) = chunk.code().get(self.ip) {
                self.ip += 1;
                return Some(byte);
            }
        }
        None
    }

    fn read_constant(&mut self) -> Result<&Val, InterpretErr> {
        let index = *self.read_byte().ok_or(InterpretErr::InterpretCompileErr)? as usize;
        if let Some(chunk) = &mut self.chunk {
            if let Some(val) = chunk.constants().get(index) {
                return Ok(val);
            }
        }
        Err(InterpretErr::InterpretCompileErr)
    }

    fn binary_op<T>(&mut self, op: T) -> Result<(), InterpretErr>
    where
        T: Fn(f64, f64) -> f64,
    {
        let b = self.stack.pop().ok_or(InterpretErr::InterpretCompileErr)?;
        let a = self.stack.pop().ok_or(InterpretErr::InterpretCompileErr)?;
        let res = Val::new(op(*a.val(), *b.val()));
        self.stack.push(res);
        Ok(())
    }
}

pub(crate) enum InterpretErr {
    InterpretCompileErr,
    InterpretRuntimeErr,
}
