use crate::ast::JodinNode;
use crate::compilation::c_compiler::components::TranslationUnit;
use crate::compilation::{MicroCompiler, C99};
use crate::core::error::JodinResult;

pub struct TopLevelDeclarationCompiler;

impl MicroCompiler<C99, Vec<TranslationUnit>> for TopLevelDeclarationCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<Vec<TranslationUnit>> {
        todo!()
    }
}
