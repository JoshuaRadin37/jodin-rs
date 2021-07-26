use crate::parsing::ast::JodinNode;

use crate::passes::toolchain::Tool;

pub struct OrderOfOperationTool;

impl Tool for OrderOfOperationTool {
    type Input = JodinNode;
    type Output = JodinNode;

    fn invoke(&mut self, _input: Self::Input) -> Self::Output {
        unimplemented!()
    }
}
