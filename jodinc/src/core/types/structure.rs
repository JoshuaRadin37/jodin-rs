//! The most basic, complex type that is just a record

use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::privacy::Visibility;

use crate::core::types::big_object::JBigObject;
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::type_environment::TypeEnvironment;
use crate::core::types::{get_type_id, CompoundType, Field, JodinType, JodinTypeReference, Type};
use crate::utility::Visitor;

/// Contains a name and its fields
#[derive(Debug)]
pub struct Structure {
    name: Identifier,
    type_id: u32,
    fields: Vec<Field>,
}

impl Structure {
    /// Creates a new named structure
    pub fn new(name: Identifier, fields: Vec<Field>) -> Self {
        Structure {
            name,
            type_id: get_type_id(),
            fields,
        }
    }

    /// Creates an anonymous structure
    pub fn anonymous_struct(fields: Vec<(String, IntermediateType)>) -> Self {
        let type_id = get_type_id();
        let name: Identifier = format!("<anonymous struct {}>", type_id).into();
        Structure {
            name: name,
            type_id,
            fields: fields
                .into_iter()
                .map(|(name, ty)| Field {
                    vis: Visibility::Public,
                    jtype: ty,
                    name: Identifier::from(name),
                })
                .collect(),
        }
    }

    /// Gets the fields of the structure
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}

impl <'t> Visitor<TypeEnvironment, JodinResult<JBigObject<'t>>> for Structure {
    fn visit(&self, environment: &TypeEnvironment) -> JodinResult<JBigObject<'t>> {
        todo!()
    }
}

impl Type<'_> for Structure {
    fn type_identifier(&self) -> Identifier {
        self.name.clone()
    }

    fn type_unique_id(&self) -> u32 {
        self.type_id
    }
}

impl CompoundType<'_> for Structure {
    fn all_members(&self) -> Vec<(&Visibility, &IntermediateType, &Identifier)> {
        self.fields.iter().map(|field| field.as_tuple()).collect()
    }
}

impl From<Structure> for JodinType {
    fn from(s: Structure) -> Self {
        JodinType::Structure(s)
    }
}
