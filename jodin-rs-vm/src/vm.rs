//! Contains the virtual machine

use crate::memory::{Heap, Stack};

use crate::bytecode::ByteCode;
use crate::chunk::{ByteCodeVector, Chunk};
use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use crate::frame::{FrameHeap, FrameStorage, FrameVarsContext, Frame};
use std::panic::{catch_unwind, UnwindSafe};

/// The machine that actually runs the bytecode
pub struct VirtualMachine {
    interrupt_sender: Sender<Interrupt>,
    halt_receiver: Receiver<()>,
}

impl VirtualMachine {
    pub fn boot_with(chunk: Chunk) -> Self {
        let (interrupt_sender, halt_receiver) = Self::create_core(chunk);
        Self {
            interrupt_sender,
            halt_receiver,
        }
    }

    fn create_core(base_heap: Chunk) -> (Sender<Interrupt>, Receiver<()>) {
        let (halt_s, halt_r) = mpsc::channel::<()>();
        let (interrupt_s, interrupt_r) = mpsc::channel::<Interrupt>();

        let mut heap = Heap::new();
        heap.data_mut().extend(&base_heap.0);
        let core = Core::new(heap, halt_s, interrupt_r);
        std::thread::spawn(move || {
            core.run();
        });
        (interrupt_s, halt_r)
    }

    pub fn send_interrupt(&self, interrupt: Interrupt) {
        self.interrupt_sender.send(interrupt).expect("Could not send interrupt")
    }

    pub fn wait_for_halt(&self) {
        while let Err(TryRecvError::Empty) = self.halt_receiver.try_recv() {}
    }
}

pub enum Interrupt {
    RunCode(Chunk),
    Halt
}

pub struct Core {
    heap: Heap,
    stack: Stack,
    halt: Sender<()>,
    interrupt_receiver: Receiver<Interrupt>,

    frames: FrameHeap,
    current_frame: usize,

    interrupt_queue: VecDeque<Interrupt>,
    wait_for_interrupt: bool,

    cont: bool,
}

impl UnwindSafe for Core {
    
}

impl Core {
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
        }
    }

    fn current_frame(&self) -> &Frame {
        self.frames.get_frame(self.current_frame).unwrap()
    }

    fn current_frame_mut(&mut self) -> &mut Frame {
        self.frames.get_frame_mut(self.current_frame).unwrap()
    }

    fn vars(&self) -> FrameVarsContext<FrameHeap> {
        todo!()
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
                    let (code, bytes) = chunkish.bytecode_and_operands(ip);
                    let ops = bytes.unwrap_or(&[]);

                    let next_ip = chunkish.next_bytecode(ip);
                    match code {
                        ByteCode::Halt => {
                            self.halt.send(());
                            break;
                        }
                        _ => {}
                    }

                    if next_ip.is_none() {
                        self.halt.send(());
                        break;
                    } else {
                        self.frames.get_frame_mut(self.current_frame).unwrap().instruction_pointer = next_ip.unwrap();
                    }
                }
            }
        }
        );
        if result.is_err() {
            send.send(());
        }
    }

    pub fn new(heap: Heap, halt: Sender<()>, interrupt_receiver: Receiver<Interrupt>) -> Self {
        Core {
            heap,
            stack: Stack::new(),
            halt,
            interrupt_receiver,
            frames: FrameHeap::instance(),
            current_frame: 0,
            interrupt_queue: Default::default(),
            wait_for_interrupt: false,
            cont: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::Chunk;
    use crate::vm::{VirtualMachine, Interrupt};

    #[test]
    fn run_basic() {
        let mut chunk = Chunk::new_start_at(256);
        chunk.halt();
        let mut vm = VirtualMachine::boot_with(chunk);
        vm.wait_for_halt();
        println!("VM Finished")
    }

    #[test]
    #[should_panic]
    fn cant_send_interrupt_after_halt() {
        let mut chunk = Chunk::new_start_at(256);

        chunk.halt();
        let mut vm = VirtualMachine::boot_with(chunk);
        vm.wait_for_halt();
        println!("VM Finished");
        vm.send_interrupt(Interrupt::Halt);
    }
}
