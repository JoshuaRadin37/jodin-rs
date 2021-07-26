use crate::core::identifier::Identifier;
use crate::core::types::JodinTypeReference;
use std::collections::HashMap;

pub struct TypeTracker {
    hash_map: HashMap<Identifier, JodinTypeReference>,
}
