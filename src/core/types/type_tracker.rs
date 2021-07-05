use std::collections::HashMap;
use crate::core::identifier::Identifier;
use crate::core::types::JodinTypeReference;

pub struct TypeTracker {
    hash_map: HashMap<Identifier, JodinTypeReference>
}