use crate::ast::JodinNode;
use crate::compilation::{Context, MicroCompiler, C99};
use crate::core::error::JodinResult;

pub struct TopLevelDeclarationCompiler;

impl MicroCompiler<C99> for TopLevelDeclarationCompiler {
    fn compile_part(&mut self, tree: &JodinNode, context: &mut Context) -> JodinResult<()> {
        todo!()
    }
}
