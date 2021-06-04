use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::literal::Literal;
use crate::parsing::ast::jodin_node::JodinNode;
use crate::parsing::ast::node_type::JodinNodeInner;
use crate::parsing::parser::JodinRule;
use pest::iterators::Pair;
use std::iter::FromIterator;

pub mod jodin_node;
pub mod node_type;
pub mod tags;

pub struct JodinNodeBuilder {
    built_ast: Option<JodinNode>,
}

struct SingleJodinNodeTreeCreator {
    path: String,
}

impl SingleJodinNodeTreeCreator {
    fn new(path: String) -> Self {
        SingleJodinNodeTreeCreator { path }
    }

    fn create_node_from_pair(&mut self, pair: Pair<JodinRule>) -> JodinResult<JodinNode> {
        Ok(match pair.as_rule() {
            JodinRule::single_identifier => {
                let string = pair.as_str();
                let id = Identifier::from(string);
                JodinNode::new(JodinNodeInner::Identifier(id))
            }
            JodinRule::identifier => {
                let inner = pair
                    .into_inner()
                    .into_iter()
                    .filter(|p| p.as_rule() == JodinRule::single_identifier)
                    .map(|p| p.as_str())
                    .collect::<Vec<_>>();
                let id = Identifier::from_iter(inner);
                JodinNodeInner::Identifier(id).into()
            }
            JodinRule::literal => {
                let literal_string = pair.as_str();
                let literal: Literal = literal_string.parse()?;
                JodinNodeInner::Literal(literal).into()
            }
            _ => panic!("Shouldn't have been able to reach this piece of code"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::parser::complete_parse;

    #[test]
    fn create_id() {
        let pairs = complete_parse(JodinRule::identifier, "hello::world").unwrap();
        let result = SingleJodinNodeTreeCreator::new("".to_string())
            .create_node_from_pair(pairs.into_iter().next().unwrap())
            .unwrap();
        let inner = result.inner();
        if let JodinNodeInner::Identifier(id) = inner {
            assert_eq!(id, &Identifier::from_iter(&["hello", "world"]));
        } else {
            panic!("Didn't create correct jodin node");
        }
    }
}
