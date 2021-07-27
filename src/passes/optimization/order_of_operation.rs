use crate::ast::{JodinNode, JodinNodeInner};

/// Helps reorder operators to match the precedence level of operators.
pub struct OrderOfOperationTool;

impl OrderOfOperationTool {
    /// Attempts to fix the order of operations within a node
    pub fn fix_order_of_operations(&self, input: &mut JodinNode) {
        for child in input.inner_mut().children_mut() {
            self.fix_order_of_operations(child);
        }

        let mut is_candidate = false;

        match input.inner() {
            JodinNodeInner::Binop {
                op: op1,
                lhs: _,
                rhs,
            } => match rhs.inner() {
                JodinNodeInner::Binop { op: op2, .. } => {
                    if op1.as_precedence() == op2.as_precedence() {
                        is_candidate = true;
                    }
                }
                _ => {}
            },

            _ => {}
        }

        if is_candidate {
            let to_fix = std::mem::replace(input, JodinNode::empty());
            if let JodinNodeInner::Binop { op: op1, lhs, rhs } = to_fix.into_inner() {
                if let JodinNodeInner::Binop {
                    op: op2,
                    lhs: rlhs,
                    rhs: rrhs,
                } = rhs.into_inner()
                {
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

                    *input = new_rhs;
                }
            }
        }
    }
}
