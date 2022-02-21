use jodin_common::assembly::prelude::Assembly;
use jodin_rs_vm::core_traits::{ArithmeticsTrait, MemoryTrait, VirtualMachine};
use jodin_rs_vm::error::VMError;
use jodin_rs_vm::mvp::MinimumALU;
use jodin_rs_vm::scoped_memory::VMMemory;
use jodin_rs_vm::vm::{VMBuilder, VM};
use std::cell::RefCell;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct JVMRunner<M: MemoryTrait = VMMemory, A: ArithmeticsTrait = MinimumALU> {
    out: Vec<u8>,
    err: Vec<u8>,
    input: Vec<u8>,
    libs: Vec<PathBuf>,
    jasm: Vec<Assembly>,
    mem: M,
    alu: A,
    main_label: String,
}

impl<M: MemoryTrait, A: ArithmeticsTrait> JVMRunner<M, A> {
    pub fn set_mem(&mut self, mem: M) {
        self.mem = mem;
    }
    pub fn set_alu(&mut self, alu: A) {
        self.alu = alu;
    }

    pub fn set_input(&mut self, input: impl AsRef<str>) {
        self.input = input.as_ref().as_bytes().to_vec();
    }

    pub fn with_lib(mut self, lib: impl AsRef<Path>) -> Self {
        self.libs.push(lib.as_ref().to_path_buf());
        self
    }

    pub fn with_libs<P: AsRef<Path>, I: IntoIterator<Item = P>>(mut self, libs: I) -> Self {
        self.libs
            .extend(libs.into_iter().map(|l| l.as_ref().to_path_buf()));
        self
    }

    pub fn with_jasm(mut self, asm: impl Into<Assembly>) -> Self {
        self.jasm.push(asm.into());
        self
    }

    pub fn execute(self) -> Result<JVMResult, Box<dyn Error>> {
        let JVMRunner {
            mut out,
            mut err,
            mut input,
            libs,
            jasm,
            mem,
            alu,
            main_label,
        } = self;

        let mut vm_builder = VMBuilder::new()
            .memory(mem)
            .alu(alu)
            .with_stdout(&mut out)
            .with_stderr(&mut err)
            .with_stdin(&*input);

        for lib in libs {
            vm_builder = vm_builder.object_path(lib);
        }

        let mut vm = vm_builder.build()?;
        for asm in jasm {
            vm.load(asm);
        }

        let result = vm.run(main_label.as_str())?;
        drop(vm);


        let out = String::from_utf8(out)?;
        let err = String::from_utf8(err)?;
        Ok(JVMResult(result, out, err))
    }

    pub fn set_main_label(&mut self, main_label: String) {
        self.main_label = main_label;
    }
}

impl<M: MemoryTrait + Default, A: ArithmeticsTrait + Default> JVMRunner<M, A> {
    pub fn new() -> Self {
        Self {
            out: vec![],
            err: vec![],
            input: vec![],
            libs: vec![],
            jasm: vec![],
            mem: M::default(),
            alu: A::default(),
            main_label: "main".to_string(),
        }
    }
}

impl Default for JVMRunner {
    fn default() -> Self {
        Self {
            out: vec![],
            err: vec![],
            input: vec![],
            libs: vec![],
            jasm: vec![],
            mem: Default::default(),
            alu: Default::default(),
            main_label: "main".to_string(),
        }
    }
}

pub struct JVMResult(pub u32, pub String, pub String);

impl JVMResult {
    pub fn exit_code(&self) -> u32 {
        self.0
    }

    pub fn is_success(&self) -> bool {
        self.exit_code() == 0
    }

    pub fn out(&self) -> &str {
        self.1.as_str()
    }

    pub fn err(&self) -> &str {
        self.2.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::jvm_runner::JVMRunner;
    use jasm_macros::{jasm, label, pop, return_};
    use jodin_common::{init_logging, LevelFilter};
    use jodin_common::assembly::prelude::Asm;

    #[test]
    fn create_jvm_runner() {
        init_logging(LevelFilter::Info);
        let runner = JVMRunner::default().with_jasm(jasm![
            ASM_STYLE
            pub main;
                push 1u32;
                push 0u32;
                return;
        ]);
        let res = runner.execute().expect("Shouldn't fail");
        assert!(res.is_success(), "basic run shouldn't fail (exit code = {})", res.exit_code());
    }

    #[test]
    fn get_output() {
        init_logging(LevelFilter::Info);
        const HELLO_WORLD: &str = "Hello, World!";
        let runner = JVMRunner::default().with_jasm(jasm![
            ASM_STYLE
            pub main;
                push HELLO_WORLD;
                native ("print", 1)
                push HELLO_WORLD;
                push 2u32;
                native ("write", 2);
                return 0u32;
        ]);
        let res = runner.execute().expect("Shouldn't fail");
        assert!(res.is_success(), "basic run shouldn't fail (exit code = {})", res.exit_code());
        assert_eq!(res.out(), HELLO_WORLD, "Expected stdout output to be {HELLO_WORLD:?}");
        assert_eq!(res.err(), HELLO_WORLD, "Expected stderr output to be {HELLO_WORLD:?}");
    }
}
