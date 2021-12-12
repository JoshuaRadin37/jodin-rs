#![deny(rustdoc::broken_intra_doc_links)]
#![deny(unused_imports)]
#![warn(
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

//! The virtual machine for jodin-rs language.
//!
//! The virtual machine will be stack-based.

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate jodin_asm_derive;

use crate::core_traits::{ArithmeticsTrait, GetAsm, MemoryTrait, VirtualMachine};
use crate::function_names::{CALL, RECEIVE_MESSAGE};
use jodin_asm::mvp::bytecode::{Asm, Assembly, Decode};
use jodin_asm::mvp::location::AsmLocation;
use jodin_asm::mvp::value::Value;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use std::io::{Read, Write};

pub mod core_traits;
pub mod function_names;

pub struct VM<M, A>
where
    M: MemoryTrait,
    A: ArithmeticsTrait,
{
    memory: M,
    alu: A,
    cont: bool,

    instructions: Assembly,
    label_to_instruction: HashMap<String, usize>,
    counter_stack: Vec<usize>,

    stdin: Box<dyn Read>,
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,

    next_anonymous_function: AtomicU64,
}

impl<M, A> VM<M, A>
where
    M: MemoryTrait,
    A: ArithmeticsTrait,
{
    fn native_method(&mut self, message: &str, mut args: Vec<Value>) {
        match message {
            "print" => {
                if let Value::Str(s) = args.remove(0) {
                    write!(self.stdout, "{}", s).expect("Couldn't print to output");
                } else {
                    panic!("Can not only pass strings to the print function")
                }
            }
            "write" => {
                let fd = if let Value::UInteger(fd) = args.remove(0) {
                    fd
                } else {
                    panic!("File descriptors should only be unsigned ints")
                };
                let output = match fd {
                    1 => &mut self.stdout,
                    2 => &mut self.stderr,
                    _ => {
                        panic!("{} is not a valid file descriptor for writing", fd);
                    }
                };
                if let Value::Str(s) = args.remove(0) {
                    write!(output, "{}", s).expect("Couldn't write");
                } else {
                    panic!("Can not only pass strings to the print function")
                }
            }
            "invoke" => {
                // invokes the message (arg 2) on the target (arg 1) with args (arg 3..)
                let mut target = args.remove(0);
                let msg = args
                    .remove(1)
                    .into_string()
                    .expect("String expected for message");
                self.send_message(&mut target, &msg, args);
            }
            _ => panic!("{:?} is not a native method", message),
        }
    }

    fn send_message(&mut self, target: &mut Value, message: &str, mut args: Vec<Value>) {
        match target {
            Value::Empty => {}
            Value::Byte(_) => {}
            Value::Float(_) => {}
            Value::Integer(_) => {}
            Value::UInteger(_) => {}
            Value::Str(_) => {}
            Value::Dictionary { dict } => {
                if let Some(mut receive_msg) = dict.get(RECEIVE_MESSAGE).cloned() {
                    if receive_msg != Value::Native {
                        self.send_message(&mut receive_msg, message, args);
                        return;
                    }
                }

                let ret = match message {
                    "get" => {
                        let name = args
                            .remove(0)
                            .into_string()
                            .expect("first value should be a string");
                        dict.get(&*name)
                            .expect(&*format!("{} not in dictionary", name))
                            .clone()
                    }
                    "put" => {
                        let name = args
                            .remove(0)
                            .into_string()
                            .expect("first value should be a string");
                        let value = args.remove(0);
                        let mut next = dict;
                        next.insert(name, value);
                        Value::Dictionary { dict: next }
                    }
                    "contains" => {
                        todo!()
                    }
                    "remove" => {
                        todo!()
                    }
                    "len" => {
                        todo!()
                    }
                    m => panic!("{:?} is not a valid message for dictionary", m),
                };
                self.memory.push(ret);
            }
            Value::Array(_) => {}
            Value::Reference(_) => {}
            Value::Bytecode(bytecode) => {
                if message != CALL {
                    panic!("Can only call bytecode objects")
                }
                let mut decoded = bytecode.decode();
                let name = self.anonymous_function_label();
                let label = Asm::Label(name.clone());
                decoded.insert(0, label);
                self.load(decoded);

                self.memory.push_scope();
                let value = Value::Function(AsmLocation::Label(name.clone()));
                self.memory.save_current_scope(&name);
                self.memory.pop_scope();

                self.send_message(value, CALL, args);
            }
            Value::Function(f) => {
                if message != CALL {
                    panic!("Can only call function objects")
                }
                self.call(f, args);
            }
            Value::Native => {
                self.native_method(message, args);
            }
        }
    }

    fn program_counter(&self) -> usize {
        *self.counter_stack.last().unwrap()
    }

    fn call(&mut self, asm_location: AsmLocation, mut args: Vec<Value>) {
        let name = match &asm_location {
            AsmLocation::ByteIndex(i) => {
                let ref instruction = self.instructions[*i];
                if let Asm::Label(name) = instruction {
                    name.clone()
                } else {
                    panic!("Functions must either be called with a label or start with a label")
                }
            }
            AsmLocation::InstructionDiff(_) => {
                panic!("Illegal for calling functions")
            }
            AsmLocation::Label(l) => l.to_string(),
        };
        self.memory.load_scope(name);
        args.reverse();
        for arg in args {
            self.memory.push(arg);
        }
        let next_pc = match asm_location {
            AsmLocation::ByteIndex(i) => i,
            AsmLocation::InstructionDiff(_) => {
                panic!("Illegal for calling functions")
            }
            AsmLocation::Label(l) => self.label_to_instruction[&l],
        };
        self.counter_stack.push(next_pc);
    }

    fn anonymous_function_label(&self) -> String {
        let num = self.next_anonymous_function.fetch_add(1, Ordering::Relaxed);
        format!("<anonymous function {}>", num)
    }
}

impl<M, A> VirtualMachine for VM<M, A>
where
    M: MemoryTrait,
    A: ArithmeticsTrait,
{
    fn interpret_instruction(&mut self, bytecode: Asm) -> Result<(), ()> {
        match bytecode {
            Asm::Label(_) | Asm::Nop => {}
            Asm::Halt => {
                self.cont = false;
            }
            Asm::Goto(_) => {}
            Asm::CondGoto(_) => {}
            Asm::Push(_) => {}
            Asm::Clear => {}
            Asm::NextVar(_) => {}
            Asm::SetVar(_) => {}
            Asm::GetVar(_) => {}
            Asm::ClearVar(_) => {}
            Asm::GetAttribute(_) => {}
            Asm::Index(_) => {}
            Asm::Return => {}
            Asm::Call(_) => {}
            Asm::Add => {}
            Asm::Subtract => {}
            Asm::Multiply => {}
            Asm::Divide => {}
            Asm::Remainder => {}
            Asm::And => {}
            Asm::Not => {}
            Asm::Or => {}
            Asm::SendMessage => {
                let target = self
                    .memory
                    .pop()
                    .expect("There should be a target value on the stack");
                let message = if let Some(Value::Str(msg)) = self.memory.pop() {
                    msg
                } else {
                    panic!("Message must exist and must be of type String")
                };
                let args = if let Some(Value::Array(args)) = self.memory.pop() {
                    args
                } else {
                    panic!("Arguments must be an array of values")
                };
                self.send_message(target, &*message, args);
            }
            _ => panic!("Invalid instruction"),
        }
        Ok(())
    }

    fn enclosed(&mut self, asm: &Assembly) -> Value {
        todo!()
    }

    fn load<Asm: GetAsm>(&mut self, asm: Asm) {
        todo!()
    }

    fn run(&mut self, start_label: &str) -> Result<u32, ()> {
        todo!()
    }
}
