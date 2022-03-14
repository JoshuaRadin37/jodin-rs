use crate::assembly::instructions::{Asm, Assembly, Bytecode, Encode};
use crate::assembly::location::AsmLocation;
use crate::core::literal::Literal;

use crate::error::{JodinError, JodinResult};
use anyhow::anyhow;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{write, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

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
    Reference(JRef),
    Bytecode(Bytecode),
    Function(AsmLocation),
    /// The native value is used to refer to two different states. When alone, the Native value
    /// is a reference to the actual virtual machine. When used as the value of an entry of an
    /// attribute that's being checked for, this means to pretend that there's no entry at all.
    Native,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JRef {
    inner: Rc<RefCell<Value>>,
}

impl JRef {
    pub fn new(v: impl Into<Value>) -> Self {
        let value = v.into();
        Self {
            inner: Rc::new(RefCell::new(value)),
        }
    }
}

impl From<Rc<RefCell<Value>>> for JRef {
    fn from(r: Rc<RefCell<Value>>) -> Self {
        Self { inner: r }
    }
}

impl Deref for JRef {
    type Target = RefCell<Value>;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl Serialize for JRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self.inner.borrow().clone();
        value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for JRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Ok(JRef::new(value))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Empty => {
                write!(f, "void")
            }
            Value::Byte(b) => {
                if f.alternate() {
                    write!(f, "{}", b)
                } else {
                    write!(f, "{}u8", b)
                }
            }
            Value::Float(fl) => {
                if f.alternate() {
                    write!(f, "{}", fl)
                } else {
                    write!(f, "{}f64", fl)
                }
            }
            Value::Integer(i) => {
                if f.alternate() {
                    write!(f, "{}", i)
                } else {
                    write!(f, "{:+}i64", i)
                }
            }
            Value::UInteger(i) => {
                if f.alternate() {
                    write!(f, "{}", i)
                } else {
                    write!(f, "{}u64", i)
                }
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
                if f.alternate() {
                    write!(f, "*{}", (&**r).borrow())
                } else {
                    write!(f, "{:p}", r.inner.as_ptr())
                }
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

    pub fn location(label: impl AsRef<str>) -> Self {
        let label = label.as_ref().to_string();
        Value::Function(AsmLocation::Label(label))
    }

    pub fn into_reference(self) -> Self {
        match self {
            Value::Reference(_) => {
                panic!("Can not have a reference to a reference")
            }
            v => Value::Reference(JRef::new(v)),
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

    pub fn try_hash<H: Hasher>(&self, hasher: &mut H) -> JodinResult<()> {
        match self {
            Value::Empty => {
                ().hash(hasher);
            }
            Value::Byte(b) => {
                b.hash(hasher);
            }
            Value::Integer(i) => {
                i.hash(hasher);
            }
            Value::UInteger(u) => {
                u.hash(hasher);
            }
            Value::Str(s) => {
                s.hash(hasher);
            }
            Value::Reference(r) => {
                r.as_ptr().hash(hasher);
            }
            _ => return Err(anyhow!("{self} can not be hashed").into()),
        }
        Ok(())
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Byte(if b { 1 } else { 0 })
    }
}

impl From<u8> for Value {
    fn from(b: u8) -> Self {
        Value::Byte(b)
    }
}

impl From<u16> for Value {
    fn from(b: u16) -> Self {
        Value::UInteger(b as u64)
    }
}

impl From<u32> for Value {
    fn from(b: u32) -> Self {
        Value::UInteger(b as u64)
    }
}

impl From<u64> for Value {
    fn from(b: u64) -> Self {
        Value::UInteger(b as u64)
    }
}

impl From<usize> for Value {
    fn from(b: usize) -> Self {
        Value::UInteger(b as u64)
    }
}

impl From<i8> for Value {
    fn from(b: i8) -> Self {
        Value::Integer(b as i64)
    }
}

impl From<i16> for Value {
    fn from(b: i16) -> Self {
        Value::Integer(b as i64)
    }
}

impl From<i32> for Value {
    fn from(b: i32) -> Self {
        Value::Integer(b as i64)
    }
}

impl From<i64> for Value {
    fn from(f: i64) -> Self {
        Value::Integer(f)
    }
}

impl From<isize> for Value {
    fn from(b: isize) -> Self {
        Value::Integer(b as i64)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
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
        Value::Reference(JRef { inner: Rc::new(r) })
    }
}
