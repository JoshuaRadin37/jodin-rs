//! The statement compiler

use crate::compilation::jodin_vm_compiler::asm_block::{rel_label, AssemblyBlock, InsertAsm};
use crate::compilation::jodin_vm_compiler::expression_compiler::ExpressionCompiler;
use crate::compilation::jodin_vm_compiler::{JodinVMCompiler, VariableUseTracker};
use crate::compilation::JodinVM;
use crate::{jasm, JodinError, JodinNode, JodinResult};
use jodin_common::ast::JodinNodeType;
use jodin_common::compilation::MicroCompiler;
use jodin_common::error::JodinErrorType;
use jodin_common::mvp::bytecode::Asm;
use jodin_common::mvp::value::Value;
use jodin_common::parsing::Tok::As;
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
        let mut expr_c = ExpressionCompiler::new(&self.tracker);

        let (cond, block, else_block) = {
            if let JodinNodeType::IfStatement {
                cond,
                statement,
                else_statement,
            } = if_tree.r#type()
            {
                (cond, statement, else_statement.as_ref())
            } else {
                return Err(JodinError::new(
                    JodinErrorType::InvalidTreeTypeGivenToCompiler("IfStatement".to_string()),
                ));
            }
        };

        let asm = jasm![if_block:
            expr_c.create_compilable(cond)?,
            Asm::cond_goto(rel_label("__true__")),
            Asm::goto(rel_label("__false__")),
            Asm::label(rel_label("__true__")),
            self.create_compilable(block)?,
            Asm::cond_goto(rel_label("__end_if__")),
            Asm::label(rel_label("__false__")),
            else_block
                .map(|e| self.create_compilable(e))
                .unwrap_or(Ok(AssemblyBlock::from(Asm::Nop)))?,
            Asm::goto(rel_label("__end_if__")),
            Asm::label(rel_label("__end_if__")),
        ];
        Ok(asm)

        // output.insert_before_label(Asm::goto(rel_label("if_end")), rel_label("true_end"));
        // if else_block.is_some() {
        //     output.insert_before_label(Asm::label(rel_label("false_end")), rel_label("if_end"));
        //     output
        //         .insert_before_label(Asm::label(rel_label("false_start")), rel_label("false_end"));
        //
        //     output.insert_before_label(Asm::goto(rel_label("if_end")), rel_label("false_end"));
        // } else {
        // }
        //
        // let mut compiled_expression = expr_c.create_compilable(cond)?;
        // compiled_expression.insert_asm(Asm::Not);
        //
        // if else_block.is_some() {
        //     output.insert_asm(Asm::cond_goto(rel_label("false_start")));
        // } else {
        //     output.insert_asm(Asm::cond_goto(rel_label("if_end")));
        // }
        //
        // if let Some(else_block) = else_block {
        //     let else_block_asm = self.create_compilable(else_block)?;
        //
        //     output.insert_after_label(else_block_asm, rel_label("false_start"));
        // }
        //
        // let block_asm = self.create_compilable(block)?;
        //
        // output.insert_after_label(block_asm, rel_label("true_start"));
        //
        // output.insert_before_label(compiled_expression, rel_label("true_start"));
        //
        // Ok(output)
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
            JodinNodeType::Call { .. } => {
                let mut expr_c = ExpressionCompiler::new(&self.tracker);
                let expr = expr_c.create_compilable(tree)?;
                block.insert_asm(expr);
                block.insert_asm(Asm::Pop); // call's return value should be thrown away
            }
            JodinNodeType::ReturnValue { expression } => {
                match expression {
                    None => block.insert_asm(Asm::Push(Value::Empty)),
                    Some(o) => {
                        let mut expr_c = ExpressionCompiler::new(&self.tracker);
                        let expr = expr_c.create_compilable(o)?;
                        block.insert_asm(expr);
                    }
                };
                block.insert_asm(Asm::Return);
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
