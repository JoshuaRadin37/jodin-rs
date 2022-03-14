use crate::safe_api::error::CompilationError;
use crate::safe_api::JodinCompilable;
use jodin_common::ast::{JodinNode, JodinNodeType};
use jodin_common::core::literal::Literal;
use jodin_common::core::operator::Operator;
use jodin_common::core::tags::TagTools;
use jodin_common::identifier::Identifier;
use jodin_common::utility::IntoBox;

pub enum ExpressionUnit {
    Lit(Literal),
    Var(Identifier),
    Binop {
        operator: Operator,
        left: Box<ExpressionUnit>,
        right: Box<ExpressionUnit>,
    },
    Uniop {
        operator: Operator,
        expr: Box<ExpressionUnit>,
    },
    Dereference(Box<ExpressionUnit>),
    GetReference(Box<ExpressionUnit>),
    Call {
        expr: Box<ExpressionUnit>,
        params: Vec<ExpressionUnit>,
    },
    Index {
        expr: Box<ExpressionUnit>,
        index: Box<ExpressionUnit>,
    },
    GetMember {
        expr: Box<ExpressionUnit>,
        member: Identifier,
    },
}

impl JodinCompilable<ExpressionUnit> for JodinNode {
    type Error = CompilationError;

    fn compile(self) -> Result<ExpressionUnit, Self::Error> {
        match self.into_inner() {
            JodinNodeType::Literal(l) => Ok(ExpressionUnit::Lit(l)),
            JodinNodeType::Identifier(i) => Ok(ExpressionUnit::Var(i)),
            JodinNodeType::Uniop { op, inner } => Ok(ExpressionUnit::Uniop {
                operator: op,
                expr: inner.compile()?.boxed(),
            }),
            JodinNodeType::CastExpression { .. } => {
                unimplemented!()
            }
            JodinNodeType::Postop { .. } => {
                unimplemented!()
            }
            JodinNodeType::Binop { op, lhs, rhs } => Ok(ExpressionUnit::Binop {
                operator: op,
                left: lhs.compile()?.boxed(),
                right: rhs.compile()?.boxed(),
            }),
            JodinNodeType::Ternary { .. } => {
                unimplemented!()
            }
            JodinNodeType::Index {
                indexed,
                expression,
            } => Ok(ExpressionUnit::Index {
                expr: indexed.compile()?.boxed(),
                index: expression.compile()?.boxed(),
            }),
            JodinNodeType::Call {
                called,
                generics_instance,
                arguments,
            } => {
                let mut generics = vec![];
                for generic in generics_instance {
                    generics.push(generic.compile()?);
                }

                let type_constructor: Box<ExpressionUnit> = if generics.is_empty() {
                    called.compile()?.boxed()
                } else {
                    ExpressionUnit::Call {
                        expr: called.compile()?.boxed(),
                        params: generics,
                    }
                    .boxed();
                };

                let mut c_arguments = vec![];
                for arg in arguments {
                    c_arguments.push(arg.compile()?);
                }
                Ok(ExpressionUnit::Call {
                    expr: type_constructor,
                    params: c_arguments,
                })
            }
            JodinNodeType::GetMember { compound, id } => Ok(ExpressionUnit::GetMember {
                expr: compound.compile()?.boxed(),
                member: id.resolved_id()?.clone(),
            }),
            JodinNodeType::ConstructorCall { .. } => {
                unimplemented!()
            }
            JodinNodeType::Dereference { node } => {
                Ok(ExpressionUnit::Dereference(node.compile()?.boxed()))
            }
            JodinNodeType::GetReference { node } => {
                Ok(ExpressionUnit::GetReference(node.compile()?.boxed()))
            }
            JodinNodeType::StructInitializer { .. } => {
                unimplemented!()
            }
            JodinNodeType::NewPointer { .. } => {
                unimplemented!()
            }
            JodinNodeType::RepeatedArrayInitializer { .. } => {
                unimplemented!()
            }
            JodinNodeType::ListInitializer { .. } => {
                unimplemented!()
            }
            _ => Err(CompilationError::InvalidTreeGiven(self)),
        }
    }
}
