use crate::parsing::ast::jodin_node::JodinNode;
use crate::parsing::ast::node_type::JodinNodeInner;
use crate::passes::toolchain::Tool;

pub struct OrderOfOperationTool;

impl Tool for OrderOfOperationTool {
    type Input = JodinNode;
    type Output = JodinNode;

    fn invoke(&mut self, input: Self::Input) -> Self::Output {
        unimplemented!()
    }
}
