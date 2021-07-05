use crate::core::identifier::Identifier;
use crate::core::import::Import;
use crate::core::literal::Literal;
use crate::core::operator::Operator;
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
    VarDeclarations {
        var_type: JodinNode,
        names: Vec<JodinNode>,
        values: Vec<Option<JodinNode>>,
    },
    FunctionDefinition {
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
        name: JodinNode,
        generic_parameters: Vec<JodinNode>,
        members: Vec<JodinNode>,
    },
    NamedValue {
        name: JodinNode,
        var_type: JodinType,
    },
    Uniop {
        op: Operator,
        inner: JodinNode,
    },
    Postop {
        op: Operator,
        inner: JodinNode,
    },
    Binop {
        op: Operator,
        lhs: JodinNode,
        rhs: JodinNode,
    },
    Index {
        indexed: JodinNode,
        expression: JodinNode,
    },
    Call {
        called: JodinNode,
        generics_instance: Vec<JodinNode>,
        parameters: Vec<JodinNode>,
    },
    GetMember {
        compound: JodinNode,
        id: JodinNode,
    },
    TopLevelDeclarations {
        decs: Vec<JodinNode>,
    },
    InNamespace {
        namespace: JodinNode,
        inner: JodinNode,
    },
    UsingIdentifier {
        import_data: Import,
    },
}

impl JodinNodeInner {
    pub fn into_result<E>(self) -> Result<JodinNode, E> {
        Ok(self.into())
    }
}

impl From<JodinNodeInner> for JodinNode {
    fn from(i: JodinNodeInner) -> Self {
        JodinNode::new(i)
    }
}
