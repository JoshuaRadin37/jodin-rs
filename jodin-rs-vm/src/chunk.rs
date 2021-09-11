//! A chunk is a set of data being used by the virtual machine that contains instructions and
//! constants.

use std::ops::{Index};
use std::slice::SliceIndex;
use crate::bytecode::{ByteCode, BinaryOpOperand};

use num_traits::FromPrimitive;

/// A chunk of code
pub struct Chunk(pub Vec<u8>);



impl<I : SliceIndex<[u8], Output=[u8]>> Index<I> for Chunk
where
{
    type Output = [u8];

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}


impl Chunk {
    /// Create a new chunk
    pub fn new() -> Self {
        Chunk(vec![])
    }

    /// Gets a byte at an index
    pub fn byte_at(&self, index: usize) -> u8 {
        self.0[index]
    }

    /// Disassembles a byte code operation
    pub fn disassemble_single_bytecode(&self, index: usize) -> String {
        let (op, bytes) = self.bytecode_and_operands(index);
        let op_string = format!("{:?}", op).replace("ByteCode::", "");
        let instruction = format!("0x{:016x}:", index);


        let bytes = bytes.unwrap_or(&[]);
        let instruction = format!("{} {:02x} {:24}", instruction, op as u8, bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(" "));
        if let Some(interpret) = op.interpret_operand_as_string(bytes) {
            format!("{} | {} {}", instruction, op_string, interpret)
        } else {
            format!("{} | {} {}", instruction, op_string, "")
        }
    }

    /// Disassembles the chunk into human-readable form
    pub fn disassemble(&self) -> String {
        let mut ptr = Some(0);
        let mut string = String::new();
        while let Some(index) = ptr {
            let line = self.disassemble_single_bytecode(index);
            string = format!("{}{}\n", string, line);
            ptr = self.next_bytecode(index)
        }
        string
    }

    /// Gets a slice of bytes, size determined by the given opcode
    pub fn get_bytes_for_bytecode(&self, opcode: &ByteCode, index: usize) -> Option<&[u8]> {
        if opcode.operand_bytes() > 0 {
            Some(&self[(index + 1)..(index + 1 + opcode.operand_bytes())])
        } else {
            None
        }
    }

    /// Gets a byte code and relevant operand bytes from an index
    pub fn bytecode_and_operands(&self, index: usize) -> (ByteCode, Option<&[u8]>) {
        let op: ByteCode = ByteCode::from_u8(self.byte_at(index)).unwrap();
        let bytes = self.get_bytes_for_bytecode(&op, index);
        (op, bytes)
    }

    /// Tries to get the next opcode after the one at this position
    pub fn next_bytecode(&self, index: usize) -> Option<usize> {
        let (_, bytes) = self.bytecode_and_operands(index);
        let count = 1 + bytes.map(|t| t.len()).unwrap_or(0);
        if index + count >= self.0.len() {
            None
        } else {
            Some(index + count)
        }
    }

    fn append_bytecode(&mut self, opcode: ByteCode) {
        self.0.push(opcode as u8)
    }

    fn append_bytes(&mut self, bytes: &[u8]) {
        self.0.extend(bytes)
    }

    /// Add a ret opcode
    pub fn ret(&mut self) {
        self.append_bytecode(ByteCode::Return);
    }

    /// Pushes a byte to the stack
    pub fn constant_1byte(&mut self, byte: u8) {
        self.append_bytecode(ByteCode::Const1);
        self.append_bytes(&[byte])
    }

    /// Pushes a short to the stack
    pub fn constant_2byte(&mut self, short: u16) {
        self.append_bytecode(ByteCode::Const2);
        let bytes: [u8; 2] = short.to_be_bytes();
        self.append_bytes(&bytes)
    }

    /// Pushes an integer to the stack
    pub fn constant_4byte(&mut self, int: u32) {
        self.append_bytecode(ByteCode::Const4);
        let bytes: [u8; 4] = int.to_be_bytes();
        self.append_bytes(&bytes)
    }

    /// Pushes a long integer to the stack
    pub fn constant_8byte(&mut self, long: u64) {
        self.append_bytecode(ByteCode::Const8);
        let bytes: [u8; 8] = long.to_be_bytes();
        self.append_bytes(&bytes)
    }

    /// Pushes an integer to the stack
    pub fn constant_float4(&mut self, float: f32) {
        self.append_bytecode(ByteCode::Float4);
        let bytes: [u8; 4] = float.to_be_bytes();
        self.append_bytes(&bytes)
    }

    /// Pushes a long integer to the stack
    pub fn constant_float8(&mut self, double: f64) {
        self.append_bytecode(ByteCode::Float8);
        let bytes: [u8; 8] = double.to_be_bytes();
        self.append_bytes(&bytes)
    }

    /// Pop n amount of bytes from the stack
    pub fn pop_n(&mut self, bytes_to_pop: usize) {
        self.append_bytecode(ByteCode::PopN);
        let bytes = bytes_to_pop.to_be_bytes();
        self.append_bytes(&bytes);
    }

    /// Add a binary operator and operand data
    pub fn binary_operator(&mut self, operator: ByteCode, operand: BinaryOpOperand) {
        self.append_bytecode(operator);
        self.append_bytes(&[operand.0])
    }

    pub fn halt(&mut self) {
        self.append_bytecode(ByteCode::Halt);
    }
}

pub struct ByteCodeVector<'a>(&'a [u8]);

impl<'a> ByteCodeVector<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        ByteCodeVector(code)
    }

    /// Gets a byte code and relevant operand bytes from an index
    pub fn bytecode_and_operands(&self, index: usize) -> (ByteCode, Option<&[u8]>) {
        let op: ByteCode = ByteCode::from_u8(*self.0.get(index).unwrap()).unwrap();
        let bytes = self.get_bytes_for_bytecode(&op, index);
        (op, bytes)
    }

    /// Gets a slice of bytes, size determined by the given opcode
    pub fn get_bytes_for_bytecode(&self, opcode: &ByteCode, index: usize) -> Option<&[u8]> {
        if opcode.operand_bytes() > 0 {
            Some(&self.0[(index + 1)..(index + 1 + opcode.operand_bytes())])
        } else {
            None
        }
    }

    /// Tries to get the next opcode after the one at this position
    pub fn next_bytecode(&self, index: usize) -> Option<usize> {
        let (_, bytes) = self.bytecode_and_operands(index);
        let count = 1 + bytes.map(|t| t.len()).unwrap_or(0);
        if index + count >= self.0.len() {
            None
        } else {
            Some(index + count)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::chunk::Chunk;
    use crate::bytecode::{ByteCode, BinaryOpOperand};

    #[test]
    fn disassemble() {
        let mut chunk = Chunk::new();
        chunk.constant_2byte(2048);
        chunk.constant_4byte(2048);
        chunk.constant_2byte(2048);
        chunk.constant_4byte(2048);
        chunk.constant_8byte(0x21a2b);
        chunk.constant_float4(0.23);
        chunk.constant_float8(21321.2321);
        chunk.pop_n(12);
        chunk.binary_operator(ByteCode::Add, BinaryOpOperand::new(false, 8, 4));
        chunk.binary_operator(ByteCode::Subtract, BinaryOpOperand::new(false, 8, 2));
        chunk.ret();

        println!("{}", chunk.disassemble())
    }
}