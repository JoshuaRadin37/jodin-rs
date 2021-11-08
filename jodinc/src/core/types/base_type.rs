use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::privacy::Visibility;
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::traits::JTrait;
use crate::core::types::Field;
use std::sync::atomic::{AtomicBool, Ordering};

static BASE_TYPE_GENERATED: AtomicBool = AtomicBool::new(false);

/// Generate the base type. Ensures that only one is ever created to prevent potential future errors
pub fn base_type() -> JodinResult<JTrait> {
    if Ok(false)
        == BASE_TYPE_GENERATED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
    {
        _base_type()
    } else {
        Err(JodinError::new(JodinErrorType::BaseTypeAlreadyGenerated))
    }
}

lazy_static! {
    pub static ref BASE_TYPE_ID: Identifier = Identifier::from("Object");
    pub static ref TO_STRING_ID: Identifier = &*BASE_TYPE_ID << &Identifier::from("to_string");
}

fn _base_type() -> JodinResult<JTrait> {
    let name = Identifier::from(&*BASE_TYPE_ID);

    let to_string = to_string_field();

    let fields = vec![to_string];

    Ok(JTrait::new(name, vec![], vec![], fields))
}

fn to_string_field() -> Field {
    let id = Identifier::from(&*TO_STRING_ID);
    let ty = IntermediateType::from(Identifier::from("String")).with_function_params(vec![]);
    Field::new(Visibility::Public, ty, id)
}

#[cfg(test)]
mod tests {
    use super::{_base_type, BASE_TYPE_ID};

    #[test]
    fn base_type() {
        let base_type = _base_type().expect("Creating the base type shouldn't fail");
        assert_eq!(&base_type.id, &*BASE_TYPE_ID);

        println!("{:#}", base_type);
    }
}
