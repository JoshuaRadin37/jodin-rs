//! How importing of types, functions, and modules will be handled in jodin.
//!
//! Imports are handled as like trees, and must refer to absolute paths. Imports can't be exported.
//! These are all valid imports:
//! - `namespace::id`
//! - `namespace::*`
//! - `namespace::{id1, id2}`
//! - `namespace::{id1, id2::id3 as id3}`

use crate::core::identifier::Identifier;
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

    /// Create an import value by evaluating part of a parse tree.
    #[cfg(feature = "pest_parser")]
    pub fn from_pair(pair: Pair<JodinRule>) -> Import {
        if JodinRule::using_path != pair.as_rule() {
            panic!(
                "Non using path given to function, given: {:?}",
                pair.as_rule()
            )
        }

        let mut inner = pair.into_inner().collect::<Vec<_>>();
        let base_id = parse_identifier(inner.drain(..=0).next().unwrap()).unwrap();
        let inner_rules = inner.iter().map(|m| m.as_rule()).collect::<Vec<_>>();

        let import_type = match inner_rules[..] {
            [] => ImportType::Direct,
            [JodinRule::t_star] => ImportType::Wildcard,
            [JodinRule::t_as, JodinRule::single_identifier] => {
                let id = parse_identifier(inner.pop().unwrap()).unwrap();
                ImportType::Aliased { alias: id }
            }
            _ => {
                let children = inner
                    .into_iter()
                    .map(|using_path| Import::from_pair(using_path))
                    .collect();
                ImportType::Children { children }
            }
        };

        Import::new(base_id, import_type)
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

    #[cfg(feature = "pest_parser")]
    use crate::parsing::complete_parse;

    #[test]
    #[cfg(feature = "pest_parser")]
    fn parse_using_path() {
        let string = "std::{vec::*, map, a::b as c}";
        match complete_parse(JodinRule::using_path, string).map_err(|e| e.into_err_and_bt().0) {
            Err(JodinErrorType::ParserError(e, ..)) => {
                println!("{}", e);
                panic!()
            }
            Err(e) => {
                panic!("{:?}", e)
            }
            Ok(mut pairs) => {
                println!("{:#?}", pairs);
                let import = Import::from_pair(pairs.nth(0).unwrap());
                println!("{:#?}", import);
                let expected = Import::new(
                    Identifier::from("std"),
                    ImportType::Children {
                        children: vec![
                            Import::new(Identifier::from("vec"), ImportType::Wildcard),
                            Import::new(Identifier::from("map"), ImportType::Direct),
                            Import::new(
                                Identifier::from_array(["a", "b"]),
                                ImportType::Aliased {
                                    alias: Identifier::from("c"),
                                },
                            ),
                        ],
                    },
                );
                assert_eq!(import, expected);
            }
        }
    }
}
