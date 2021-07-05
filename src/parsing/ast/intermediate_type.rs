use crate::core::error::{JodinError, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::types::primitives::Primitive;
use crate::parsing::ast::jodin_node::JodinNode;
use crate::parsing::ast::{parse_identifier, JodinNodeBuilder};
use crate::parsing::parser::JodinRule;
use crate::passes::toolchain::FallibleTool;
use pest::iterators::Pair;

#[derive(Debug)]
pub struct IntermediateType {
    is_const: bool,
    type_specifier: TypeSpecifier,
    generics: Vec<IntermediateType>,
    tails: Vec<TypeTail>,
}

impl IntermediateType {
    pub fn new(
        is_const: bool,
        type_specifier: TypeSpecifier,
        generics: Vec<IntermediateType>,
        tails: Vec<TypeTail>,
    ) -> Self {
        IntermediateType {
            is_const,
            type_specifier,
            generics,
            tails,
        }
    }
}

impl IntermediateType {
    pub fn is_const(&self) -> bool {
        self.is_const
    }
    pub fn type_specifier(&self) -> &TypeSpecifier {
        &self.type_specifier
    }
    pub fn tails(&self) -> &Vec<TypeTail> {
        &self.tails
    }
}

#[derive(Debug)]
pub enum TypeSpecifier {
    Id(Identifier),
    Primitive(Primitive),
}

#[derive(Debug)]
pub enum TypeTail {
    Pointer,
    Array(Option<JodinNode>),
    Function(Vec<IntermediateType>),
}

impl IntermediateType {}
