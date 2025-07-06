mod chunk;
mod debug;
mod value;

use crate::{
    chunk::{Chunk, OpCode},
    value::Val,
};

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(Val::new(1.5));
    chunk.write_chunk(OpCode::OpConstant as u8, 1);
    chunk.write_chunk(constant as u8, 1); // usize to u8 conversion!

    chunk.write_chunk(OpCode::OpReturn as u8, 1);

    chunk.disassemble("test chunk");
}
