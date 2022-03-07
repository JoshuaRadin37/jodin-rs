//! The bytecode module contains all information regarding bytecodes.
//!
//! Does not store any information about how byte codes are actually implemented

use crate::assembly::error::BytecodeError;
use crate::assembly::location::AsmLocation;
use crate::assembly::value::Value;

/// The size of pointers
pub const PTR_SIZE: usize = std::mem::size_of::<usize>();

/// The VM's bytecode are op codes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum Asm {
    Label(String),
    /// A public facing label
    PublicLabel(String),

    /// Marks the start of static code. A return statement breaks out of static code
    Static,

    Nop,

    Halt,
    /// Always go to this location
    Goto(AsmLocation),
    /// Go to this location if the value on the stack is NOT 0.
    CondGoto(AsmLocation),

    /// Push a [Value](Value) to the stack
    Push(Value),
    /// Pops the top of the stack
    Pop,
    /// Clears the stack
    Clear,

    /// Pops the top most value on the stack and saves it to the variable number
    SetVar(u64),
    /// Pushes var # to the top of the stack
    GetVar(u64),
    /// Clears variable #
    ClearVar(u64),

    /// Push some value associated with a symbol.
    GetSymbol(String),
    /// Pops the top of the stack and sets it to symbol.
    ///
    /// # Warning
    /// Only works while VM is in "kernel" mode
    SetSymbol(String),

    /// Gets an attribute from a dictionary.
    GetAttribute(String),
    /// Gets a value from an array
    Index(usize),
    /// Packs n amount of values from the stack into an array
    Pack(usize),
    /// Dereference a pointer
    Deref,
    /// Gets a reference
    GetRef,
    /// Pop the first value, which is a reference, and then pops a second value
    /// and sets the ref to that value
    SetRef,

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

    /// Checks if top of stack is greater than 0
    GT0,
    /// Checks if the first popped val is > then the second popped val
    Gt,

    /// & two values on the stack
    And,
    /// ! a values on the stack
    Not,
    /// || two values on the stack
    Or,

    /// Pop the top of the stack and makes the popped value either be
    /// a 1u8 or a 0u8.
    ///
    /// - !0 -> 1
    /// - 0 -> 0
    Boolify,

    // boolean ops

    BooleanAnd,
    BooleanOr,
    BooleanNot,
    BooleanXor,

    /// Enables the _dynamic_ aspect in this location. Pops 3 values from the stack.
    /// The first is the object being sent a message. The second is the "name" of the message.
    /// The third is an array of arguments.
    ///
    /// There should be a default implementation for all value types, but with the ability to override
    /// somehow.
    SendMessage,
    // send message shortcuts
    /// Last thing on stack becomes a reference
    IntoReference,
    /// Invoke native method with the last N values are args
    NativeMethod(String, usize),
}

impl Asm {
    pub fn label<S: AsRef<str>>(lbl: S) -> Self {
        Self::Label(lbl.as_ref().to_string())
    }

    pub fn pub_label<S: AsRef<str>>(lbl: S) -> Self {
        Self::PublicLabel(lbl.as_ref().to_string())
    }

    pub fn push<V>(value: V) -> Self
    where
        Value: From<V>,
    {
        Self::Push(value.into())
    }

    pub fn goto(lbl: impl AsRef<str>) -> Self {
        Self::Goto(AsmLocation::Label(lbl.as_ref().to_string()))
    }
    pub fn cond_goto(lbl: impl AsRef<str>) -> Self {
        Self::CondGoto(AsmLocation::Label(lbl.as_ref().to_string()))
    }

    pub fn native_method<S: AsRef<str>, I: Into<Option<usize>>>(native: S, args: I) -> Self {
        let args = args.into().unwrap_or(0);
        Self::NativeMethod(native.as_ref().to_string(), args)
    }

    pub fn get_attribute<S>(attribute_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Self::GetAttribute(attribute_name.as_ref().to_string())
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

pub trait GetAsm {
    fn get_asm(&self) -> Assembly;
}

impl GetAsm for Assembly {
    fn get_asm(&self) -> Assembly {
        self.clone()
    }
}

impl GetAsm for &Assembly {
    fn get_asm(&self) -> Assembly {
        (*self).clone()
    }
}

impl<GB: GetBytecode> GetAsm for GB {
    fn get_asm(&self) -> Vec<Asm> {
        let bytecode = self.get_bytecode().expect("Could not get bytecode");
        bytecode.decode()
    }
}

/// You can get bytecode from this object.
pub trait GetBytecode {
    fn get_bytecode(&self) -> Result<Bytecode, BytecodeError>;
}
