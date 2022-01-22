use crate::assembly::instructions::{Asm, Assembly, Bytecode, Encode};
use crate::assembly::location::AsmLocation;
use crate::core::literal::Literal;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Value {
    Empty,
    Byte(u8),
    Float(f64),
    Integer(i64),
    UInteger(u64),
    Str(String),
    Dictionary(HashMap<String, Value>),
    Array(Vec<Value>),
    Reference(Box<RefCell<Value>>),
    Bytecode(Bytecode),
    Function(AsmLocation),
    /// The native value is used to refer to two different states. When alone, the Native value
    /// is a reference to the actual virtual machine. When used as the value of an entry of an
    /// attribute that's being checked for, this means to pretend that there's no entry at all.
    Native,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Empty => {
                write!(f, "void")
            }
            Value::Byte(b) => {
                write!(f, "{}u8", b)
            }
            Value::Float(fl) => {
                write!(f, "{}f64", fl)
            }
            Value::Integer(i) => {
                write!(f, "{:+}i64", i)
            }
            Value::UInteger(i) => {
                write!(f, "{}u64", i)
            }
            Value::Str(s) => {
                write!(f, "{}", s)
            }
            Value::Dictionary(d) => {
                write!(
                    f,
                    "{:?}",
                    d.iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<HashMap<_, _>>()
                )
            }
            Value::Array(a) => {
                write!(
                    f,
                    "{:?}",
                    a.iter().map(|k| k.to_string()).collect::<Vec<_>>()
                )
            }
            Value::Reference(r) => {
                write!(f, "*{}", (&**r).borrow())
            }
            Value::Bytecode(b) => {
                write!(f, "{:?}", b)
            }
            Value::Function(fu) => {
                write!(f, "<{:?}>", fu)
            }
            Value::Native => {
                write!(f, "NATIVE")
            }
        }
    }
}

impl From<Literal> for Value {
    fn from(l: Literal) -> Self {
        match l {
            Literal::String(s) => Value::Str(s),
            Literal::Char(c) => Value::Byte(c as u8),
            Literal::Boolean(b) => Value::Byte(b.into()),
            Literal::Float(f) => Value::Float(f as f64),
            Literal::Double(d) => Value::Float(d),
            Literal::Byte(v) => Value::Integer(v as i64),
            Literal::Short(v) => Value::Integer(v as i64),
            Literal::Int(v) => Value::Integer(v as i64),
            Literal::Long(v) => Value::Integer(v as i64),
            Literal::UnsignedByte(b) => Value::Byte(b),
            Literal::UnsignedShort(v) => Value::UInteger(v as u64),
            Literal::UnsignedInt(v) => Value::UInteger(v as u64),
            Literal::UnsignedLong(v) => Value::UInteger(v as u64),
        }
    }
}

impl Value {
    pub fn into_string(self) -> Option<String> {
        if let Value::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn new<V: Into<Value>>(v: V) -> Self {
        v.into()
    }

    pub fn new_dict() -> Self {
        Value::Dictionary(Default::default())
    }

    pub fn into_reference(self) -> Self {
        match self {
            Value::Reference(_) => {
                panic!("Can not have a reference to a reference")
            }
            v => Value::Reference(Box::new(RefCell::new(v))),
        }
    }

    pub fn is_null_ptr(&self) -> bool {
        match self {
            Value::Reference(b) => {
                if let Value::Empty = &*(&**b).borrow() {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Byte(if b { 1 } else { 0 })
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

impl<V> From<Vec<V>> for Value
where
    Value: From<V>,
{
    fn from(vs: Vec<V>) -> Self {
        Value::Array(vs.into_iter().map(|v| v.into()).collect())
    }
}

impl<V> From<HashMap<String, V>> for Value
where
    Value: From<V>,
{
    fn from(m: HashMap<String, V>) -> Self {
        Value::Dictionary(m.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

impl<V> From<Vec<(String, V)>> for Value
where
    Value: From<V>,
{
    fn from(m: Vec<(String, V)>) -> Self {
        Value::Dictionary(m.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

impl<V, const N: usize> From<[(String, V); N]> for Value
where
    Value: From<V>,
{
    fn from(m: [(String, V); N]) -> Self {
        Value::Dictionary(m.into_iter().map(|(k, v)| (k.clone(), v.into())).collect())
    }
}

impl<V, const N: usize> From<[(&str, V); N]> for Value
where
    Value: From<V>,
{
    fn from(m: [(&str, V); N]) -> Self {
        Value::Dictionary(
            m.into_iter()
                .map(|(k, v)| (k.to_string(), v.into()))
                .collect(),
        )
    }
}

impl From<RefCell<Value>> for Value {
    fn from(r: RefCell<Value>) -> Self {
        Value::Reference(Box::new(r))
    }
}
