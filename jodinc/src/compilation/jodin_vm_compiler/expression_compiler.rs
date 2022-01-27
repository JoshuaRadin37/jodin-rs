//! The expression compiler

use crate::compilation::jodin_vm_compiler::VariableUseTracker;
use crate::compilation::JodinVM;
use crate::{JodinNode, JodinResult};
use jodin_common::assembly::asm_block::{AssemblyBlock, InsertAsm};
use jodin_common::ast::JodinNodeType;
use jodin_common::compilation::MicroCompiler;
use jodin_common::core::operator::Operator;
use jodin_common::core::tags::TagTools;
use jodin_common::core::NATIVE_OBJECT;
use jodin_common::error::JodinErrorType;

use jodin_common::assembly::instructions::Asm;

use jasm_macros::expr;
use jodin_common::assembly::value::Value;
use jodin_common::block;
use jodin_rs_vm::function_names::CALL;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
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
                if let JodinNodeType::Identifier(id) = called.r#type() {
                    if id == NATIVE_OBJECT {
                        let arguments = arguments.as_slice();
                        let message = self.expr(&arguments[0])?;
                        let args = &arguments[1..];
                        let mut arg_count = 0;
                        for arg in args.iter().rev() {
                            output.insert_asm(block![self.expr(arg)?,]);
                            arg_count += 1;
                        }
                        output.insert_asm(block![
                            Asm::Pack(arg_count),
                            message,
                            Asm::Push(Value::Native),
                            Asm::Pack(3),
                            Asm::Push(Value::Str("invoke".to_string())),
                            Asm::Push(Value::Native),
                            Asm::SendMessage
                        ]);
                        return Ok(output);
                    }
                }
                // todo: Need to decide if theres a way to call a function without always having to
                // todo: rely on the call method
                let mut arg_count = 0;

                for arg in arguments.iter().rev() {
                    output.insert_asm(block![self.expr(arg)?,]);
                    arg_count += 1;
                }

                let called_asm = self.expr(called)?;
                output.insert_asm(Asm::Pack(arg_count));

                let message = Asm::Push(Value::Str(CALL.to_string()));
                output.insert_asm(message);
                output.insert_asm(called_asm);
                output.insert_asm(Asm::SendMessage);
            }
            e => {
                panic!("Illegal node type given for expr: {:#?}", e)
            }
        }

        if let Some(Asm::GetVar(_)) = output.normalize().last() {
            output.insert_asm(Asm::Deref);
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
                        output.insert_asm(expr![==, left, right])
                    }
                    Operator::Nequal => output.insert_asm(expr![!=, left, right]),
                    Operator::Lt => {
                        /*
                        l < r
                        0 < r - l

                         */
                        output.insert_asm(expr![<, left, right])
                    }
                    Operator::Lte => {
                        /*
                        l <= r
                        !(l > r)
                         */
                        output.insert_asm(expr![<=, left, right])
                    }
                    Operator::Gt => output.insert_asm(expr![>, left, right]),
                    Operator::Gte => output.insert_asm(expr![>=, left, right]),
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
