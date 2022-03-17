use crate::safe_api::error::CompilationError;
use crate::safe_api::{JodinCompilable, JodinCompilableUtilities};
use jodin_common::ast::{JodinNode, JodinNodeType};
use jodin_common::core::literal::Literal;
use jodin_common::core::operator::Operator;
use jodin_common::core::tags::TagTools;
use jodin_common::identifier::Identifier;
use jodin_common::utility::IntoBox;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Lit(Literal),
    Var(Identifier),
    Binop {
        operator: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Uniop {
        operator: Operator,
        expr: Box<Expr>,
    },
    Dereference(Box<Expr>),
    GetReference(Box<Expr>),
    Call {
        expr: Box<Expr>,
        params: Vec<Expr>,
    },
    Index {
        expr: Box<Expr>,
        index: Box<Expr>,
    },
    GetMember {
        expr: Box<Expr>,
        member: Identifier,
    },
}

impl JodinCompilable<Expr> for JodinNode {
    type Error = CompilationError;

    fn compile_to(self) -> Result<Expr, Self::Error> {
        let (inner, tags, index) = self.deconstruct();
        match inner {
            JodinNodeType::Literal(l) => Ok(Expr::Lit(l)),
            JodinNodeType::Identifier(i) => Ok(Expr::Var(i)),
            JodinNodeType::Uniop { op, inner } => Ok(Expr::Uniop {
                operator: op,
                expr: inner.compile_boxed()?,
            }),
            JodinNodeType::CastExpression { .. } => {
                unimplemented!()
            }
            JodinNodeType::Postop { .. } => {
                unimplemented!()
            }
            JodinNodeType::Binop { op, lhs, rhs } => Ok(Expr::Binop {
                operator: op,
                left: lhs.compile_boxed()?,
                right: rhs.compile_boxed()?,
            }),
            JodinNodeType::Ternary { .. } => {
                unimplemented!()
            }
            JodinNodeType::Index {
                indexed,
                expression,
            } => Ok(Expr::Index {
                expr: indexed.compile_boxed()?,
                index: expression.compile_boxed()?,
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

                let type_constructor: Box<Expr> = if generics.is_empty() {
                    called.compile_boxed()?
                } else {
                    Expr::Call {
                        expr: called.compile_boxed()?,
                        params: generics,
                    }
                    .boxed()
                };

                let mut c_arguments = vec![];
                for arg in arguments {
                    c_arguments.push(arg.compile()?);
                }
                Ok(Expr::Call {
                    expr: type_constructor,
                    params: c_arguments,
                })
            }
            JodinNodeType::GetMember { compound, id } => Ok(Expr::GetMember {
                expr: compound.compile_boxed()?,
                member: id.resolved_id()?.clone(),
            }),
            JodinNodeType::ConstructorCall { .. } => {
                unimplemented!()
            }
            JodinNodeType::Dereference { node } => Ok(Expr::Dereference(node.compile_boxed()?)),
            JodinNodeType::GetReference { node } => Ok(Expr::GetReference(node.compile_boxed()?)),
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
            inner => Err(CompilationError::InvalidTreeGiven(JodinNode::reconstruct(
                inner, tags, index,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jodin_common::parsing::parse_expression;

    #[test]
    fn create_op() {
        let tree = parse_expression("3 + 2*5").expect("is parsable");
        let expr_unit: Expr = tree
            .compile()
            .expect("Couldn't compile into an expression unit");
        assert_eq!(
            expr_unit,
            Expr::Binop {
                operator: Operator::Plus,
                left: Expr::Lit(Literal::Int(3)).boxed(),
                right: Expr::Binop {
                    operator: Operator::Star,
                    left: Expr::Lit(Literal::Int(2)).boxed(),
                    right: Expr::Lit(Literal::Int(5)).boxed()
                }
                .boxed()
            },
            "Parsed incorrectly, or parse tree in different order than expected."
        )
    }
}
