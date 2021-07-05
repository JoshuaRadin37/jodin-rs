use crate::compilation_settings::CompilationSettings;
use crate::core::error::{JodinErrorType, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::import::Import;
use crate::core::literal::Literal;
use crate::parsing::ast::jodin_node::JodinNode;
use crate::parsing::ast::node_type::JodinNodeInner;
use crate::parsing::parser::JodinRule;
use crate::passes::toolchain::{
    FallibleCollectorTool, JodinFallibleCollectorTool, JodinFallibleTool,
};
use pest::iterators::Pair;
use std::fs::File;
use std::io::Write;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

pub mod jodin_node;
pub mod node_type;
pub mod tags;

pub struct JodinNodeBuilder<'a> {
    built_ast: Option<JodinNode>,
    settings: &'a CompilationSettings,
}

impl<'a> JodinNodeBuilder<'a> {
    pub fn new(settings: &'a CompilationSettings) -> Self {
        JodinNodeBuilder {
            built_ast: None,
            settings,
        }
    }

    /// Add a source string to the builder
    pub fn add_source_string(&mut self, path: String, pair: Pair<JodinRule>) -> JodinResult<()> {
        let mut builder = SingleJodinNodeTreeCreator::new(path.clone().to_string());
        let mut tree: JodinNode = builder.invoke(pair).map_err(|mut err| {
            if let JodinErrorType::ParserError(_, path_opt) = &mut err.error_type_mut() {
                *path_opt = Some(path.clone());
            }
            err
        })?;

        if self.settings.output_ast {
            println!("{}", path);
            let string = format!("{:#?}", tree);
            let mut new_path = PathBuf::from(path);
            new_path.set_extension("ast");
            println!("Trying to make {:?}", new_path);
            let newer_path = self.settings.output_file_path(new_path);

            let mut file = File::create(newer_path)?;
            writeln!(file, "{}", string)?;
        }

        match (&mut self.built_ast, tree.inner_mut()) {
            (Some(built), JodinNodeInner::TopLevelDeclarations { decs }) => {
                if let JodinNodeInner::TopLevelDeclarations { decs: old_decs } = built.inner_mut() {
                    let decs = std::mem::replace(decs, vec![]);
                    old_decs.extend(decs);
                } else {
                    unreachable!()
                }
            }
            (missing, JodinNodeInner::TopLevelDeclarations { decs }) => *missing = Some(tree),
            _ => {
                panic!("Non top level decs declaration created")
            }
        }

        Ok(())
    }

    /// Finishes the ast tree
    pub fn finish(self) -> JodinResult<JodinNode> {
        self.built_ast.ok_or(JodinErrorType::EmptyJodinTree.into())
    }
}

impl<'a> JodinFallibleCollectorTool for JodinNodeBuilder<'a> {
    type Input = (String, Pair<'a, JodinRule>);
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
            JodinRule::top_level_declarations => {
                let mut decs = vec![];
                for pair in pair.into_inner() {
                    decs.push(self.create_node_from_pair(pair)?);
                }
                JodinNodeInner::TopLevelDeclarations { decs }.into()
            }
            JodinRule::using_statement => {
                let path = pair.into_inner().nth(0).unwrap();
                let import = Import::from_pair(path);
                JodinNodeInner::UsingIdentifier {
                    import_data: import,
                }
                .into()
            }
            JodinRule::in_namespace => {
                let mut inner = pair.into_inner();
                let id = inner.nth(0).unwrap();
                let id_node = self.create_node_from_pair(id)?;
                let affected = inner.nth(0).unwrap();
                let affected_node = self.create_node_from_pair(affected)?;
                JodinNodeInner::InNamespace {
                    namespace: id_node,
                    inner: affected_node,
                }
                .into()
            }
            //JodinRule::function_definition => {}
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
            JodinRule::declaration => {
                let mut inner = pair.into_inner();
                let canonical_type = self.create_node_from_pair(inner.nth(0).unwrap())?;
                let mut declarator_list = inner.nth(0).unwrap();
                let pairs = declarator_list
                    .into_inner()
                    .into_iter();
                let mut names = Vec::new();
                let mut values = Vec::new();
                for init_declarator in pairs {
                    let mut inner = init_declarator.into_inner();
                    let name = self.create_node_from_pair(inner.nth(0).unwrap())?;
                    let value = match inner.nth(0) {
                        Some(initializer) => {
                            Some(self.create_node_from_pair(initializer)?)
                        },
                        None => None
                    };
                    names.push(name);
                    values.push(value);
                }

                JodinNodeInner::VarDeclarations {
                    var_type: canonical_type,
                    names,
                    values
                }.into()
            },
            JodinRule::canonical_type => {
                todo!()
            }
            // just go into inner
            JodinRule::top_level_declaration | JodinRule::jodin_file => {
                let inner = pair.into_inner().nth(0).unwrap();
                self.create_node_from_pair(inner)?
            }
            rule => return Err(JodinErrorType::InvalidJodinRuleForASTCreation(rule).into()),
        })
    }
}

impl<'a> JodinFallibleTool for SingleJodinNodeTreeCreator<'a> {
    type Input = Pair<'a, JodinRule>;
    type Output = JodinNode;

    fn invoke(&mut self, input: Self::Input) -> JodinResult<Self::Output> {
        let mut ret = self.create_node_from_pair(input);
        if let Err(JodinErrorType::ParserError(_, path)) =
            ret.as_mut().map_err(|mut e| e.error_type_mut())
        {
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
