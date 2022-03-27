//! How importing of types, functions, and modules will be handled in jodin.
//!
//! Imports are handled as like trees, and must refer to absolute paths. Imports can't be exported.
//! These are all valid imports:
//! - `namespace::id`
//! - `namespace::*`
//! - `namespace::{id1, id2}`
//! - `namespace::{id1, id2::id3 as id3}`

use crate::identifier::Identifier;
#[cfg(feature = "pest_parser")]
use crate::parsing::JodinRule;
#[cfg(feature = "pest_parser")]
use pest::iterators::Pair;

/// Represents an import "tree".
#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    id: Identifier,
    import_type: ImportType,
}

impl Import {
    /// Create a new import.
    ///
    /// # Arguments
    ///
    /// * `id`: the base identifier
    /// * `import_type`: the type of import
    ///
    /// returns: Import
    pub fn new(id: Identifier, import_type: ImportType) -> Self {
        Import { id, import_type }
    }

    /// The base identifier.
    pub fn id(&self) -> &Identifier {
        &self.id
    }
    /// The import type.
    pub fn import_type(&self) -> &ImportType {
        &self.import_type
    }

    /// Concatenate parent info to this id
    pub fn concat_parent_to_id(&self, parent: &Identifier) -> Self {
        Self {
            id: parent + &self.id,
            import_type: self.import_type.clone(),
        }
    }

    /// Gets the modules that are imported by this import.
    ///
    /// # Important
    /// This _isn't_ the identifiers that are being imported, but the modules that **OWN** the identifiers
    pub fn imported_modules(&self) -> Vec<Identifier> {
        match &self.import_type {
            ImportType::Direct | ImportType::Aliased { .. } => {
                self.id.parent().into_iter().cloned().collect()
            }
            ImportType::Wildcard => {
                vec![self.id.clone()]
            }
            ImportType::Children { children } => children
                .iter()
                .map(|import| import.imported_modules())
                .map(|modules| modules.into_iter().map(|id| &self.id + &id))
                .flatten()
                .collect(),
        }
    }
}
/// The type of import
#[derive(Debug, PartialEq, Clone)]
pub enum ImportType {
    /// Import exactly this identifier.
    Direct,
    /// Import exactly this identifier, but give it another name to refer to as.
    Aliased {
        /// The aliased name
        alias: Identifier,
    },
    /// Import everything within this namespace.
    Wildcard,
    /// Import these children
    Children {
        /// The child imports
        children: Vec<Import>,
    },
}

#[cfg(test)]
mod tests {
    use crate::core::import::{Import, ImportType};
    use crate::identifier::Identifier;

    #[test]
    fn get_imports() {
        let import = Import::new(
            id!(nm1),
            ImportType::Children {
                children: vec![
                    Import::new(id!(id1), ImportType::Direct),
                    Import::new(id!(nm2), ImportType::Wildcard),
                    Import::new(id!(nm3::id2), ImportType::Direct),
                ],
            },
        );

        let modules = import.imported_modules();
        assert_eq!(modules, vec![id!(nm1::nm2), id!(nm1::nm3)]);
    }
}
