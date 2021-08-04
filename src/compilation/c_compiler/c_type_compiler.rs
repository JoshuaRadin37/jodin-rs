use crate::ast::JodinNode;
use crate::compilation::c_compiler::TranslationUnit;
use crate::compilation::{MicroCompiler, C99};
use crate::core::error::JodinResult;

pub struct CTypeCompiler;

impl MicroCompiler<C99, Vec<TranslationUnit>> for CTypeCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<Vec<TranslationUnit>> {
        todo!()
    }
}
