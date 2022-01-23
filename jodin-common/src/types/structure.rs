//! The most basic, complex type that is just a record

use crate::identifier::Identifier;

use crate::types::intermediate_type::IntermediateType;
use crate::types::resolved_type::{BuildResolvedType, ResolvedTypeBuilder};

use crate::types::{get_type_id, CompoundType, Field, JodinType, Type};

/// Contains a name and its fields
#[derive(Debug, Clone)]
pub struct Structure {
    name: Identifier,
    type_id: u32,
    fields: Vec<Field<IntermediateType>>,
}

impl BuildResolvedType<'_> for Structure {
    fn build_resolved_type(&self, builder: &mut ResolvedTypeBuilder) {
        let arc = builder.env().get_type_by_name(&self.name).unwrap();
        builder.set_base_type(arc);
    }
}

impl Structure {
    /// Creates a new named structure
    pub fn new(name: Identifier, fields: Vec<Field<IntermediateType>>) -> Self {
        Structure {
            name,
            type_id: get_type_id(),
            fields,
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
    pub fn fields(&self) -> Vec<&Field<IntermediateType>> {
        self.fields.iter().collect()
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
    fn all_members(&self) -> Vec<&Field<IntermediateType>> {
        self.fields.iter().collect()
    }
}

impl From<Structure> for JodinType {
    fn from(s: Structure) -> Self {
        JodinType::Structure(s)
    }
}
