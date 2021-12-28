//! The expression compiler

use crate::compilation::jodin_vm_compiler::asm_block::{AssemblyBlock, InsertAsm};
use crate::compilation::jodin_vm_compiler::VariableUseTracker;
use crate::compilation::{JodinVM, MicroCompiler};
use crate::{jasm, JodinNode, JodinResult};
use jodin_common::ast::JodinNodeType;
use jodin_common::core::operator::Operator;
use jodin_common::core::tags::TagTools;
use jodin_common::error::JodinErrorType;
use jodin_common::mvp::bytecode::{Asm, Assembly};
use jodin_common::mvp::location::AsmLocation;
use jodin_common::mvp::value::Value;
use jodin_rs_vm::function_names::CALL;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ExpressionCompiler(Rc<RefCell<VariableUseTracker>>);

impl ExpressionCompiler {
    pub fn new(tracker: &Rc<RefCell<VariableUseTracker>>) -> Self {
        ExpressionCompiler(tracker.clone())
    }

    fn expr(&self, tree: &JodinNode) -> JodinResult<AssemblyBlock> {
        let mut output = AssemblyBlock::new(None);
        match tree.r#type() {
            JodinNodeType::Literal(_)
            | JodinNodeType::Identifier(_)
            | JodinNodeType::ConstructorCall { .. }
            | JodinNodeType::RepeatedArrayInitializer { .. }
            | JodinNodeType::ListInitializer { .. } => {
                output.insert_asm(self.atom(tree)?);
            }
            JodinNodeType::Binop { .. } => {
                output.insert_asm(self.binop(tree)?);
            }
            JodinNodeType::Call {
                called,
                generics_instance: _,
                arguments,
            } => {
                // todo: Need to decide if theres a way to call a function without always having to
                // todo: rely on the call method
                let mut arg_count = 0;
                for arg in arguments.iter().rev() {
                    output.insert_asm(self.expr(arg)?);
                    arg_count += 1;
                }

                let called = self.expr(called)?;
                let message = Asm::Push(Value::Str(CALL.to_string()));
                output.insert_asm(message);
                output.insert_asm(called);
                output.insert_asm(Asm::SendMessage);
            }
            e => {
                panic!("Illegal node type given for expr: {:#?}", e)
            }
        }
        if output.len() == 0 {
            panic!("expression did not form any bytecode")
        }
        Ok(output)
    }

    fn atom(&self, tree: &JodinNode) -> JodinResult<AssemblyBlock> {
        match tree.r#type() {
            JodinNodeType::Literal(l) => Ok(AssemblyBlock::from(Asm::Push(l.into()))),
            JodinNodeType::Identifier(..) => {
                let id = tree.resolved_id()?;
                let var = self.0.borrow().get_id(id);
                match var {
                    None => Ok(AssemblyBlock::from(Asm::GetSymbol(
                        id.os_compat().unwrap().to_str().unwrap().to_string(),
                    ))),
                    Some(v) => Ok(AssemblyBlock::from(Asm::GetVar(v as u64))),
                }
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
        match tree.r#type() {
            JodinNodeType::Binop { op, lhs, rhs } => {
                let mut output = AssemblyBlock::new(None);
                let left = self.expr(lhs)?;
                let right = self.expr(rhs)?;

                match op {
                    Operator::Plus => {
                        output.insert_asm(right);
                        output.insert_asm(left);
                        output.insert_asm(Asm::Add)
                    }
                    Operator::Minus => {
                        output.insert_asm(right);
                        output.insert_asm(left);
                        output.insert_asm(Asm::Subtract)
                    }
                    Operator::Star => {
                        output.insert_asm(right);
                        output.insert_asm(left);
                        output.insert_asm(Asm::Multiply)
                    }
                    Operator::Modulo => {
                        output.insert_asm(right);
                        output.insert_asm(left);
                        output.insert_asm(Asm::Remainder)
                    }
                    Operator::Divide => {
                        output.insert_asm(right);
                        output.insert_asm(left);
                        output.insert_asm(Asm::Divide)
                    }
                    Operator::Xor => {
                        todo!("XOR not implemented")
                    }
                    Operator::Dand | Operator::And => {
                        output.insert_asm(right);
                        output.insert_asm(left);
                        output.insert_asm(Asm::And)
                    }
                    Operator::Dor | Operator::Or => {
                        output.insert_asm(right);
                        output.insert_asm(left);
                        output.insert_asm(Asm::Or)
                    }
                    Operator::Equal => {
                        // (a - b) == 0 ? !bool(a - b)
                        output.insert_asm(right);
                        output.insert_asm(left);
                        output.insert_asm(jasm![Asm::Subtract, Asm::Boolify, Asm::Not])
                    }
                    Operator::Nequal => output.insert_asm(jasm![Asm::Subtract, Asm::Boolify]),
                    Operator::Lt => {
                        /*
                        l < r
                        0 < r - l

                         */
                        output.insert_asm(jasm![left, right, Asm::Subtract, Asm::GT0])
                    }
                    Operator::Lte => {
                        /*
                        l <= r
                        !(l > r)
                         */
                        output.insert_asm(jasm![right, left, Asm::Subtract, Asm::GT0, Asm::Not])
                    }
                    Operator::Gt => output.insert_asm(jasm![right, left, Asm::Subtract, Asm::GT0]),
                    Operator::Gte => {
                        output.insert_asm(jasm![left, right, Asm::Subtract, Asm::GT0, Asm::Not])
                    }
                    Operator::LShift => {
                        todo!("<< not implemented")
                    }
                    Operator::RShift => {
                        todo!(">> not implemented")
                    }
                    _ => {
                        return Err(JodinErrorType::InvalidTreeTypeGivenToCompiler(
                            "bi-operator".to_string(),
                        )
                        .into())
                    }
                }

                Ok(output)
            }
            _ => unreachable!(),
        }
    }
}

impl MicroCompiler<JodinVM, AssemblyBlock> for ExpressionCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<AssemblyBlock> {
        self.expr(tree)
    }
}