use crate::compilation::jodin_vm_compiler::asm_block::AssemblyBlock;
use crate::compilation::jodin_vm_compiler::function_compiler::FunctionCompiler;
use crate::{JodinError, JodinNode, JodinResult};
use jodin_common::asm_version::Version;
use jodin_common::ast::JodinNodeType;
use jodin_common::compilation::{
    Compilable, Compiler, Context, MicroCompiler, PaddedWriter, Target,
};
use jodin_common::compilation_settings::CompilationSettings;
use jodin_common::core::tags::TagTools;
use jodin_common::error::JodinErrorType;
use jodin_common::identifier::{Identifiable, Identifier};
use jodin_common::mvp::bytecode::{Asm, Assembly, Bytecode, Encode};
use jodin_common::unit::CompilationObject;
use jodin_common::utility::Tree;
use std::borrow::Borrow;
use std::collections::{HashMap, VecDeque};
use std::fmt::{write, Display, Formatter, Write as fmtWrite};
use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::io;
use std::io::{stdout, Write};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicI32;

pub mod asm_block;
mod expression_compiler;
mod function_compiler;
mod statement_compiler;

/// The jodin compiler
pub struct JodinVM(Version);

impl Target for JodinVM {}

pub struct JodinVMCompiler<'c> {
    writer_override: Option<Box<dyn io::Write + 'c>>,
    lifetime: PhantomData<&'c ()>,
}

impl<'c> JodinVMCompiler<'c> {
    pub fn new<W>(writer: W) -> JodinVMCompiler<'c>
    where
        Option<W>: From<W>,
        W: io::Write + 'c,
    {
        let as_opt: Option<_> = writer.into();
        let as_box = as_opt.map(|w| {
            let boxed: Box<dyn io::Write> = Box::new(w);
            boxed
        });
        JodinVMCompiler {
            writer_override: as_box,
            lifetime: PhantomData::default(),
        }
    }
}

impl Default for JodinVMCompiler<'static> {
    fn default() -> Self {
        Self {
            writer_override: None,
            lifetime: PhantomData::default(),
        }
    }
}

impl<'c> Compiler<JodinVM> for JodinVMCompiler<'c> {
    fn compile(&mut self, tree: &JodinNode, settings: &CompilationSettings) -> JodinResult<()> {
        let modules = split_by_module(tree);
        let context = Context::new();
        for module in modules {
            info!("Compiling module {:?}", module.identifier);
            match &mut self.writer_override {
                None => {
                    let mut m_compiler = module.compiler(&settings.target_directory);
                    for member in module.members {
                        info!("Compiling {:?}", member);
                        let resolved_id = member.resolved_id()?;
                        let mut m_compiler =
                            m_compiler.translation_object_compiler(resolved_id.this());
                        m_compiler.compile(member, settings)?;
                    }
                }
                Some(s) => {
                    let mut writer = PaddedWriter::new(s);
                    module.compile(&context, &mut writer)?;
                }
            };
        }

        Ok(())
    }
}

pub struct ModuleCompiler {
    dir_path: PathBuf,
    module_id: Identifier,
}

impl ModuleCompiler {
    pub fn new<O: AsRef<Path>>(id: &Identifier, path: O) -> Self {
        ModuleCompiler {
            dir_path: path.as_ref().to_path_buf(),
            module_id: id.clone(),
        }
    }

    pub fn translation_object_compiler(
        &self,
        target: impl AsRef<str>,
    ) -> TranslationObjectCompiler {
        TranslationObjectCompiler {
            module_compiler: self,
            relative_path: PathBuf::from(target.as_ref()),
        }
    }
}

pub struct TranslationObjectCompiler<'m> {
    module_compiler: &'m ModuleCompiler,
    relative_path: PathBuf,
}

impl<'m> TranslationObjectCompiler<'m> {
    /// Get the target path of the translation object
    pub fn object_path(&self) -> PathBuf {
        PathBuf::from_iter(&[&self.module_compiler.dir_path, &self.relative_path])
            .with_extension("jobj")
    }
}

impl Compiler<JodinVM> for TranslationObjectCompiler<'_> {
    fn compile(&mut self, tree: &JodinNode, settings: &CompilationSettings) -> JodinResult<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.object_path())?;

        let ref mut w = PaddedWriter::new(file);
        let as_obj = self.create_compilable(tree)?;

        Compilable::<JodinVM>::compile(as_obj, &Context::new(), w)
    }
}

impl MicroCompiler<JodinVM, CompilationObject> for TranslationObjectCompiler<'_> {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<CompilationObject> {
        match tree.r#type() {
            JodinNodeType::FunctionDefinition { .. } => {
                let mut compiler = FunctionCompiler::default();
                let block = compiler.create_compilable(tree)?;
                let obj = CompilationObject::new(
                    self.object_path(),
                    self.module_compiler.module_id.clone(),
                    vec![],
                    block.normalize(),
                );
                Ok(obj)
            }
            _ => return Err(JodinError::new(JodinErrorType::IllegalTreeType)),
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
        if !self.identifier.is_empty() {
            for c in &self.identifier {
                buffer.push(c);
            }
        }
        if !self.members.is_empty() {
            std::fs::create_dir_all(&buffer);
        }
        ModuleCompiler::new(&self.identifier, buffer)
    }
}

impl<'j> Compilable<JodinVM> for Module<'j> {
    fn compile<W: io::Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        for x in self.members {
            writeln!(w, "{:#?}", x);
        }
        Ok(())
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
            let child = inner;
            let mut new_module = Module {
                identifier: namespace.clone(),
                members: vec![],
            };
            output.extend(_split_by_module(child, &mut new_module));
            output.push_front(new_module);
        }
        JodinNodeType::TopLevelDeclarations { decs } => {
            for dec in decs {
                output.extend(_split_by_module(dec, current_module));
            }
        }
        JodinNodeType::FunctionDefinition { .. }
        | JodinNodeType::CompoundTypeDefinition { .. }
        | JodinNodeType::ExternDeclaration { .. } => {
            debug!("{:?} added to module {:?}", tree, current_module.identifier);
            current_module.members.push(tree);
        }
        _ => {
            debug!(
                "{:?} wasn't added to module {:?}",
                tree, current_module.identifier
            );
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
        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::compilation::jodin_vm_compiler::JodinVMCompiler;
    use crate::parsing::parse_program;
    use crate::{process_jodin_node, JodinResult};
    use jodin_asm::init_logging;
    use jodin_common::compilation::Compiler;
    use jodin_common::compilation_settings::CompilationSettings;
    use jodin_common::init_logging;
    use log::LevelFilter;

    #[test]
    fn fibonacci() {
        const FIB_FUNCTION: &str = r#"
        in std
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
        println!("{:#?}", processed);
        let mut buffer = vec![0u8; 0];
        let mut compiler = JodinVMCompiler::new(&mut buffer);
        let result = compiler.compile(&processed, &CompilationSettings::new());
        drop(compiler);
        match result {
            Ok(_) => {
                println!("{}", String::from_utf8(buffer).expect("maybe utf-8 output"));
            }
            Err(e) => {
                eprintln!("{}", e);
                panic!("Fib failed to compile")
            }
        }
    }
}

#[derive(Default)]
pub struct VariableUseTracker {
    next_variable: usize,
    unused_vars: Vec<usize>,
    id_to_var_number: HashMap<Identifier, usize>,
}

impl VariableUseTracker {
    pub fn next_var(&mut self, id: &Identifier) -> usize {
        let num = if self.unused_vars.is_empty() {
            let next = self.next_variable;
            self.next_variable += 1;
            next
        } else {
            self.unused_vars.pop().unwrap()
        };
        self.id_to_var_number.insert(id.clone(), num);
        debug!("Set id {} to var #{}", id, num);
        num
    }

    pub fn next_var_asm(&mut self, id: &Identifier) -> Asm {
        let var = self.next_var(id);
        Asm::SetVar(var as u64)
    }

    pub fn get_id<I: Into<Identifier>>(&self, id: I) -> Option<usize> {
        self.id_to_var_number.get(&id.into()).copied()
    }

    pub fn contains_id<I: Into<Identifier>>(&self, id: I) -> bool {
        self.id_to_var_number.contains_key(&id.into())
    }

    /// Clears a variable regardless of whether it's set or not
    pub fn clear_var(&mut self, var: usize) {
        if let Some((id, _)) = self.id_to_var_number.iter().find(|&(_, &val)| val == var) {
            let id = id.clone();
            self.clear_id(&id)
        }
    }

    /// Clears an identifier, returning a var to the unused pool
    pub fn clear_id<Q>(&mut self, id: &Q)
    where
        Identifier: Borrow<Q>,
        Q: Hash + Eq,
    {
        if let Some(removed) = self.id_to_var_number.remove(id) {
            self.unused_vars.push(removed);
        }
    }
}

pub fn invalid_tree_type(expected: impl AsRef<str>) -> JodinErrorType {
    JodinErrorType::InvalidTreeTypeGivenToCompiler(expected.as_ref().to_string())
}
