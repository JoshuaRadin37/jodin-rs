//! The most basic, complex type that is just a record

use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::privacy::Visibility;
use std::sync::{Arc, Weak};

use crate::core::types::big_object::{JBigObject, JBigObjectBuilder};
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::type_environment::TypeEnvironment;
use crate::core::types::{
    get_type_id, CompoundType, Field, JField, JodinType, JodinTypeReference, Type,
};
use crate::utility::Visitor;

/// Contains a name and its fields
#[derive(Debug, Clone)]
pub struct Structure {
    name: Identifier,
    type_id: u32,
    fields: Vec<Field<Weak<JodinType>>>,
}

impl Structure {
    /// Creates a new named structure
    pub fn new(name: Identifier, fields: Vec<Field<&Arc<JodinType>>>) -> Self {
        Structure {
            name,
            type_id: get_type_id(),
            fields: fields
                .into_iter()
                .map(|Field { vis, jtype, name }| Field::new(vis, Arc::downgrade(jtype), name))
                .collect(),
        }
    }

    // /// Creates an anonymous structure
    // pub fn anonymous_struct(fields: Vec<(String, IntermediateType)>) -> Self {
    //     let type_id = get_type_id();
    //     let name: Identifier = format!("<anonymous struct {}>", type_id).into();
    //     Structure {
    //         name: name,
    //         type_id,
    //         fields: fields
    //             .into_iter()
    //             .map(|(name, ty)| Field {
    //                 vis: Visibility::Public,
    //                 jtype: ty,
    //                 name: Identifier::from(name),
    //             })
    //             .collect(),
    //     }
    // }

    /// Gets the fields of the structure
    pub fn fields(&self) -> &Vec<Field<Weak<JodinType>>> {
        &self.fields
    }
}

impl<'t> Visitor<'t, TypeEnvironment, JodinResult<JBigObject<'t>>> for Structure {
    fn visit(&'t self, environment: &'t TypeEnvironment) -> JodinResult<JBigObject<'t>> {
        let mut builder = environment.big_object_builder(self);
        Ok(builder.build())
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
    fn all_members(&self) -> Vec<&JField> {
        self.fields.iter().collect()
    }
}

impl From<Structure> for JodinType {
    fn from(s: Structure) -> Self {
        JodinType::Structure(s)
    }
}
