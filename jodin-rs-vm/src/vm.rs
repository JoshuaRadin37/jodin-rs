use crate::error::VMError;
use crate::{ArithmeticsTrait, GetAsm, MemoryTrait, VirtualMachine, CALL, RECEIVE_MESSAGE};
use jodin_asm::mvp::bytecode::{Asm, Assembly, Decode};
use jodin_asm::mvp::location::AsmLocation;
use jodin_asm::mvp::value::Value;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{stderr, stdin, stdout, Read, Write};
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
                let as_ref = Value::Reference(Box::new(RefCell::new(target)));
                self.memory.push(as_ref);
            }
            "copy" => {
                let target = args.remove(0);
                let cloned = target.clone();
                self.memory.push(target);
                self.memory.push(cloned);
            }
            _ => panic!("{:?} is not a native method", message),
        }
    }

    fn send_message(&mut self, target: &mut Value, message: &str, mut args: Vec<Value>) {
        info!("Sending {:?} to {:?} with args {:?}", message, target, args);
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
                self.send_message(as_mut_ref, message, args);
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

                self.send_message(&mut value, CALL, args);
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

    fn call(&mut self, asm_location: &AsmLocation, mut args: Vec<Value>) {
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
        self.counter_stack.push(next_pc);
    }

    fn anonymous_function_label(&self) -> String {
        let num = self.next_anonymous_function.fetch_add(1, Ordering::Relaxed);
        format!("<anonymous function {}>", num)
    }

    fn set_program_counter(&mut self, pc: usize) {
        self.counter_stack.pop();
        self.counter_stack.push(pc);
    }
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
            Asm::Label(_) | Asm::Nop => {}
            Asm::Halt => {
                self.cont = false;
            }
            Asm::Goto(_) => {}
            Asm::CondGoto(_) => {}
            Asm::Push(v) => {
                self.memory.push(v.clone());
            }
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
                self.send_message(&mut target, &*message, args);
            }
            _ => panic!("Invalid instruction"),
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
            if let Asm::Label(asm_label) = &asm {
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

    fn run(&mut self, start_label: &str) -> Result<u32, VMError> {
        self.cont = true;
        let start_counter = self.label_to_instruction[start_label];
        self.counter_stack.push(start_counter);
        while self.cont && (1..=self.instructions.len() - 1).contains(&self.program_counter()) {
            let pc = self.program_counter();
            let ref instruction = self.instructions[pc].clone();
            debug!("0x{:016X}: {:?}", pc, instruction);
            let next = self.interpret_instruction(instruction, pc)?;

            self.set_program_counter(next);
        }
        match self.memory.pop() {
            None => Err(VMError::NoExitCode),
            Some(Value::UInteger(u)) => Ok(u as u32),
            Some(v) => Err(VMError::ExitCodeInvalidType(v)),
        }
    }
}

pub struct VMBuilder<'l, A, M> {
    arithmetic: Option<A>,
    memory: Option<M>,
    stdin: Box<dyn Read + 'l>,
    stdout: Box<dyn Write + 'l>,
    stderr: Box<dyn Write + 'l>,
}

impl<'l, A: ArithmeticsTrait, M: MemoryTrait> VMBuilder<'l, A, M> {
    pub fn build(self) -> VM<'l, M, A> {
        let VMBuilder {
            arithmetic,
            memory,
            stdin,
            stdout,
            stderr,
        } = self;
        VM {
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
        }
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
