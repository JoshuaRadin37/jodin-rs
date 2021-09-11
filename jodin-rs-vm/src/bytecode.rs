//! The bytecode module contains all information regarding bytecodes.
//!
//! Does not store any information about how byte codes are actually implemented

use byteorder::{BigEndian, ByteOrder};
use bitfield::bitfield;

/// The size of pointers
pub const PTR_SIZE: usize = std::mem::size_of::<usize>();

/// The VM's bytecode are op codes
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
#[non_exhaustive]
pub enum ByteCode {
    Nop,
    Halt,
    WaitForRunCode,
    /// Return to the previous frame
    Return,
    /// Put one byte onto the stack
    Const1,
    /// Put two bytes onto the stack
    Const2,
    /// Puts four bytes onto the stack
    Const4,
    /// Puts eight bytes onto the stack
    Const8,
    /// Puts a 32 bit float onto the stack
    Float4,
    /// Puts a 64 bite float onto the stack
    Float8,
    /// Pop N bytes from the stack
    PopN,
    /// Add two values
    Add,
    /// Subtract two values
    Subtract,
    /// Multiply two values
    Multiply,
    /// Divide two values
    Divide,
    /// Get the remainder of two values
    Remainder,
}

impl ByteCode {
    /// Get the number of operand bytes used by an instruction
    pub fn operand_bytes(&self) -> usize {
        match self {
            ByteCode::Return => 0,
            ByteCode::Const1 => 1,
            ByteCode::Const2 => 2,
            ByteCode::Const4 => 4,
            ByteCode::Const8 => 8,
            ByteCode::Float4 => 4,
            ByteCode::Float8 => 8,
            ByteCode::PopN => PTR_SIZE,
            ByteCode::Add => 1,
            ByteCode::Subtract => 1,
            ByteCode::Multiply => 1,
            ByteCode::Divide => 1,
            ByteCode::Remainder => 1,
            _ => 0,
        }
    }

    /// If this op has operand bytes, attempts to create a readable string from it
    pub fn interpret_operand_as_string(&self, bytes: &[u8]) -> Option<String> {
        if bytes.is_empty() {
            return None;
        }
        match self {
            ByteCode::Const1 => {
                Some(bytes[0].to_string())
            }
            ByteCode::Const2 => {
                Some(BigEndian::read_u16(bytes).to_string())
            }
            ByteCode::Const4 => {
                Some(BigEndian::read_u32(bytes).to_string())
            }

            ByteCode::Const8 => {
                Some(BigEndian::read_u64(bytes).to_string())
            }
            ByteCode::Float4 => {
                Some(BigEndian::read_f32(bytes).to_string())
            }
            ByteCode::Float8 => {
                Some(BigEndian::read_f64(bytes).to_string())
            }
            ByteCode::PopN => {
                if PTR_SIZE == 4 {
                    Some(BigEndian::read_u32(bytes).to_string())
                } else if PTR_SIZE == 8 {
                    Some(BigEndian::read_u64(bytes).to_string())
                } else {
                    panic!("Only 32 byte or 64 byte systems supported")
                }
            }
            ByteCode::Return |
            ByteCode::Add |
            ByteCode::Subtract |
            ByteCode::Multiply |
            ByteCode::Divide |
            ByteCode::Remainder => {
                let b = BinaryOpOperand(bytes[0]);
                Some(format!("{} {} bytes, {} bytes", if b.is_floats() { "float"} else { "integer"}, b.lhs_size(), b.rhs_size()))
            }
            _ => None
        }
    }


}

bitfield! {
    pub struct BinaryOpOperand(u8);
    impl Debug;
    u8;
    _is_floats, set_is_floats: 6;
    min_lhs_size, set_lhs_size: 5, 3;
    min_rhs_size, set_rhs_size: 2, 0;
}

impl BinaryOpOperand {
    /// Create the operand for a binary operator
    pub fn new(is_floats: bool, lhs_size: usize, rhs_size: usize) -> Self {
        let mut ret = BinaryOpOperand(0);
        ret.set_is_floats(is_floats);
        ret.set_lhs_size((lhs_size - 1) as u8);
        ret.set_rhs_size((rhs_size - 1) as u8);
        ret
    }

    /// Whether this binary op is on floats
    pub fn is_floats(&self) -> bool {
        self._is_floats()
    }
    
    /// Gets the size of the lhs operand
    pub fn lhs_size(&self) -> usize {
        self.min_lhs_size() as usize + 1
    }

    /// Gets the size of the rhs operand
    pub fn rhs_size(&self) -> usize {
        self.min_rhs_size() as usize + 1
    }
}
