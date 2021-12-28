//! The micro-compiler for functions

use crate::compilation::jodin_vm_compiler::asm_block::{
    rel_label, temp_label, AssemblyBlock, InsertAsm,
};
use crate::compilation::jodin_vm_compiler::statement_compiler::StatementCompiler;
use crate::compilation::jodin_vm_compiler::{invalid_tree_type, VariableUseTracker};
use crate::compilation::{JodinVM, MicroCompiler};
use crate::{JodinError, JodinNode, JodinResult};
use jodin_common::ast::JodinNodeType;
use jodin_common::core::tags::TagTools;
use jodin_common::error::JodinErrorType;
use jodin_common::mvp::bytecode::{Asm, Assembly};
use jodin_common::mvp::location::AsmLocation;
use jodin_common::mvp::value::Value;
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
        Self(as_ref)
    }
}

impl FunctionCompiler {
    pub fn new(tracker: VariableUseTracker) -> Self {
        Self(Rc::new(RefCell::new(tracker)))
    }
}

impl MicroCompiler<JodinVM, AssemblyBlock> for FunctionCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<AssemblyBlock> {
        let mut output = AssemblyBlock::with_id(tree.resolved_id().unwrap());
        output.insert_asm(Asm::PublicLabel(tree.resolved_id().unwrap().to_string()));
        output.insert_asm(temp_label("__func_params__"));
        let (return_type, args, block) = {
            if let JodinNodeType::FunctionDefinition {
                name: _,
                return_type,
                arguments,
                block,
            } = tree.r#type()
            {
                (return_type, arguments, block)
            } else {
                return Err(JodinError::new(invalid_tree_type("FunctionDefinition")));
            }
        };
        let mut args_block = AssemblyBlock::new(None);
        for arg in args.iter() {
            if let JodinNodeType::NamedValue { name, .. } = arg.r#type() {
                let id = name.resolved_id()?;
                let asm = self.0.borrow_mut().next_var_asm(id);
                args_block.insert_asm(asm);
            }
        }
        output.insert_after_label(args_block, temp_label("__func_params__"));
        output.insert_asm(Asm::label(rel_label("__func_start__")));

        let mut statement_compiler = StatementCompiler::from(&self.0);

        let block = statement_compiler.create_compilable(block)?;

        output.insert_after_label(block, rel_label("__func_start__"));

        output.insert_asm(Asm::goto(rel_label("__func_end__")));
        output.insert_asm(Asm::label(rel_label("__func_end__")));

        if return_type.is_void() {
            output.insert_asm(Asm::Push(Value::Empty));
            output.insert_asm(Asm::Return);
        }
        Ok(output)
    }
}
