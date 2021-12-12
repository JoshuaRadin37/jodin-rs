use crate::mvp::bytecode::{Asm, Assembly, Bytecode, Encode};
use crate::mvp::location::AsmLocation;
use num_traits::{PrimInt, Signed};
use smallvec::SmallVec;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Value {
    Empty,
    Byte(u8),
    Float(f64),
    Integer(i64),
    UInteger(u64),
    Str(String),
    Dictionary {
        dict: HashMap<String, Value>,
    },
    Array(Vec<Value>),
    Reference(RefCell<Value>),
    Bytecode(Bytecode),
    Function(AsmLocation),
    /// The native value is used to refer to two different states. When alone, the Native value
    /// is a reference to the actual virtual machine. When used as the value of an entry of an
    /// attribute that's being checked for, this means to pretend that there's no entry at all.
    Native,
}

impl Value {
    pub fn into_string(self) -> Option<String> {
        if let Value::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<i64> for Value {
    fn from(f: i64) -> Self {
        Value::Integer(f)
    }
}

impl From<u64> for Value {
    fn from(f: u64) -> Self {
        Value::UInteger(f)
    }
}

impl From<&str> for Value {
    fn from(f: &str) -> Self {
        Value::Str(f.to_string())
    }
}

impl From<String> for Value {
    fn from(f: String) -> Self {
        Value::Str(f)
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Empty
    }
}

impl From<&[Asm]> for Value {
    fn from(asm: &[Asm]) -> Self {
        let assemble: Vec<Asm> = Vec::from(asm);
        let as_bytecode = assemble.encode();
        Value::Bytecode(as_bytecode)
    }
}

impl From<Assembly> for Value {
    fn from(asm: Assembly) -> Self {
        let as_bytecode = asm.encode();
        Value::Bytecode(as_bytecode)
    }
}
