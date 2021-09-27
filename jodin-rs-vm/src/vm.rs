//! Contains the virtual machine

use crate::memory::{Heap, Stack, PopFromStack};

use crate::bytecode::ByteCode;
use crate::chunk::{ByteCodeVector, Chunk};
use crate::frame::{Frame};
use std::collections::VecDeque;
use std::panic::{catch_unwind, UnwindSafe};
use std::sync::{mpsc, RwLock, Arc};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use crate::compound_types::{Pointer, Array, Pair, LocalVarsDeclarations, FunctionInfo};
use crate::symbols::{SystemCallTable, SystemCall, Symbol};
use std::ffi::CString;
use std::str::FromStr;


pub const PUSH_SYMBOL_TO_STACK: usize = 0;
pub const CREATE_LOCAL_VARS_DECLARATION: usize = 1;



/// The machine that actually runs the bytecode
pub struct VirtualMachine {
    interrupt_sender: Sender<Interrupt>,
    halt_receiver: Receiver<()>,
    sys_calls: Arc<RwLock<SystemCallTable<SYS_CALLS>>>
}

impl VirtualMachine {
    pub fn boot_with(chunk: Chunk) -> Self {
        let mut sys_calls = SystemCallTable::new();
        sys_calls[PUSH_SYMBOL_TO_STACK] = SystemCall::VM(Core::push_current_symbol);
        sys_calls[CREATE_LOCAL_VARS_DECLARATION] = SystemCall::VM(Core::create_local_vars_declaration);


        let sys_calls = Arc::new(RwLock::new(sys_calls));
        let (interrupt_sender, halt_receiver) = Self::create_core(chunk, sys_calls.clone());
        Self {
            interrupt_sender,
            halt_receiver,
            sys_calls
        }
    }

    fn create_core(base_heap: Chunk, ref sys_calls: Arc<RwLock<SystemCallTable<SYS_CALLS>>>) -> (Sender<Interrupt>, Receiver<()>) {
        let (halt_s, halt_r) = mpsc::channel::<()>();
        let (interrupt_s, interrupt_r) = mpsc::channel::<Interrupt>();

        let mut heap = Heap::new();
        heap.data_mut().extend(&base_heap.0);
        let core = Core::new(heap, halt_s, interrupt_r, sys_calls);
        std::thread::spawn(move || {
            core.run();
        });
        (interrupt_s, halt_r)
    }

    pub fn send_interrupt(&self, interrupt: Interrupt) {
        self.interrupt_sender
            .send(interrupt)
            .expect("Could not send interrupt")
    }

    pub fn wait_for_halt(&self) {
        while let Err(TryRecvError::Empty) = self.halt_receiver.try_recv() {}
    }
}

pub enum Interrupt {
    RunCode(Chunk),
    Halt,
    SysCall(usize)
}

pub const SYS_CALLS: usize = 256;

pub struct Core {
    heap: Heap,
    pub stack: Stack,
    halt: Sender<()>,
    interrupt_receiver: Receiver<Interrupt>,

    frames: Vec<Frame>,
    current_frame: usize,

    interrupt_queue: VecDeque<Interrupt>,
    wait_for_interrupt: bool,

    cont: bool,

    sys_calls: Arc<RwLock<SystemCallTable<SYS_CALLS>>>
}

impl UnwindSafe for Core {}

unsafe impl Sync for Core { }
unsafe impl Send for Core { }

impl Core {
    pub fn new(heap: Heap, halt: Sender<()>, interrupt_receiver: Receiver<Interrupt>, sys_calls: &Arc<RwLock<SystemCallTable<SYS_CALLS>>>) -> Self {
        Core {
            heap,
            stack: Stack::new(),
            halt,
            interrupt_receiver,
            frames: vec![],
            current_frame: 0,
            interrupt_queue: Default::default(),
            wait_for_interrupt: false,
            cont: true,
            sys_calls: sys_calls.clone()
        }
    }

    fn create_local_vars_declaration(&mut self) {
        let count: usize = self.stack.pop().unwrap();
        let mut vec = VecDeque::new();
        for _ in 0..count {
            let pair: Pair<usize, usize> = self.stack.pop().unwrap();
            vec.push_front(pair);
        }
        let array = Array::new(Vec::from(vec));
        let locals = LocalVarsDeclarations::new(array);
        self.stack.push(locals);
    }

    fn push_current_symbol(&mut self) {
        let symbol = self.current_frame().within_symbol.clone().unwrap();
        let string = symbol.to_string();
        let c_string = CString::new(string).expect("symbol is invalid c_string");
        self.stack.push(c_string);
    }

    fn apply_generics(&mut self) {
        let symbol: CString = self.stack.pop().unwrap();
        let generics_array: Array<CString> = self.stack.pop().unwrap();

        let symbol = Symbol::from_str(symbol.to_str().unwrap()).unwrap();

    }


    fn handle_interrupts(&mut self) {
        while let Ok(next) = self.interrupt_receiver.try_recv() {
            self.interrupt_queue.push_back(next);
        }

        let drain = self.interrupt_queue.drain(..).collect::<Vec<_>>();
        for interrupt in drain {
            self.handle_interrupt(interrupt);
        }
    }

    fn handle_interrupt(&mut self, interrupt: Interrupt) {
        match interrupt {
            Interrupt::RunCode(code) => {
                let next_ip = self.heap.data().len();
                self.heap.data_mut().extend(code.0);
                self.current_frame_mut().instruction_pointer = next_ip;
            }
            Interrupt::Halt => {
                self.halt.send(());
                self.cont = false;
            }
            Interrupt::SysCall(call) => {
                let table = self.sys_calls.read().expect("Syscalls poisoned");
                let sys_call = table[call];
                drop(table);
                match sys_call {
                    SystemCall::VM(vm) => {
                        (vm)(self);
                    }
                    SystemCall::FunctionPointer(pointer) => {
                        self.call(pointer);
                    }
                }
            }
        }
    }
    fn current_frame(&self) -> &Frame {
        &self.frames[self.current_frame]
    }

    fn current_frame_mut(&mut self) -> &mut Frame {
        &mut self.frames[self.current_frame]
    }

    fn get_var(&mut self, parent_count: usize, variable: usize) {
        let (size, ptr) = self.variable_pointer(parent_count, &variable);
        let slice = self.heap.get_data(ptr as usize, size as usize);
        self.stack.push(slice);
    }

    fn store_to_var(&mut self, parent_count: usize, variable: usize) {
        let (size, ptr) = self.variable_pointer(parent_count, &variable);

        // pop the variable and store it
        let bytes = self.stack.pop_vec(size as usize);

        self.heap.set_data(ptr as usize, bytes);
    }

    fn variable_pointer(&self, parent_count: usize, variable: &usize) -> (usize, usize) {
        let mut frame_ptr = self.current_frame();
        for _ in 0..parent_count {
            let next_frame = frame_ptr.frame_parent;
            frame_ptr = &self.frames[next_frame];
        }

        let heap_pointer = frame_ptr.locals_heap_pointer;
        let (offset, size) = frame_ptr.locals_offset_size[&variable];

        let ptr = heap_pointer + offset;
        (size, ptr)
    }

    fn call(&mut self, pointer: Pointer) {
        let ip = pointer.0;
        if ip == 0 {
            panic!("Function at pointer 0 is invalid!")
        }
    }

    pub fn call_function_info(&mut self, function_info: &FunctionInfo) {

    }

    fn run(mut self) {
        let send = self.halt.clone();
        let result = catch_unwind(move || {
            while self.cont {
                self.handle_interrupts();

                if !self.wait_for_interrupt && self.cont {
                    let ip = self.current_frame().instruction_pointer;

                    let chunkish = ByteCodeVector::new(self.heap.data());
                    println!("{}", chunkish.disassemble_instruction(ip));
                    let (code, _bytes) = chunkish.bytecode_and_operands(ip);

                    let mut next_ip = chunkish.next_bytecode(ip);
                    match code {
                        ByteCode::Halt => {
                            self.halt.send(()).unwrap();
                            break;
                        }
                        ByteCode::WaitForRunCode => {
                            next_ip = Some(ip);
                        }
                        ByteCode::SysCall => {
                            let sys_call_num: usize = self.stack.pop().unwrap();
                            self.interrupt_queue.push_back(Interrupt::SysCall(sys_call_num));
                        }
                        _ => {}
                    }

                    if next_ip.is_none() {
                        self.halt.send(()).unwrap();
                        break;
                    } else {
                        self.frames
                            .get_mut(self.current_frame)
                            .unwrap()
                            .instruction_pointer = next_ip.unwrap();
                    }
                }
            }
        });
        if result.is_err() {
            send.send(()).unwrap();
        }
    }


}

#[cfg(test)]
mod tests {
    use crate::chunk::Chunk;
    use crate::vm::{Interrupt, VirtualMachine};

    #[test]
    fn run_basic() {
        let mut chunk = Chunk::new_start_at(256);
        chunk.halt();
        let vm = VirtualMachine::boot_with(chunk);
        vm.wait_for_halt();
        println!("VM Finished")
    }

    #[test]
    #[should_panic]
    fn cant_send_interrupt_after_halt() {
        let mut chunk = Chunk::new_start_at(256);

        chunk.halt();
        let vm = VirtualMachine::boot_with(chunk);
        vm.wait_for_halt();
        println!("VM Finished");
        vm.send_interrupt(Interrupt::Halt);
    }
}
