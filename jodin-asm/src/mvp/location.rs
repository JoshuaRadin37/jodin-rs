use crate::mvp::error::BytecodeError;
use crate::mvp::value::Value;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AsmLocation {
    ByteIndex(usize),
    InstructionDiff(isize),
    Label(String),
}

impl TryFrom<&Value> for AsmLocation {
    type Error = BytecodeError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::UInteger(u) => Ok(AsmLocation::ByteIndex(usize::try_from(*u)?)),
            Value::Integer(i) => Ok(AsmLocation::InstructionDiff(isize::try_from(*i)?)),
            Value::Str(s) => Ok(AsmLocation::Label(s.clone())),
            v => Err(BytecodeError::InvalidLocationFromValue(v.clone())),
        }
    }
}
