//! The statement compiler

use crate::compilation::jodin_vm_compiler::expression_compiler::ExpressionCompiler;
use crate::compilation::jodin_vm_compiler::VariableUseTracker;
use crate::compilation::JodinVM;
use crate::{JodinError, JodinNode, JodinResult};
use jodin_common::assembly::asm_block::{rel_label, AssemblyBlock, InsertAsm};
use jodin_common::assembly::instructions::Asm;
use jodin_common::assembly::value::Value;
use jodin_common::ast::JodinNodeType;
use jodin_common::compilation::MicroCompiler;
use jodin_common::error::JodinErrorType;

use jasm_macros::cond;
use jodin_common::block;
use std::cell::RefCell;
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

        let asm = match else_block {
            None => {
                cond! {
                    if (expr_c.create_compilable(cond)?) {
                        self.create_compilable(block)?
                    }
                }
            }
            Some(r#else) => {
                cond! {
                    if (expr_c.create_compilable(cond)?) {
                        self.create_compilable(block)?
                    } else {
                        self.create_compilable(r#else)?
                    }
                }
            }
        };
        Ok(asm)
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
