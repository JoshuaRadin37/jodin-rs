//! The function types
//!
//! They're actually traits btw

use crate::core::types::traits::JTrait;

/// Functions are traits
pub trait FunctionTrait {
    fn into_trait(self) -> JTrait;
}

pub struct FunctionType {}
