//! The micro-compiler for functions

use crate::ast::tags::TagTools;
use crate::compilation::jodin_vm_compiler::asm_block::{
    rel_label, temp_label, AssemblyBlock, InsertAsm,
};
use crate::compilation::jodin_vm_compiler::VariableUseTracker;
use crate::compilation::{JodinVM, MicroCompiler};
use crate::{JodinNode, JodinResult};
use jodin_asm::mvp::bytecode::{Asm, Assembly};
use jodin_asm::mvp::location::AsmLocation;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct FunctionCompiler(Rc<RefCell<VariableUseTracker>>);

impl<T> From<T> for FunctionCompiler
where
    T: AsRef<Rc<RefCell<VariableUseTracker>>>,
{
    fn from(as_ref: T) -> Self {
        let as_ref = as_ref.as_ref().clone();
        FunctionCompiler(as_ref)
    }
}

impl FunctionCompiler {
    pub fn new(tracker: VariableUseTracker) -> Self {
        FunctionCompiler(Rc::new(RefCell::new(tracker)))
    }
}

impl MicroCompiler<JodinVM, AssemblyBlock> for FunctionCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<AssemblyBlock> {
        let mut output = AssemblyBlock::with_id(tree.resolved_id().unwrap());
        output.insert_asm(Asm::label(rel_label("__func_start__")));
        output.insert_asm(Asm::goto(rel_label("__func_end__")));
        output.insert_asm(Asm::label(rel_label("__func_end__")));
        output.insert_asm(Asm::Return);
        Ok(output)
    }
}
