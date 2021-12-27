//! The expression compiler

use crate::compilation::jodin_vm_compiler::asm_block::AssemblyBlock;
use crate::compilation::jodin_vm_compiler::VariableUseTracker;
use crate::compilation::{JodinVM, MicroCompiler};
use crate::{JodinNode, JodinResult};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ExpressionCompiler(Rc<RefCell<VariableUseTracker>>);

impl ExpressionCompiler {
    pub fn new(tracker: &Rc<RefCell<VariableUseTracker>>) -> Self {
        ExpressionCompiler(tracker.clone())
    }
}

impl MicroCompiler<JodinVM, AssemblyBlock> for ExpressionCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<AssemblyBlock> {
        let mut output = AssemblyBlock::new(None);

        Ok(output)
    }
}
