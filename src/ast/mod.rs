//! This module contains all of the relevant parts for how ASTs are created.
//!
//! The abstract syntax tree should be made up of as few different types as possible, and when
//! instead of adding more fields to a variant of the [node type] enum, instead tags should be added.
//!
//! Tags are a way of adding information to the AST without needing to have many different fields for
//! every single instance of a [JodinNode]
//!
//! [node type]: self::node_type::JodinNodeInner
//! [JodinNode]: self::jodin_node::JodinNode

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::path::PathBuf;

use pest::iterators::{Pair, Pairs};
use pest::RuleType;

use crate::ast::intermediate_type::{IntermediateType, TypeSpecifier, TypeTail};
pub use crate::ast::jodin_node::JodinNode;
pub use crate::ast::node_type::JodinNodeInner;
use crate::compilation_settings::CompilationSettings;
use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::import::Import;
use crate::core::literal::Literal;
use crate::core::operator::Operator;
use crate::core::privacy::{Visibility, VisibilityTag};
use crate::core::types::primitives::Primitive;
use crate::parsing::JodinRule;
use crate::passes::toolchain::{JodinFallibleCollectorTool, JodinFallibleTool};
use std::convert::TryFrom;

pub mod intermediate_type;
mod jodin_node;
mod node_type;
pub mod tags;

/// Builds a JodinNode Abstract Syntax Tree.
pub struct JodinNodeBuilder<'a> {
    built_ast: Option<JodinNode>,
    settings: &'a CompilationSettings,
}

impl<'a> JodinNodeBuilder<'a> {
    /// Creates a new JodinNodeBuilder with a reference to the compilation settings.
    pub fn new(settings: &'a CompilationSettings) -> Self {
        JodinNodeBuilder {
            built_ast: None,
            settings,
        }
    }

    /// Add a source string to the builder
    pub fn add_source_string(&mut self, path: String, pair: Pair<JodinRule>) -> JodinResult<()> {
        let mut builder = JodinNodeGenerator::new(path.clone().to_string());
        let mut tree: JodinNode = builder.invoke(pair).map_err(|mut err| {
            if let JodinErrorType::ParserError(_, path_opt) = &mut err.error_type {
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
            (missing, JodinNodeInner::TopLevelDeclarations { decs: _ }) => *missing = Some(tree),
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

/// Parses a parse tree to attempt to create an identifier.
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

/// Creates a tree from a single string.
pub struct JodinNodeGenerator<'a> {
    path: String,
    _data: &'a PhantomData<()>,
}

impl JodinNodeGenerator<'_> {
    /// Creates a new instance from it's target string
    fn new(path: String) -> Self {
        JodinNodeGenerator {
            path,
            _data: &PhantomData,
        }
    }

    /// The main method that creates jodin nodes from a parse tree.
    pub fn generate_node(
        &mut self,
        pair: Pair<JodinRule>,
        _inherits: Vec<JodinNode>,
    ) -> JodinResult<JodinNode> {
        let inner_rules: Box<[JodinRule]> = pair_as_rules(&pair);
        println!("Rule: {:?} -> {:?}", pair.as_rule(), inner_rules);

        Ok(match pair.as_rule() {
            JodinRule::top_level_declarations => {
                let mut decs = vec![];
                for pair in pair.into_inner() {
                    decs.push(self.generate_node(pair, vec![])?);
                }
                JodinNodeInner::TopLevelDeclarations { decs }.into()
            }
            JodinRule::using_statement => {
                let path = pair.into_inner().nth(0).unwrap();
                let import = Import::from_pair(path);
                JodinNodeInner::ImportIdentifiers {
                    import_data: import,
                }
                .into()
            }
            JodinRule::in_namespace => {
                let mut inner = pair.into_inner();
                let id = inner.nth(0).unwrap();
                let id_node = self.generate_node(id, vec![])?;
                let affected = inner.nth(0).unwrap();
                let affected_node = self.generate_node(affected, vec![])?;
                JodinNodeInner::InNamespace {
                    namespace: id_node,
                    inner: affected_node,
                }
                .into()
            }
            //jodin_rule::function_definition => {}
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
            JodinRule::t_true => JodinNodeInner::Literal(Literal::Boolean(true)).into(),
            JodinRule::t_false => JodinNodeInner::Literal(Literal::Boolean(false)).into(),
            JodinRule::declaration => {
                let mut inner = pair.into_inner();
                let (_visibility, canonical_type, declarator_list) = match *inner_rules {
                    [JodinRule::visibility, JodinRule::canonical_type, JodinRule::init_declarator_list, ..] =>
                    {
                        println!("Visibility present");
                        (inner.next(), inner.next().unwrap(), inner.next().unwrap())
                    }
                    [JodinRule::canonical_type, JodinRule::init_declarator_list, ..] => {
                        println!("Visibility not present");
                        (None, inner.next().unwrap(), inner.next().unwrap())
                    }
                    _ => unreachable!(),
                };

                let canonical_type = self.generate_node(canonical_type, vec![])?;
                let pairs = declarator_list.into_inner().into_iter();
                let mut names = Vec::new();
                let mut values = Vec::new();

                for init_declarator in pairs {
                    println!("init declarator: {:?}", init_declarator.as_str());
                    let mut inner = init_declarator.into_inner();
                    let name = self.generate_node(inner.nth(0).unwrap(), vec![])?;
                    let value = match inner.nth(0) {
                        Some(initializer) => Some(self.generate_node(initializer, vec![])?),
                        None => None,
                    };
                    names.push(name);
                    values.push(value);
                }

                JodinNodeInner::VarDeclarations {
                    var_type: canonical_type,
                    names,
                    values,
                }
                .into()
            }
            JodinRule::canonical_type => {
                let intermediate_type = self.new_intermediate_type(pair)?;
                JodinNodeInner::Type(intermediate_type).into()
            }
            // Expressions
            JodinRule::expression => {
                let mut dict = IndexedPair::new(pair.into_inner());
                let expr =
                    self.generate_node(dict.get(JodinRule::double_or_expression)?, vec![])?;
                if let Ok(mut exprs) = dict.get_all(JodinRule::expression) {
                    let yes = self.generate_node(exprs.remove(0), vec![])?;
                    let no = self.generate_node(exprs.remove(0), vec![])?;
                    return JodinNodeInner::Ternary {
                        cond: expr,
                        yes,
                        no,
                    }
                    .into_result();
                }
                expr
            }
            // binops
            JodinRule::double_or_expression
            | JodinRule::double_and_expression
            | JodinRule::or_expression
            | JodinRule::xor_expression
            | JodinRule::and_expression
            | JodinRule::equation
            | JodinRule::c_expression
            | JodinRule::g_expression
            | JodinRule::t_expression
            | JodinRule::factor => {
                let mut inner = pair.into_inner();
                let lhs = self.generate_node(inner.nth(0).unwrap(), vec![])?;
                let mut rest: Vec<_> = inner.collect();
                if rest.is_empty() {
                    lhs
                } else {
                    let op = rest.remove(0);
                    let rhs = rest.remove(0);
                    let rhs = self.generate_node(rhs, vec![])?;
                    let op = match op.as_rule() {
                        JodinRule::t_dor => Operator::Dor,
                        JodinRule::t_or => Operator::Or,
                        JodinRule::t_dand => Operator::Dand,
                        JodinRule::t_xor => Operator::Xor,
                        JodinRule::t_and => Operator::And,
                        JodinRule::equality => {
                            let inner = op.into_inner().nth(0).unwrap();
                            let inner_rule = inner.as_rule();
                            match inner_rule {
                                JodinRule::t_eq => Operator::Equal,
                                JodinRule::t_neq => Operator::Nequal,
                                _ => unreachable!(),
                            }
                        }
                        JodinRule::comparison => {
                            let inner = op.into_inner().nth(0).unwrap();
                            let inner_rule = inner.as_rule();
                            match inner_rule {
                                JodinRule::t_lcarot => Operator::Lt,
                                JodinRule::t_rcarot => Operator::Gt,
                                JodinRule::t_lte => Operator::Lte,
                                JodinRule::t_gte => Operator::Gte,
                                _ => unreachable!(),
                            }
                        }
                        JodinRule::shift => {
                            let inner = op.into_inner().nth(0).unwrap();
                            let inner_rule = inner.as_rule();
                            match inner_rule {
                                JodinRule::t_lshift => Operator::LShift,
                                JodinRule::t_rshift => Operator::RShift,
                                _ => unreachable!(),
                            }
                        }
                        JodinRule::add_op => {
                            let inner = op.into_inner().nth(0).unwrap();
                            let inner_rule = inner.as_rule();
                            match inner_rule {
                                JodinRule::t_plus => Operator::Plus,
                                JodinRule::t_minus => Operator::Minus,
                                _ => unreachable!(),
                            }
                        }
                        JodinRule::mul_op => {
                            let inner = op.into_inner().nth(0).unwrap();
                            let inner_rule = inner.as_rule();
                            match inner_rule {
                                JodinRule::t_star => Operator::Star,
                                JodinRule::t_div => Operator::Divide,
                                JodinRule::t_mod => Operator::Modulo,
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    };
                    match rhs.into_inner() {
                        /*
                        JodinNodeInner::Binop {
                            op: op2,
                            lhs: mid,
                            rhs: rhs2,
                        } => {
                            let new_inner: JodinNode =
                                JodinNodeInner::Binop { op, lhs, rhs: mid }.into();
                            let outer = JodinNodeInner::Binop {
                                op: op2,
                                lhs: new_inner,
                                rhs: rhs2,
                            };
                            outer.into()
                        }

                         */
                        other => JodinNodeInner::Binop {
                            op,
                            lhs,
                            rhs: other.into(),
                        }
                        .into(),
                    }
                }
            }
            // uniop
            JodinRule::uni_op => {
                let mut inner = pair.into_inner();
                let operator = match inner.nth(0).unwrap().into_inner().nth(0).unwrap().as_rule() {
                    JodinRule::t_minus => Operator::Minus,
                    JodinRule::t_bang => Operator::Not,
                    JodinRule::t_star => Operator::Star,
                    JodinRule::t_and => Operator::And,
                    JodinRule::t_plus => Operator::Plus,
                    JodinRule::t_inc => Operator::Increment,
                    JodinRule::t_dec => Operator::Decrement,
                    _ => unreachable!(),
                };
                let factor = self.generate_node(inner.nth(0).unwrap(), vec![])?;
                JodinNodeInner::Uniop {
                    op: operator,
                    inner: factor,
                }
                .into()
            }
            JodinRule::cast_expression => {
                let mut indexed = IndexedPair::new(pair.into_inner());
                let canonical_type =
                    self.new_intermediate_type(indexed.get(JodinRule::canonical_type)?)?;
                let factor = self.generate_node(indexed.get(JodinRule::factor)?, vec![])?;
                JodinNodeInner::CastExpression {
                    to_type: canonical_type,
                    factor,
                }
                .into()
            }
            JodinRule::struct_definition => {
                let mut indexed_pair = IndexedPair::new(pair.into_inner());
                let (visibility, id) = match *inner_rules {
                    [JodinRule::visibility, JodinRule::t_struct, JodinRule::identifier, ..] => (
                        Some(indexed_pair.get(JodinRule::visibility).unwrap().as_rule()),
                        indexed_pair.get(JodinRule::identifier).unwrap(),
                    ),
                    [JodinRule::t_struct, JodinRule::identifier, ..] => {
                        (None, indexed_pair.get(JodinRule::identifier).unwrap())
                    }
                    _ => unreachable!(),
                };

                let field_pairs = indexed_pair
                    .get_all(JodinRule::struct_field_declaration)
                    .unwrap();
                let mut fields: Vec<_> = vec![];
                for pair in field_pairs {
                    fields.push(self.generate_node(pair, vec![])?);
                }

                let visibility = Visibility::try_from(visibility)?;
                let mut node: JodinNode = JodinNodeInner::StructureDefinition {
                    name: self.generate_node(id, vec![])?,
                    members: fields,
                }
                .into();
                node.add_tag(VisibilityTag::new(visibility))?;
                node
            }
            JodinRule::struct_field_declaration => {
                let mut inner = pair.into_inner();
                let canonical_type: Pair<_> = inner.next().unwrap();
                let single_id: Pair<_> = inner.next().unwrap();

                JodinNodeInner::NamedValue {
                    name: self.generate_node(single_id, vec![])?,
                    var_type: self.new_intermediate_type(canonical_type)?,
                }
                .into()
            }
            JodinRule::function_definition => {
                let mut inner = pair.into_inner();
                let (visibility, generics) = match *inner_rules {
                    [JodinRule::visibility, JodinRule::generic_declarator, ..] => {
                        let visibility = self.generate_node(inner.next().unwrap(), vec![])?;
                        let generics = self.generate_node(inner.next().unwrap(), vec![])?;
                        (Some(visibility), Some(generics))
                    }
                    [JodinRule::visibility, ..] => {
                        let visibility = self.generate_node(inner.next().unwrap(), vec![])?;
                        (Some(visibility), None)
                    }
                    [JodinRule::generic_declarator, ..] => {
                        let generics = self.generate_node(inner.next().unwrap(), vec![])?;
                        (None, Some(generics))
                    }
                    [..] => (None, None),
                    _ => unreachable!(),
                };

                let name = self.generate_node(inner.next().unwrap(), vec![])?;

                let mut node: JodinNode = JodinNodeInner::FunctionDefinition {
                    name: (),
                    return_type: IntermediateType {},
                    arguments: vec![],
                    generic_parameters: vec![],
                    block: (),
                }
                .into();
                node.add_tag(VisibilityTag::new(Visibility::try_from(visibility)?))?;
                node
            }
            // just go into inner
            JodinRule::top_level_declaration | JodinRule::jodin_file => {
                let inner = pair.into_inner().nth(0).unwrap();
                self.generate_node(inner, vec![])?
            }
            rule => {
                JodinNodeInner::Unimplemented {
                    jodin_rule: rule,
                    affected_string: pair.as_str().to_string(),
                }
                .into()
                //return Err(JodinErrorType::InvalidJodinRuleForASTCreation(rule).into())
            }
        })
    }

    /// Attempts to convert a parse tree into an [IntermediateType](crate::ast::intermediate_type::IntermediateType)
    pub fn new_intermediate_type(
        &mut self,
        pair: Pair<JodinRule>,
    ) -> JodinResult<IntermediateType> {
        let mut vector: Vec<Pair<_>> = pair.into_inner().collect();
        let is_const = vector
            .iter()
            .find(|pair| pair.as_rule() == JodinRule::t_const)
            .is_some();
        if is_const {
            vector.remove(0);
        }
        let type_specifier_pos = vector
            .iter()
            .position(|ty| ty.as_rule() == JodinRule::type_specifier)
            .unwrap();
        let type_specifier_pair = vector.remove(type_specifier_pos);
        let mut inner = type_specifier_pair.into_inner();
        let mut first = inner.nth(0).unwrap();
        let mut unsigned = false;
        if first.as_rule() == JodinRule::t_unsigned {
            unsigned = true;
            first = inner.nth(0).unwrap();
        }
        let type_specifier = match first.as_rule() {
            JodinRule::identifier => {
                let id = parse_identifier(first).unwrap();
                TypeSpecifier::Id(id)
            }
            JodinRule::t_boolean => TypeSpecifier::Primitive(Primitive::Boolean),
            JodinRule::t_double => TypeSpecifier::Primitive(Primitive::Double),
            JodinRule::t_float => TypeSpecifier::Primitive(Primitive::Float),
            JodinRule::t_void => TypeSpecifier::Primitive(Primitive::Void),
            JodinRule::t_char => match unsigned {
                true => TypeSpecifier::Primitive(Primitive::UnsignedByte),
                false => TypeSpecifier::Primitive(Primitive::Byte),
            },
            JodinRule::t_short => match unsigned {
                true => TypeSpecifier::Primitive(Primitive::UnsignedShort),
                false => TypeSpecifier::Primitive(Primitive::Short),
            },
            JodinRule::t_int => match unsigned {
                true => TypeSpecifier::Primitive(Primitive::UnsignedInt),
                false => TypeSpecifier::Primitive(Primitive::Int),
            },
            JodinRule::t_long => match unsigned {
                true => TypeSpecifier::Primitive(Primitive::UnsignedLong),
                false => TypeSpecifier::Primitive(Primitive::Long),
            },
            _ => unreachable!(),
        };

        let mut generics = vec![];
        if let Some(JodinRule::type_list) = vector.first().map(|p| p.as_rule()) {
            let generics_pair = vector.remove(0);
            for pair in generics_pair.into_inner() {
                generics.push(self.new_intermediate_type(pair)?);
            }
        }

        let mut tails = vec![];
        for tail in vector {
            let tail: TypeTail = match tail.as_rule() {
                JodinRule::function_declarator => {
                    let inner = tail.into_inner().nth(0);
                    //let mut inner = tail.into_inner();
                    let mut inner_types = vec![];
                    if let Some(inner) = inner {
                        for pair in inner.into_inner() {
                            inner_types.push(self.new_intermediate_type(pair)?)
                        }
                    }
                    TypeTail::Function(inner_types)
                }
                JodinRule::pointer => TypeTail::Pointer,
                JodinRule::array_declarator => {
                    let inner = tail.into_inner();
                    if let Some(expression) = inner
                        .into_iter()
                        .find(|pair| pair.as_rule() == JodinRule::expression)
                    {
                        let exp = self.generate_node(expression, vec![])?;
                        TypeTail::Array(Some(exp))
                    } else {
                        TypeTail::Array(None)
                    }
                }
                error => panic!("invalid rule: {:?}", error),
            };
            tails.push(tail);
        }

        Ok(IntermediateType::new(
            is_const,
            type_specifier,
            generics,
            tails,
        ))
    }
}

impl<'a> JodinFallibleTool for JodinNodeGenerator<'a> {
    type Input = Pair<'a, JodinRule>;
    type Output = JodinNode;

    fn invoke(&mut self, input: Self::Input) -> JodinResult<Self::Output> {
        let mut ret = self.generate_node(input, vec![]);
        if let Err(JodinErrorType::ParserError(_, path)) =
            ret.as_mut().map_err(|e| &mut e.error_type)
        {
            *path = Some(self.path.clone())
        }
        ret
    }
}

/// Stores a mapping from a JodinRule to a list of parse trees that used that rule.
pub struct IndexedPair<'a> {
    map: HashMap<JodinRule, Vec<Pair<'a, JodinRule>>>,
}

impl<'a> IndexedPair<'a> {
    /// Creates an instance of IndexedPair using the Pairs iterator.
    pub fn new(pairs: Pairs<'a, JodinRule>) -> IndexedPair<'a> {
        let mut map: HashMap<_, Vec<Pair<'a, _>>> = HashMap::new();
        for pair in pairs {
            let vector = map.entry(pair.as_rule()).or_default();
            vector.push(pair);
        }
        IndexedPair { map }
    }

    /// Attempts to get a single instance of a Pair from a rule. This removes this parse tree
    /// from the IndexedPair.
    pub fn get(&mut self, rule: JodinRule) -> JodinResult<Pair<'a, JodinRule>> {
        let vec = self.map.remove(&rule).ok_or(JodinError::new(
            JodinErrorType::InvalidJodinRuleForASTCreation(rule),
        ))?;
        if vec.is_empty() {
            return Err(JodinError::new(
                JodinErrorType::InvalidJodinRuleForASTCreation(rule),
            ));
        }

        Ok(vec.into_iter().nth(0).unwrap())
    }

    /// Attempts to get all instances of a Pair from a rule. This removes this parse tree
    /// from the IndexedPair.
    pub fn get_all(&mut self, rule: JodinRule) -> JodinResult<Vec<Pair<'a, JodinRule>>> {
        let vec = self.map.remove(&rule).ok_or(JodinError::new(
            JodinErrorType::InvalidJodinRuleForASTCreation(rule),
        ))?;
        if vec.is_empty() {
            return Err(JodinError::new(
                JodinErrorType::InvalidJodinRuleForASTCreation(rule),
            ));
        }

        Ok(vec)
    }
}

/// Generates a vector of rules from a reference to a pairs instance. This allows for easy pattern
/// matching to see what version of a rule was used.
///
/// # Arguments
///
/// * `pairs`: A reference to a pairs instance.
///
/// returns: Box<\[R], Global> A slice of rules
pub fn as_rules<R: RuleType>(pairs: &Pairs<R>) -> Box<[R]> {
    let pairs = pairs.clone();
    let vec: Vec<_> = pairs.map(|pair| pair.as_rule()).collect();
    vec.into_boxed_slice()
}

/// Generates a vector of rules from a reference to a pair instance. This allows for easy pattern
/// matching to see what version of a rule was used.
///
/// # Arguments
///
/// * `pairs`: A reference to a pairs instance.
///
/// returns: Box<\[R\], Global> A slice of rules
pub fn pair_as_rules<R: RuleType>(pair: &Pair<R>) -> Box<[R]> {
    let pairs = pair.clone();
    let vec: Vec<_> = pairs.into_inner().map(|pair| pair.as_rule()).collect();
    vec.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use crate::parsing::complete_parse;

    use super::*;

    #[test]
    fn create_id() {
        let pairs = complete_parse(JodinRule::identifier, "hello::world").unwrap();
        let result = JodinNodeGenerator::new("".to_string())
            .generate_node(pairs.into_iter().next().unwrap(), vec![])
            .unwrap();
        let inner = result.inner();
        if let JodinNodeInner::Identifier(id) = inner {
            assert_eq!(id, &Identifier::from_iter(&["hello", "world"]));
        } else {
            panic!("Didn't create correct jodin node");
        }
    }
}
