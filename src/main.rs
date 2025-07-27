mod chunk;
mod debug;
mod value;
mod vm;

use std::rc::Rc;

use crate::{
    chunk::{Chunk, OpCode},
    value::Val,
    vm::VM,
};

fn main() {
    let mut vm = VM::new();
    let mut chunk = Chunk::new();

    let mut constant = chunk.add_constant(Val::new(1.2));
    chunk.write_chunk(OpCode::OpConstant as u8, 1);
    chunk.write_chunk(constant as u8, 1); // usize to u8 conversion!

    constant = chunk.add_constant(Val::new(3.4));
    chunk.write_chunk(OpCode::OpConstant as u8, 1);
    chunk.write_chunk(constant as u8, 1);

    chunk.write_chunk(OpCode::OpAdd as u8, 1);

    constant = chunk.add_constant(Val::new(5.6));
    chunk.write_chunk(OpCode::OpConstant as u8, 1);
    chunk.write_chunk(constant as u8, 1);

    chunk.write_chunk(OpCode::OpDivide as u8, 1);
    chunk.write_chunk(OpCode::OpNegate as u8, 1);

    chunk.write_chunk(OpCode::OpReturn as u8, 1);

    let _ = vm.interpret(Rc::from(chunk));
}
