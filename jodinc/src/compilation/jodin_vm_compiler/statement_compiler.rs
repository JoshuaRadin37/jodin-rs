//! The statement compiler

use crate::ast::JodinNodeType;
use crate::compilation::jodin_vm_compiler::asm_block::{rel_label, AssemblyBlock, InsertAsm};
use crate::compilation::jodin_vm_compiler::expression_compiler::ExpressionCompiler;
use crate::compilation::jodin_vm_compiler::{JodinVMCompiler, VariableUseTracker};
use crate::compilation::{JodinVM, MicroCompiler};
use crate::core::error::JodinErrorType;
use crate::parsing::Tok::As;
use crate::{JodinError, JodinNode, JodinResult};
use jodin_asm::mvp::bytecode::Asm;
use std::cell::RefCell;
use std::os::macos::raw::stat;
use std::rc::Rc;

#[derive(Default)]
pub struct StatementCompiler {
    tracker: Rc<RefCell<VariableUseTracker>>,
}

impl From<&Rc<RefCell<VariableUseTracker>>> for StatementCompiler {
    fn from(as_ref: &Rc<RefCell<VariableUseTracker>>) -> Self {
        let as_ref = as_ref.clone();
        Self { tracker: as_ref }
    }
}

impl StatementCompiler {
    pub fn new(tracker: VariableUseTracker) -> Self {
        Self {
            tracker: Rc::new(RefCell::new(tracker)),
        }
    }

    pub fn if_statement(&mut self, if_tree: &JodinNode) -> JodinResult<AssemblyBlock> {
        let mut output = AssemblyBlock::new(&"if".to_string());

        output.insert_asm(Asm::label(rel_label("true_start")));
        output.insert_asm(Asm::label(rel_label("true_end")));
        output.insert_asm(Asm::label(rel_label("if_end")));

        let mut expr_c = ExpressionCompiler::new(&self.tracker);

        let (cond, block, else_block) = {
            if let JodinNodeType::IfStatement {
                cond,
                statement,
                else_statement,
            } = if_tree.r#type()
            {
                (cond, statement, else_statement)
            } else {
                return Err(JodinError::new(
                    JodinErrorType::InvalidTreeTypeGivenToCompiler("IfStatement".to_string()),
                ));
            }
        };

        output.insert_before_label(Asm::goto(rel_label("if_end")), rel_label("true_end"));
        if else_block.is_some() {
            output.insert_asm_front(Asm::label(rel_label("false_end")));
            output.insert_asm_front(Asm::label(rel_label("false_start")));

            output.insert_before_label(Asm::goto(rel_label("if_end")), rel_label("false_end"));
        } else {
        }

        let mut compiled_expression = expr_c.create_compilable(cond)?;
        compiled_expression.insert_asm(Asm::Not);

        if else_block.is_some() {
            output.insert_asm(Asm::cond_goto(rel_label("false_start")));
        } else {
            output.insert_asm(Asm::cond_goto(rel_label("if_end")));
        }

        output.insert_before_label(compiled_expression, rel_label("true_start"));

        Ok(output)
    }
}

impl MicroCompiler<JodinVM, AssemblyBlock> for StatementCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<AssemblyBlock> {
        let mut block = AssemblyBlock::new(None);
        match tree.r#type() {
            JodinNodeType::Block { expressions } => {
                for expr in expressions {
                    let asm = self.create_compilable(expr)?;
                    block.insert_asm(asm);
                }
            }
            JodinNodeType::IfStatement { .. } => block.insert_asm(self.if_statement(tree)?),
            _ => {
                // return Err(JodinError::new(
                //     JodinErrorType::InvalidTreeTypeGivenToCompiler("...".to_string()),
                // ))
            }
        }
        Ok(block)
    }
}
