use crate::ast::JodinNode;

use crate::passes::toolchain::Tool;

/// Helps reorder operators to match the precedence level of operators.
pub struct OrderOfOperationTool;

impl Tool for OrderOfOperationTool {
    type Input = JodinNode;
    type Output = JodinNode;

    fn invoke(&mut self, _input: Self::Input) -> Self::Output {
        unimplemented!()
    }
}
