use crate::value::Val;

#[repr(u8)]
pub(crate) enum OpCode {
    OpConstant,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn,
}

impl TryFrom<&u8> for OpCode {
    type Error = &'static str;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::OpConstant),
            1 => Ok(OpCode::OpAdd),
            2 => Ok(OpCode::OpSubtract),
            3 => Ok(OpCode::OpMultiply),
            4 => Ok(OpCode::OpDivide),
            5 => Ok(OpCode::OpNegate),
            6 => Ok(OpCode::OpReturn),
            _ => Err(""),
        }
    }
}

pub(crate) struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: Vec<Val>,
}

impl Chunk {
    pub(crate) fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub(crate) fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub(crate) fn lines(&self) -> &Vec<usize> {
        &self.lines
    }

    pub(crate) fn constants(&self) -> &Vec<Val> {
        &self.constants
    }

    pub(crate) fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub(crate) fn add_constant(&mut self, value: Val) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
