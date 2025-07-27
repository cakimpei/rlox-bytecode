use crate::chunk::{Chunk, OpCode};

#[derive(Debug)]
pub(crate) enum DisassembleInstrErr {
    IndexOutOfRange,
    CorruptInstruction,
    LineIndexOutOfRange,
}

impl Chunk {
    pub(crate) fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset: usize = 0;
        while offset < self.code().len() {
            offset = self
                .disassemble_instruction(offset)
                .expect(&format!("Cannot dissassemble instruction at {}", offset));
        }
    }

    pub(crate) fn disassemble_instruction(
        &self,
        offset: usize,
    ) -> Result<usize, DisassembleInstrErr> {
        print!("{:0>4} ", offset);
        let curr_line = self
            .lines()
            .get(offset)
            .ok_or_else(|| DisassembleInstrErr::LineIndexOutOfRange)?;

        if offset > 0
            && curr_line
                == self
                    .lines()
                    .get(offset - 1)
                    .ok_or_else(|| DisassembleInstrErr::LineIndexOutOfRange)?
        {
            print!("   | ");
        } else {
            print!("{:>4} ", curr_line);
        }

        if let Some(byte) = self.code().get(offset) {
            if let Ok(instruction) = OpCode::try_from(byte) {
                match instruction {
                    OpCode::OpConstant => self.constant_instruction("OP_CONSTANT", offset),
                    OpCode::OpAdd => self.simple_instruction("OP_ADD", offset),
                    OpCode::OpSubtract => self.simple_instruction("OP_SUBTRACT", offset),
                    OpCode::OpMultiply => self.simple_instruction("OP_MULTIPLY", offset),
                    OpCode::OpDivide => self.simple_instruction("OP_DIVIDE", offset),
                    OpCode::OpNegate => self.simple_instruction("OP_NEGATE", offset),
                    OpCode::OpReturn => self.simple_instruction("OP_RETURN", offset),
                }
            } else {
                println!("Unknown opcode {}", byte);
                Ok(offset + 1)
            }
        } else {
            Err(DisassembleInstrErr::IndexOutOfRange)
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> Result<usize, DisassembleInstrErr> {
        println!("{}", name);
        Ok(offset + 1)
    }

    fn constant_instruction(
        &self,
        name: &str,
        offset: usize,
    ) -> Result<usize, DisassembleInstrErr> {
        if let Some(constant_byte) = self.code().get(offset + 1) {
            if let Some(constant) = self.constants().get(*constant_byte as usize) {
                println!("{:<16} {:>4} '{}'", name, constant_byte, constant);
                return Ok(offset + 2);
            }
        }
        Err(DisassembleInstrErr::CorruptInstruction)
    }
}
