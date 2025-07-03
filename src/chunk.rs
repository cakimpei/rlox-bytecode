use crate::value::Value;

#[repr(u8)]
pub(crate) enum OpCode {
    OpConstant,
    OpReturn,
}

pub(crate) struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: Vec<Value>,
}

impl Chunk {
    pub(crate) fn new() -> Self {
        Chunk { code: Vec::new(), lines: Vec::new(), constants: Vec::new() }
    }

    pub(crate) fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub(crate) fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}