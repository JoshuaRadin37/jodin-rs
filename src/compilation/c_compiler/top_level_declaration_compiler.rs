use crate::ast::{JodinNode, JodinNodeInner};
use crate::compilation::c_compiler::components::TranslationUnit;
use crate::compilation::{MicroCompiler, C99};
use crate::core::error::JodinResult;

pub struct TopLevelDeclarationCompiler;

impl MicroCompiler<C99, Vec<TranslationUnit>> for TopLevelDeclarationCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<Vec<TranslationUnit>> {
        use JodinNodeInner::*;
        let mut ret = vec![];
        let node = tree.inner();
        match node {
            InNamespace {
                namespace: _,
                inner,
            } => {
                ret.extend(self.create_compilable(inner)?);
            }
            VarDeclarations {
                var_type,
                names,
                values,
            } => {}
            _ => {}
        }

        Ok(ret)
    }
}
