//! Traits are interfaces

use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::types::generic_context::GenericParameter;
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::resolved_type::{ResolveType, ResolvedType};
use crate::core::types::type_environment::TypeEnvironment;
use crate::core::types::{get_type_id, Field, JodinType, Type};
use crate::utility::Visitor;
use itertools::Itertools;
use std::fmt::{DebugStruct, Display, Formatter};
use std::sync::Arc;

/// A jodin trait structure
#[derive(Debug, Clone)]
pub struct JTrait {
    /// the identifier of the trait
    pub id: Identifier,
    type_id: u32,
    /// The generics of the trait
    pub generics: Vec<GenericParameter>,
    /// The super traits of this trait
    pub extends: Vec<Identifier>,
    pub entries: Vec<Field<IntermediateType>>,
}

impl Display for JTrait {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let title = format!(
            "trait {}{}",
            self.id,
            match self.generics.len() {
                0 => "".to_string(),
                _ => format!(
                    "<{}>",
                    self.generics.iter().map(|g| g.to_string()).join(",")
                ),
            }
        );
        let mut debug_struct = f.debug_struct(title.as_str());
        debug_struct.field("extends", &self.extends);
        for entry in &self.entries {
            let name = format!("({:?}) {}", entry.vis, entry.name);
            debug_struct.field(&name, &entry.jtype);
        }
        debug_struct.finish()
    }
}

impl JTrait {
    /// Create a new jtrait object
    pub fn new(
        id: Identifier,
        generics: Vec<GenericParameter>,
        extends: Vec<Identifier>,
        entries: Vec<Field<IntermediateType>>,
    ) -> Self {
        JTrait {
            id,
            type_id: get_type_id(),
            generics,
            extends,
            entries,
        }
    }
}

impl Into<JodinType> for JTrait {
    fn into(self) -> JodinType {
        JodinType::JTrait(self)
    }
}

impl ResolveType for JTrait {
    fn resolve(&self, environment: &TypeEnvironment) -> ResolvedType {
        todo!()
    }
}

impl<'t> Type<'t> for JTrait {
    fn type_identifier(&self) -> Identifier {
        self.id.clone()
    }

    fn type_unique_id(&self) -> u32 {
        self.type_id
    }
}

#[derive(Debug, Clone)]
pub struct JTraitObject {
    owner_type: Identifier,
    jtrait: IntermediateType,
    entries: Vec<Field<IntermediateType>>,
    type_id: u32,
}

impl Into<JodinType> for JTraitObject {
    fn into(self) -> JodinType {
        JodinType::JTraitObject(self)
    }
}

impl ResolveType for JTraitObject {
    fn resolve(&self, environment: &TypeEnvironment) -> ResolvedType {
        todo!()
    }
}

impl Type<'_> for JTraitObject {
    fn type_identifier(&self) -> Identifier {
        self.jtrait.identifier().unwrap() >> &self.owner_type
    }

    fn type_unique_id(&self) -> u32 {
        self.type_id
    }
}
