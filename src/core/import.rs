use crate::core::identifier::Identifier;
use crate::parsing::ast::parse_identifier;
use crate::parsing::parser::JodinRule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq)]
pub struct Import {
    id: Identifier,
    import_type: ImportType,
}

impl Import {
    pub fn new(id: Identifier, import_type: ImportType) -> Self {
        Import { id, import_type }
    }

    pub fn id(&self) -> &Identifier {
        &self.id
    }
    pub fn import_type(&self) -> &ImportType {
        &self.import_type
    }

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

#[derive(Debug, PartialEq)]
pub enum ImportType {
    Direct,
    Aliased { alias: Identifier },
    Wildcard,
    Children { children: Vec<Import> },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::error::JodinErrorType;
    use crate::parsing::parser::complete_parse;
    use std::iter::FromIterator;

    #[test]
    fn parse_using_path() {
        let string = "std::{vec::*, map, a::b as c}";
        match complete_parse(JodinRule::using_path, string) {
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
