//! Contains the virtual machine

use crate::memory::{Heap, Stack};

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use crate::chunk::{Chunk, ByteCodeVector};
use crate::bytecode::ByteCode;

/// The machine that actually runs the bytecode
pub struct VirtualMachine {

    interrupt_sender: Sender<Interrupt>,
    halt_receiver: Receiver<()>
}


impl  VirtualMachine  {
    pub fn boot_with(chunk: Chunk) -> Self {
        let (interrupt_sender, halt_receiver) = Self::create_core(chunk);
        Self {
            interrupt_sender,
            halt_receiver
        }
    }

    fn create_core(base_heap: Chunk) -> (Sender<Interrupt>, Receiver<()>)  {
        let (halt_s, halt_r) = mpsc::channel::<()>();
        let (interrupt_s, interrupt_r) = mpsc::channel::<Interrupt>();

        let  mut heap = Heap::new();
        heap.data_mut().extend(&base_heap.0);
        let mut core =  Core::new(heap, halt_s, interrupt_r);
        std::thread::spawn(move || {
            core.run();
        });
        (interrupt_s, halt_r)
    }

    pub fn wait_for_halt(&mut self) {
        while let Err(TryRecvError::Empty) = self.halt_receiver.try_recv() {

        }
    }
}

pub enum Interrupt {
    RunCode(usize)
}

pub struct Core {
    heap: Heap,
    stack: Stack,
    halt: Sender<()>,
    interrupt_receiver: Receiver<Interrupt>
}

impl Core {


    fn run(&mut self) {
        let mut cont = true;
        let mut ip = 0;
        while cont {
            let chunkish = ByteCodeVector::new(self.heap.data());
            let (code, bytes) = chunkish.bytecode_and_operands(ip);
            let next_ip = chunkish.next_bytecode(ip);
            match code {
                ByteCode::Halt => {
                    cont = false;
                    self.halt.send(());
                    break;
                }
                _ => { }
            }


            if next_ip.is_none() {
                cont = false;
                self.halt.send(());
            } else {
                ip = next_ip.unwrap();
            }
        }
    }

    pub fn new(heap: Heap, halt: Sender<()>, interrupt_receiver: Receiver<Interrupt>) -> Self {
        Core { heap, stack: Stack::new(), halt, interrupt_receiver }
    }
}


#[cfg(test)]
mod tests {
    use crate::chunk::Chunk;
    use crate::vm::VirtualMachine;

    #[test]
    fn run_basic() {
        let mut chunk = Chunk::new();
        chunk.halt();
        let mut vm = VirtualMachine::boot_with(chunk);
        vm.wait_for_halt();
        println!("VM Finished")
    }
}