use crate::ast::tags::TagTools;
use crate::ast::JodinNodeType;
use crate::compilation::jodin_vm_compiler::asm_block::AssemblyBlock;
use crate::compilation::{Compilable, Compiler, Context, MicroCompiler, PaddedWriter, Target};
use crate::compilation_settings::CompilationSettings;
use crate::core::identifier::{Identifiable, Identifier};
use crate::utility::Tree;
use crate::{JodinNode, JodinResult};
use jodin_asm::asm_version::Version;
use jodin_asm::mvp::bytecode::{Asm, Assembly, Bytecode, Encode};
use std::collections::VecDeque;
use std::fmt::{write, Display, Formatter, Write};
use std::fs::File;
use std::io;
use std::io::stdout;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicI32;

pub mod asm_block;

/// The jodin compiler
pub struct JodinVM(Version);

impl Target for JodinVM {}

pub struct JodinVMCompiler;

impl Compiler<JodinVM> for JodinVMCompiler {
    fn compile(&mut self, tree: &JodinNode, settings: &CompilationSettings) -> JodinResult<()> {
        let modules = split_by_module(tree);
        let context = Context::new();
        for module in modules {
            let mut compiler = if settings.compile_to_stdout {
                ModuleCompiler::new(stdout())
            } else {
                module.compiler(&settings.target_directory)
            };

            module.compile(&context, &mut compiler);
        }

        Ok(())
    }
}

pub struct ModuleCompiler {
    writer: PaddedWriter<Box<dyn io::Write>>,
}

impl ModuleCompiler {
    pub fn new<W: io::Write>(writer: W) -> Self {
        ModuleCompiler {
            writer: PaddedWriter::new(Box::new(writer)),
        }
    }
}

pub struct Module<'j> {
    pub identifier: Identifier,
    pub members: Vec<&'j JodinNode>,
}

impl<'j> Module<'j> {
    /// Creates a compiler that
    pub fn compiler<P: AsRef<Path>>(&self, base_directory: P) -> ModuleCompiler {
        let mut buffer = PathBuf::from(base_directory.as_ref());
        for c in &self.identifier {
            buffer.push(c);
        }
        buffer.set_extension(".jasm");
        let file = File::create(&buffer).expect(
            format!(
                "Could not create compilation target file (target: {:?})",
                buffer
            )
            .as_str(),
        );
        ModuleCompiler::new(file)
    }
}

impl Compilable<JodinVM> for Module {
    fn compile<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        todo!()
    }
}

pub struct CompiledModule {
    pub identifier: Identifier,
    pub assembly: Assembly,
}

/// Splits a parse tree by module
pub fn split_by_module(tree: &JodinNode) -> Vec<Module> {
    let mut module = Module {
        identifier: Identifier::empty(),
        members: vec![],
    };

    let mut made = _split_by_module(tree, &mut module);
    made.insert(0, module);
    made
}

fn _split_by_module<'j>(tree: &'j JodinNode, current_module: &mut Module<'j>) -> Vec<Module<'j>> {
    let mut output = VecDeque::new();
    match tree.inner() {
        JodinNodeType::InNamespace { namespace, inner } => {
            let namespace = namespace.resolved_id().unwrap();
            let children = inner.direct_children();
            let mut new_module = Module {
                identifier: namespace.clone(),
                members: vec![],
            };
            for child in children {
                output.extend(_split_by_module(child, &mut new_module));
            }
            output.push_front(new_module);
        }
        _other => {
            current_module.members.push(tree);
        }
    }
    output.into()
}

/// A compiled object is a finished assembly module
#[derive(Debug)]
pub struct CompiledObject {
    /// The identifier of the module, corresponding to some identifier within the program
    pub identifier: Identifier,
    /// The version of the compiled object
    pub version: Version,
    /// The assembly block
    pub block: AssemblyBlock,
}

impl Display for CompiledObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version.to_magic_number())?;
        write!(f, "#{}#", self.identifier);
        let assembly = self.block.normalize();
        let encoded: Bytecode = assembly.encode();
        write!(f, "{}", encoded.len())?;
        for by in encoded {
            f.write_char(by as char)?;
        }
        writeln!()
    }
}

#[cfg(test)]
mod tests {
    use crate::parsing::parse_program;
    use crate::process_jodin_node;
    use jodin_asm::init_logging;
    use log::LevelFilter;

    #[test]
    fn fibonacci() {
        const FIB_FUNCTION: &str = r#"
        fn fib(n: int) -> int {
            if (n < 2) {
                let x: int = 2;
                return n;
            } else {
                return fib(n-1) + fib(n-2);  
            }
        }
        "#;

        init_logging(LevelFilter::Info);
        let declaration = match parse_program(FIB_FUNCTION) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("{}", e);
                panic!("Couldn't parse fib function");
            }
        };
        let (processed, _) = process_jodin_node(declaration).expect("Should be processable");
        println!("{:#?}", processed)
    }
}
