//! A way to track types within a project

use crate::core::identifier_resolution::Registry;
use crate::core::types::JodinTypeReference;

/// A type tracker
pub struct TypeTracker {
    hash_map: Registry<JodinTypeReference>,
}
