use crate::core::identifier::Identifier;
use crate::core::literal::Literal;
use crate::core::types::{JodinType, JodinTypeReference};
use crate::parsing::ast::jodin_node::JodinNode;
use crate::parsing::keywords::Keyword;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum ResolvedType {
    Type(JodinTypeReference),
    Unresolved(Identifier),
}

#[derive(Debug)]
pub enum JodinNodeInner {
    Type(JodinTypeReference),
    Keyword(Keyword),
    Literal(Literal),
    Identifier(Identifier),
    VarDeclaration {
        name: JodinNode,
        var_type: JodinNode,
        value: JodinNode,
    },
    Function {
        name: JodinNode,
        return_type: JodinNode,
        parameters: Vec<JodinNode>,
        generic_parameters: Vec<JodinNode>,
        block: JodinNode,
    },
    Block {
        expressions: Vec<JodinNode>,
    },
    StructureDefinition {
        name: Option<JodinNode>,
        members: Vec<JodinNode>,
    },
    NamedValue {
        name: JodinNode,
        var_type: JodinType,
    },
}
