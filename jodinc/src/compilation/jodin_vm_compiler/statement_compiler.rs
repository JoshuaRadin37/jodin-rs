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

use jasm_macros::{cond, if_, value, var, while_};
use jodin_common::block;
use jodin_common::core::tags::TagTools;
use jodin_common::types::StorageModifier;
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
                if_! {
                    (expr_c.create_compilable(cond)?) {
                        self.create_compilable(block)?
                    }
                }
            }
            Some(r#else) => {
                if_! {
                    (expr_c.create_compilable(cond)?) {
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
            JodinNodeType::StoreVariable {
                storage_type: StorageModifier::Local,
                name,
                var_type: _,
                maybe_initial_value,
            } => {
                let mut tracker = self.tracker.borrow_mut();
                let id = name.resolved_id()?;
                let var = tracker.next_var(id) as u64;
                drop(tracker);
                let value = match maybe_initial_value {
                    None => {
                        block!(value!(()))
                    }
                    Some(val) => {
                        let mut expr_c = ExpressionCompiler::new(&self.tracker);
                        expr_c.create_compilable(val)?
                    }
                };
                block.insert_asm(var!(var => value));
            }
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
            JodinNodeType::WhileStatement { cond, statement } => {
                let mut expr_c = ExpressionCompiler::new(&self.tracker);
                let cond = expr_c.create_compilable(cond)?;
                let statement = self.create_compilable(statement)?;
                block.insert_asm(while_! {
                    (cond) { statement }
                })
            }
            _ => {
                // return Err(JodinError::new(
                //     JodinErrorType::InvalidTreeTypeGivenToCompiler("...".to_string()),
                // ))
            }
        }
        Ok(block)
    }
}
