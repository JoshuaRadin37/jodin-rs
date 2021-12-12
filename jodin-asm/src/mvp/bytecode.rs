//! The bytecode module contains all information regarding bytecodes.
//!
//! Does not store any information about how byte codes are actually implemented

use crate::mvp::location::AsmLocation;
use crate::mvp::value::Value;
use bitfield::bitfield;
use byteorder::{ByteOrder, LittleEndian};

/// The size of pointers
pub const PTR_SIZE: usize = std::mem::size_of::<usize>();

/// The VM's bytecode are op codes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Asm {
    Label(String),

    Nop,
    Halt,

    /// Always go to this location
    Goto(AsmLocation),
    /// Go to this location if the value on the stack is NOT 0.
    CondGoto(AsmLocation),

    Push(Value),
    /// Clears the stack
    Clear,

    /// Pops the top most value on the stack and saves it to the next available variable number
    NextVar(u64),
    SetVar(u64),
    GetVar(u64),
    ClearVar(u64),

    /// Gets an attribute from a dictionary.
    GetAttribute(String),
    /// Gets a value from an array
    Index(usize),
    /// Packs {0} amount of values from the stack into an array
    Pack(usize),

    /// Return to the previous frame
    Return,
    /// Calls a function. Passes a popped value as an argument.
    Call(AsmLocation),

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

    /// & two values on the stack
    And,
    /// | two values on the stack
    Not,
    /// ! a value on the stack
    Or,

    /// Enables the _dynamic_ aspect in this location. Pops 3 values from the stack.
    /// The first is the object being sent a message. The second is the "name" of the message.
    /// The third is an array of arguments.
    ///
    /// There should be a default implementation for all value types, but with the ability to override
    /// somehow.
    SendMessage,
}

impl Asm {
    pub fn push<V>(value: V) -> Self
    where
        Value: From<V>,
    {
        Self::Push(value.into())
    }
}

pub type Assembly = Vec<Asm>;
pub type Bytecode = Vec<u8>;

pub trait Encode {
    fn encode(self) -> Bytecode;
}

impl Encode for Assembly {
    fn encode(self) -> Bytecode {
        bincode::serialize(&self).unwrap()
    }
}

pub trait Decode {
    fn decode(self) -> Assembly;
}

impl Decode for Bytecode {
    fn decode(self) -> Assembly {
        bincode::deserialize(&*self).unwrap()
    }
}
