use crate::compilation::Target;
use jodin_asm::mvp::bytecode::Asm;

pub struct JodinVM;

impl Target for JodinVM {}

struct AssemblyBlock {
    pub name: String,
    assembly: Vec<AssemblyBlockComponent>,
}

trait PushAsm<T> {
    fn push_asm(&mut self, asm: T);
}

enum AssemblyBlockComponent {
    SingleInstruction(Asm),
    Block(AssemblyBlock),
}

impl PushAsm<Asm> for AssemblyBlock {
    fn push_asm(&mut self, asm: Asm) {
        todo!()
    }
}
