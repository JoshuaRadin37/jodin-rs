//! The expression compiler

use crate::ast::JodinNodeType;
use crate::compilation::jodin_vm_compiler::asm_block::AssemblyBlock;
use crate::compilation::jodin_vm_compiler::VariableUseTracker;
use crate::compilation::{JodinVM, MicroCompiler};
use crate::{JodinNode, JodinResult};
use jodin_asm::mvp::bytecode::{Asm, Assembly};
use jodin_asm::mvp::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ExpressionCompiler(Rc<RefCell<VariableUseTracker>>);

impl ExpressionCompiler {
    pub fn new(tracker: &Rc<RefCell<VariableUseTracker>>) -> Self {
        ExpressionCompiler(tracker.clone())
    }

    fn atom(&self, tree: &JodinNode) -> JodinResult<AssemblyBlock> {
        match tree.r#type() {
            JodinNodeType::Literal(l) => Ok(AssemblyBlock::from(Asm::Push(l.into()))),
            JodinNodeType::Identifier(id) => {
                let var = self
                    .0
                    .borrow()
                    .get_id(id)
                    .expect(format!("id {:?} not set", id).as_str());
                Ok(AssemblyBlock::from(Asm::GetVar(var as u64)))
            }
            JodinNodeType::ConstructorCall { .. } => {
                todo!()
            }
            JodinNodeType::RepeatedArrayInitializer { .. } => {
                todo!()
            }
            JodinNodeType::ListInitializer { .. } => {
                todo!()
            }
            _ => unreachable!(),
        }
    }

    fn binop(&self, tree: &JodinNode) -> JodinResult<AssemblyBlock> {
        todo!()
    }
}

impl MicroCompiler<JodinVM, AssemblyBlock> for ExpressionCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<AssemblyBlock> {
        let mut output = AssemblyBlock::new(None);

        Ok(output)
    }
}
