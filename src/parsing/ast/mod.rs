use crate::core::error::{JodinError, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::literal::Literal;
use crate::parsing::ast::jodin_node::JodinNode;
use crate::parsing::ast::node_type::JodinNodeInner;
use crate::parsing::parser::JodinRule;
use crate::passes::toolchain::{
    FallibleCollectorTool, JodinFallibleCollectorTool, JodinFallibleTool,
};
use pest::iterators::Pair;
use std::iter::FromIterator;
use std::marker::PhantomData;

pub mod jodin_node;
pub mod node_type;
pub mod tags;

pub struct JodinNodeBuilder<'a> {
    built_ast: Option<JodinNode>,
    _data: &'a PhantomData<()>,
}

impl<'a> JodinNodeBuilder<'a> {
    pub fn new() -> Self {
        JodinNodeBuilder {
            built_ast: None,
            _data: &PhantomData,
        }
    }

    /// Add a source string to the builder
    pub fn add_source_string(&mut self, path: &str, pair: Pair<JodinRule>) -> JodinResult<()> {
        let mut builder = SingleJodinNodeTreeCreator::new(path.to_string());
        let tree: JodinNode = builder.invoke(pair)?;

        Ok(())
    }

    /// Finishes the ast tree
    pub fn finish(self) -> JodinResult<JodinNode> {
        self.built_ast.ok_or(JodinError::EmptyJodinTree)
    }
}

impl<'a> JodinFallibleCollectorTool for JodinNodeBuilder<'a> {
    type Input = (&'a str, Pair<'a, JodinRule>);
    type Output = ();

    fn invoke<I: IntoIterator<Item = Self::Input>>(
        &mut self,
        input_iter: I,
    ) -> JodinResult<Self::Output> {
        for (path, pair) in input_iter {
            self.add_source_string(path, pair)?;
        }
        Ok(())
    }
}

pub fn parse_identifier(pair: Pair<JodinRule>) -> JodinResult<Identifier> {
    match pair.as_rule() {
        JodinRule::single_identifier => {
            let string = pair.as_str();
            let id = Identifier::from(string);
            Ok(id)
        }
        JodinRule::identifier => {
            let inner = pair
                .into_inner()
                .into_iter()
                .filter(|p| p.as_rule() == JodinRule::single_identifier)
                .map(|p| p.as_str())
                .collect::<Vec<_>>();
            let id = Identifier::from_iter(inner);
            Ok(id)
        }
        _ => panic!(
            "Shouldn't use this function on this type of node: {:?}",
            pair
        ),
    }
}

struct SingleJodinNodeTreeCreator<'a> {
    path: String,
    _data: &'a PhantomData<()>,
}

impl SingleJodinNodeTreeCreator<'_> {
    fn new(path: String) -> Self {
        SingleJodinNodeTreeCreator {
            path,
            _data: &PhantomData,
        }
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

impl<'a> JodinFallibleTool for SingleJodinNodeTreeCreator<'a> {
    type Input = Pair<'a, JodinRule>;
    type Output = JodinNode;

    fn invoke(&mut self, input: Self::Input) -> JodinResult<Self::Output> {
        let mut ret = self.create_node_from_pair(input);
        if let Err(JodinError::ParserError(_, path)) = &mut ret {
            *path = Some(self.path.clone())
        }
        ret
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
