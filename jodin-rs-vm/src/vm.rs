use crate::core_traits::VMLoadable;
use crate::error::VMError;
use crate::fault::{Fault, FaultHandle, FaultJumpTable};
use crate::{ArithmeticsTrait, MemoryTrait, VMTryLoadable, VirtualMachine, CALL, RECEIVE_MESSAGE};
use jodin_common::error::JodinErrorType;
use jodin_common::mvp::bytecode::{Asm, Assembly, Decode, GetAsm};
use jodin_common::mvp::location::AsmLocation;
use jodin_common::mvp::value::Value;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::ffi::OsStr;
use std::io::{stderr, stdin, stdout, Read, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct VM<'l, M, A>
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

    stdin: Box<dyn Read + 'l>,
    stdout: Box<dyn Write + 'l>,
    stderr: Box<dyn Write + 'l>,

    next_anonymous_function: AtomicU64,

    handler: Option<FaultHandle>,

    fault_table: FaultJumpTable,
    kernel_mode: bool,
}

impl<'l, M, A> VM<'l, M, A>
where
    M: MemoryTrait,
    A: ArithmeticsTrait,
{
    pub fn set_stdin<R: Read + 'l>(&mut self, reader: R) {
        self.stdin = Box::new(reader);
    }

    pub fn set_stdout<W: Write + 'l>(&mut self, writer: W) {
        self.stdout = Box::new(writer);
    }

    pub fn set_stderr<W: Write + 'l>(&mut self, writer: W) {
        self.stderr = Box::new(writer);
    }

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
            "ref" => {
                let target = args.remove(0);

                let as_ref = target.into_reference();
                self.memory.push(as_ref);
            }
            "copy" => {
                let target = args.remove(0);
                let cloned = target.clone();
                self.memory.push(target);
                self.memory.push(cloned);
            }
            "@print_stack" => {
                debug!("memory: {:#?}", self.memory);
            }
            _ => panic!("{:?} is not a native method", message),
        }
    }

    fn send_message(
        &mut self,
        target: &mut Value,
        message: &str,
        mut args: Vec<Value>,
    ) -> Option<usize> {
        debug!("Sending {:?} to {:?} with args {:?}", message, target, args);
        match target {
            Value::Empty => {}
            Value::Byte(_) => {}
            Value::Float(_) => {}
            Value::Integer(_) => {}
            Value::UInteger(_) => {}
            Value::Str(_) => {}
            Value::Dictionary(dict) => {
                if let Some(mut receive_msg) = dict.get(RECEIVE_MESSAGE).cloned() {
                    if receive_msg != Value::Native {
                        return self.send_message(&mut receive_msg, CALL, args);
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
                        dict.insert(name, value);
                        Value::Empty
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
            Value::Reference(reference) => {
                let mut as_mut = reference.borrow_mut();
                let as_mut_ref = &mut *as_mut;
                return self.send_message(as_mut_ref, message, args);
            }
            Value::Bytecode(bytecode) => {
                if message != CALL {
                    panic!("Can only call bytecode objects")
                }
                let mut decoded = bytecode.clone().decode();
                let name = self.anonymous_function_label();
                let label = Asm::Label(name.clone());
                decoded.insert(0, label);
                self.load(decoded);

                self.memory.push_scope();
                let mut value = Value::Function(AsmLocation::Label(name.clone()));
                self.memory.save_current_scope(&name);
                self.memory.pop_scope();

                return self.send_message(&mut value, CALL, args);
            }
            Value::Function(f) => {
                if message != CALL {
                    panic!("Can only call function objects")
                }
                return self.call(f, args);
            }
            Value::Native => {
                self.native_method(message, args);
            }
        }
        return None;
    }

    fn program_counter(&self) -> usize {
        self.counter_stack.last().copied().unwrap_or(0)
    }

    fn call(&mut self, asm_location: &AsmLocation, mut args: Vec<Value>) -> Option<usize> {
        debug!(
            "Attempting to call {:?} with args: {:?}",
            asm_location, args
        );
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
            &AsmLocation::ByteIndex(i) => i,
            AsmLocation::InstructionDiff(_) => {
                panic!("Illegal for calling functions")
            }
            AsmLocation::Label(l) => self.label_to_instruction[l],
        };
        debug!("Returning next PC to function at index 0x{:016X}", next_pc);
        self.counter_stack.push(0);
        Some(next_pc)
    }

    fn anonymous_function_label(&self) -> String {
        let num = self.next_anonymous_function.fetch_add(1, Ordering::Relaxed);
        format!("<anonymous function {}>", num)
    }

    fn set_program_counter(&mut self, pc: usize) {
        self.counter_stack.pop();
        self.counter_stack.push(pc);
    }

    pub fn in_fault(&self) -> bool {
        self.handler.is_some()
    }

    fn end_fault(&mut self, handle: FaultHandle) {
        let FaultHandle {
            stored_pc,
            stored_stack,
            fault: _,
            target_function: _,
        } = handle;

        self.counter_stack = stored_pc;
        self.memory.replace_stack(stored_stack);
    }

    fn handle_native_fault(&mut self, handle: &FaultHandle) {}
}

impl<M, A> VirtualMachine for VM<'_, M, A>
where
    M: MemoryTrait,
    A: ArithmeticsTrait,
{
    fn interpret_instruction(
        &mut self,
        bytecode: &Asm,
        instruction_pointer: usize,
    ) -> Result<usize, VMError> {
        let mut next_instruction = instruction_pointer + 1;
        match bytecode {
            Asm::Label(_) | Asm::PublicLabel(_) | Asm::Nop => {}
            Asm::Return => {
                self.counter_stack.pop();
                let next = self
                    .counter_stack
                    .last()
                    .cloned()
                    .map(|v| v + 1)
                    .unwrap_or(0);
                trace!("Returning to instruction {}", next);
                next_instruction = next;
            }
            Asm::Halt => {
                self.cont = false;
            }
            Asm::Push(v) => {
                self.memory.push(v.clone());
            }
            Asm::GetAttribute(attr) => {
                let dict = self.memory.pop().expect("No value found on stack");
                let val = match dict {
                    Value::Dictionary(mut dict) => {
                        dict.remove(attr.as_str()).expect("Attribute must exist")
                    }
                    Value::Reference(refr) => {
                        let inner = refr.borrow();
                        if let Value::Dictionary(dict) = &*inner {
                            dict.get(attr.as_str())
                                .expect("Attribute must exist")
                                .clone()
                        } else {
                            return Err(VMError::InvalidType {
                                value: inner.deref().clone(),
                                expected: "Dictionary".to_string(),
                            });
                        }
                    }
                    v => {
                        return Err(VMError::InvalidType {
                            value: v,
                            expected: "Dictionary".to_string(),
                        });
                    }
                };
                self.memory.push(val);
            }
            &Asm::SetVar(v) => {
                let value = self
                    .memory
                    .pop()
                    .expect("value expected from stack to save to ");
                self.memory.set_var(v as usize, value);
            }
            &Asm::GetVar(v) => {
                let val = self.memory.get_var(v as usize).expect("no var");
                let value: Value = val.into();
                self.memory.push(value);
            }
            &Asm::ClearVar(v) => {}
            Asm::SendMessage => {
                let mut target = self
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
                if let Some(next) = self.send_message(&mut target, &*message, args) {
                    next_instruction = next;
                }
            }
            Asm::IntoReference => {
                let mut target = Value::Native;
                let message = "ref";
                let args = vec![self
                    .memory
                    .pop()
                    .expect("There should be a target value on the stack")];
                if let Some(next) = self.send_message(&mut target, message, args) {
                    next_instruction = next;
                }
            }
            Asm::NativeMethod(msg, count) => {
                let mut target = Value::Native;
                let message = &*msg;
                let mut args = vec![];
                for _ in 0..*count {
                    args.push(
                        self.memory
                            .pop()
                            .expect("Expected a value on the stack for native method call"),
                    )
                }
                if let Some(next) = self.send_message(&mut target, message, args) {
                    next_instruction = next;
                }
            }
            a => panic!("Invalid instruction: {:?}", a),
        }
        Ok(next_instruction)
    }

    fn enclosed(&mut self, asm: &Assembly) -> Value {
        todo!()
    }

    fn load<Assembly: GetAsm>(&mut self, asm: Assembly) {
        let start_index = self.instructions.len();
        let as_asm = asm.get_asm();
        let mut new_labels = HashMap::new();
        for (index, asm) in as_asm.into_iter().enumerate() {
            let label = match &asm {
                Asm::Label(lbl) => Some(lbl),
                Asm::PublicLabel(lbl) => Some(lbl),
                _ => None,
            };

            if let Some(asm_label) = label {
                let label_index = start_index + index;
                match self.label_to_instruction.entry(asm_label.clone()) {
                    Entry::Occupied(_) => {
                        panic!("label {:?} already registered", asm_label);
                    }
                    Entry::Vacant(v) => {
                        v.insert(label_index);
                        new_labels.insert(asm_label.clone(), label_index);
                    }
                }
            }
            self.instructions.push(asm);
        }
        info!("Created new labels = {:?}", new_labels);
    }

    fn load_static<Assembly: GetAsm>(&mut self, asm: Assembly) {
        let start_index = self.instructions.len();
        let label = "@@STATIC".to_string();
        self.label_to_instruction.insert(label.clone(), start_index);
        self.load(asm);
        if self.run(&*label).expect("VM Error encountered") != 0 {
            panic!("VM Failed")
        }
    }

    fn run(&mut self, start_label: &str) -> Result<u32, VMError> {
        self.cont = true;
        let start_counter = self.label_to_instruction[start_label];
        self.counter_stack.push(start_counter);
        loop {
            while self.cont && (1..=self.instructions.len() - 1).contains(&self.program_counter()) {
                let pc = self.program_counter();
                let ref instruction = self.instructions[pc].clone();
                info!(target: "virtual_machine", "0x{:016X}: {:?}", pc, instruction);
                let next = self.interpret_instruction(instruction, pc)?;
                self.set_program_counter(next);
            }

            match std::mem::replace(&mut self.handler, None) {
                None => break,
                Some(handle) => {
                    self.kernel_mode = false;
                    self.end_fault(handle);
                }
            }
        }
        match self.memory.pop() {
            None => Err(VMError::NoExitCode),
            Some(Value::UInteger(u)) => Ok(u as u32),
            Some(v) => Err(VMError::ExitCodeInvalidType(v)),
        }
    }

    fn fault(&mut self, fault: Fault) {
        let target = self.fault_table.get_fault_jump(&fault);

        let saved_counter = std::mem::replace(&mut self.counter_stack, vec![0]);
        let saved_stack = self.memory.take_stack();
        let handle = FaultHandle::new(saved_counter, saved_stack, fault, target.clone());

        let next_pc = match &target {
            Value::Function(AsmLocation::Label(s)) => {
                match self.label_to_instruction.entry(s.clone()) {
                    Entry::Occupied(v) => *v.get(),
                    Entry::Vacant(_) => {
                        self.fault(Fault::DoubleFault);
                        return;
                    }
                }
            }
            Value::Native => {
                self.kernel_mode = true;
                self.handle_native_fault(&handle);
                0
            }
            v => panic!("Invalid value for fault jump target (value = {:?})", v),
        };
        self.handler = Some(handle);
        self.counter_stack.push(next_pc);
        self.kernel_mode = true;
    }

    fn is_kernel_mode(&self) -> bool {
        self.kernel_mode
    }
}

pub struct VMBuilder<'l, A, M> {
    arithmetic: Option<A>,
    memory: Option<M>,
    stdin: Box<dyn Read + 'l>,
    stdout: Box<dyn Write + 'l>,
    stderr: Box<dyn Write + 'l>,
    object_path: Vec<PathBuf>,
}

impl<'l, A: ArithmeticsTrait, M: MemoryTrait> VMBuilder<'l, A, M> {
    pub fn build(self) -> Result<VM<'l, M, A>, VMError> {
        let VMBuilder {
            arithmetic,
            memory,
            stdin,
            stdout,
            stderr,
            object_path,
        } = self;
        let mut vm = VM {
            memory: memory.expect("Memory module must be set"),
            alu: arithmetic.expect("Arithmetic module must be set"),
            cont: false,
            instructions: vec![Asm::Nop],
            label_to_instruction: Default::default(),
            counter_stack: vec![],
            stdin,
            stdout,
            stderr,
            next_anonymous_function: Default::default(),

            handler: None,
            fault_table: Default::default(),
            kernel_mode: false,
        };
        for obj_path in object_path {
            obj_path.try_load_into_vm(&mut vm)?;
        }
        Ok(vm)
    }
}

impl<'l, A, M> VMBuilder<'l, A, M> {
    pub fn new() -> Self {
        Self {
            arithmetic: None,
            memory: None,
            stdin: Box::new(stdin()),
            stdout: Box::new(stdout()),
            stderr: Box::new(stderr()),
            object_path: vec![],
        }
    }

    pub fn with_stdin<R: Read + 'l>(mut self, reader: R) -> Self {
        self.stdin = Box::new(reader);
        self
    }

    pub fn with_stdout<W: Write + 'l>(mut self, writer: W) -> Self {
        self.stdout = Box::new(writer);
        self
    }

    pub fn with_stderr<W: Write + 'l>(mut self, writer: W) -> Self {
        self.stderr = Box::new(writer);
        self
    }

    pub fn object_path<P: AsRef<OsStr>>(mut self, path: P) -> Self {
        let as_path = PathBuf::from(path.as_ref());
        self.object_path.push(as_path);
        self
    }
}

impl<A: ArithmeticsTrait, M> VMBuilder<'_, A, M> {
    pub fn alu(mut self, alu: A) -> Self {
        self.arithmetic = Some(alu);
        self
    }
}

impl<A, M: MemoryTrait> VMBuilder<'_, A, M> {
    pub fn memory(mut self, memory: M) -> Self {
        self.memory = Some(memory);
        self
    }
}
