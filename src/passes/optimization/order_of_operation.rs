use crate::ast::{JodinNode, JodinNodeInner};

/// Helps reorder operators to match the precedence level of operators.
pub struct OrderOfOperationTool;

impl OrderOfOperationTool {
    /// Attempts to fix the order of operations within a node
    pub fn fix_order_of_operations(&self, input: &mut JodinNode) {
        let is_candidate = match input.inner() {
            JodinNodeInner::Binop { .. } => true,
            _ => false,
        };

        if is_candidate {
            let to_fix = std::mem::replace(input, JodinNode::empty());
        }

        for child in input.inner_mut().children_mut() {
            self.fix_order_of_operations(child);
        }
    }
}
