use crate::ast::{JodinNode, JodinNodeInner};

use crate::passes::toolchain::Tool;

/// Helps reorder operators to match the precedence level of operators.
pub struct OrderOfOperationTool;

impl<'a> Tool for OrderOfOperationTool {
    type Input = JodinNode;
    type Output = JodinNode;

    fn invoke(&mut self, input: JodinNode) -> JodinNode {
        match input.into_inner() {
            JodinNodeInner::Binop { op: op1, lhs, rhs } => match rhs.into_inner() {
                JodinNodeInner::Binop {
                    op: op2,
                    lhs: rlhs,
                    rhs: rrhs,
                } => {
                    return if op1.as_precedence() == op2.as_precedence() {
                        let new_lhs: JodinNode = JodinNodeInner::Binop {
                            op: op1,
                            lhs,
                            rhs: rlhs,
                        }
                            .into();
                        let new_rhs: JodinNode = JodinNodeInner::Binop {
                            op: op2,
                            lhs: new_lhs,
                            rhs: rrhs,
                        }
                            .into();

                        new_rhs
                    } else {
                        let new_rhs: JodinNode = JodinNodeInner::Binop {
                            op: op2,
                            lhs: rlhs,
                            rhs: rrhs,
                        }
                            .into();
                        let new_lhs: JodinNode = JodinNodeInner::Binop {
                            op: op1,
                            lhs,
                            rhs: new_rhs,
                        }
                            .into();

                        new_lhs
                    }
                }
                other => JodinNodeInner::Binop {
                    op: op1,
                    lhs,
                    rhs: other.into(),
                }
                .into(),
            },
            other => other.into(),
        }
    }
}