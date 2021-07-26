use crate::core::identifier::Identifier;
use crate::core::import::Import;
use crate::core::literal::Literal;
use crate::core::operator::Operator;

use crate::parsing::ast::intermediate_type::IntermediateType;
use crate::parsing::ast::jodin_node::JodinNode;
use crate::parsing::keywords::Keyword;
use crate::parsing::parser::JodinRule;

#[derive(Debug)]
pub enum JodinNodeInner {
    Type(IntermediateType),
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
        return_type: IntermediateType,
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
        var_type: IntermediateType,
    },
    Uniop {
        op: Operator,
        inner: JodinNode,
    },
    CastExpression {
        to_type: IntermediateType,
        factor: JodinNode,
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
    Ternary {
        cond: JodinNode,
        yes: JodinNode,
        no: JodinNode,
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
    Unimplemented {
        jodin_rule: JodinRule,
        affected_string: String,
    },
}

impl JodinNodeInner {
    pub fn into_result<E>(self) -> Result<JodinNode, E> {
        Ok(self.into())
    }

    pub fn children(&self) -> impl IntoIterator<Item = &JodinNode> {
        let vector: Vec<&JodinNode> = match self {
            JodinNodeInner::Type(_) => {
                vec![]
            }
            JodinNodeInner::Keyword(_) => {
                vec![]
            }
            JodinNodeInner::Literal(_) => {
                vec![]
            }
            JodinNodeInner::Identifier(_) => {
                vec![]
            }
            JodinNodeInner::VarDeclarations {
                var_type,
                names,
                values,
            } => {
                let mut ret = vec![var_type];
                ret.extend(names);
                ret.extend(values.iter().filter_map(|node| node.as_ref()));
                ret
            }
            JodinNodeInner::FunctionDefinition {
                name,
                return_type: _,
                parameters,
                generic_parameters,
                block,
            } => {
                let mut ret = vec![name];
                ret.extend(parameters);
                ret.extend(generic_parameters);
                ret.push(block);
                ret
            }
            JodinNodeInner::Block { expressions } => expressions.into_iter().collect(),
            JodinNodeInner::StructureDefinition {
                name,
                generic_parameters,
                members,
            } => {
                let mut ret = vec![name];
                ret.extend(generic_parameters);
                ret.extend(members);
                ret
            }
            JodinNodeInner::NamedValue { name, var_type: _ } => {
                vec![name]
            }
            JodinNodeInner::Uniop { op: _, inner } => {
                vec![inner]
            }
            JodinNodeInner::CastExpression { to_type: _, factor } => {
                vec![factor]
            }
            JodinNodeInner::Postop { op: _, inner } => {
                vec![inner]
            }
            JodinNodeInner::Binop { op: _, lhs, rhs } => {
                vec![lhs, rhs]
            }
            JodinNodeInner::Ternary { cond, yes, no } => {
                vec![cond, yes, no]
            }
            JodinNodeInner::Index {
                indexed,
                expression,
            } => {
                vec![indexed, expression]
            }
            JodinNodeInner::Call {
                called,
                generics_instance,
                parameters,
            } => {
                let mut ret = vec![called];
                ret.extend(generics_instance);
                ret.extend(parameters);
                ret
            }
            JodinNodeInner::GetMember { compound, id } => {
                vec![compound, id]
            }
            JodinNodeInner::TopLevelDeclarations { decs } => decs.iter().collect(),
            JodinNodeInner::InNamespace { namespace, inner } => {
                vec![namespace, inner]
            }
            JodinNodeInner::UsingIdentifier { .. } => {
                vec![]
            }
            JodinNodeInner::Unimplemented { .. } => {
                vec![]
            }
        };
        vector
    }

    pub fn children_mut(&mut self) -> impl IntoIterator<Item = &mut JodinNode> {
        let vector: Vec<&mut JodinNode> = match self {
            JodinNodeInner::Type(_) => {
                vec![]
            }
            JodinNodeInner::Keyword(_) => {
                vec![]
            }
            JodinNodeInner::Literal(_) => {
                vec![]
            }
            JodinNodeInner::Identifier(_) => {
                vec![]
            }
            JodinNodeInner::VarDeclarations {
                var_type,
                names,
                values,
            } => {
                let mut ret = vec![var_type];
                ret.extend(names);
                ret.extend(values.iter_mut().filter_map(|node| node.as_mut()));
                ret
            }
            JodinNodeInner::FunctionDefinition {
                name,
                return_type: _,
                parameters,
                generic_parameters,
                block,
            } => {
                let mut ret = vec![name];
                ret.extend(parameters);
                ret.extend(generic_parameters);
                ret.push(block);
                ret
            }
            JodinNodeInner::Block { expressions } => expressions.into_iter().collect(),
            JodinNodeInner::StructureDefinition {
                name,
                generic_parameters,
                members,
            } => {
                let mut ret = vec![name];
                ret.extend(generic_parameters);
                ret.extend(members);
                ret
            }
            JodinNodeInner::NamedValue { name, var_type: _ } => {
                vec![name]
            }
            JodinNodeInner::Uniop { op: _, inner } => {
                vec![inner]
            }
            JodinNodeInner::CastExpression { to_type: _, factor } => {
                vec![factor]
            }
            JodinNodeInner::Postop { op: _, inner } => {
                vec![inner]
            }
            JodinNodeInner::Binop { op: _, lhs, rhs } => {
                vec![lhs, rhs]
            }
            JodinNodeInner::Ternary { cond, yes, no } => {
                vec![cond, yes, no]
            }
            JodinNodeInner::Index {
                indexed,
                expression,
            } => {
                vec![indexed, expression]
            }
            JodinNodeInner::Call {
                called,
                generics_instance,
                parameters,
            } => {
                let mut ret = vec![called];
                ret.extend(generics_instance);
                ret.extend(parameters);
                ret
            }
            JodinNodeInner::GetMember { compound, id } => {
                vec![compound, id]
            }
            JodinNodeInner::TopLevelDeclarations { decs } => decs.iter_mut().collect(),
            JodinNodeInner::InNamespace { namespace, inner } => {
                vec![namespace, inner]
            }
            JodinNodeInner::UsingIdentifier { .. } => {
                vec![]
            }
            JodinNodeInner::Unimplemented { .. } => {
                vec![]
            }
        };
        vector
    }
}

impl From<JodinNodeInner> for JodinNode {
    fn from(i: JodinNodeInner) -> Self {
        JodinNode::new(i)
    }
}
