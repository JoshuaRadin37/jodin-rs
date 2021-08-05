use crate::ast::{JodinNode, JodinNodeInner};
use crate::compilation::c_compiler::components::TranslationUnit;
use crate::compilation::c_compiler::{CType, CTypeCompiler, CValidIdentifier};
use crate::compilation::{MicroCompiler, C99};
use crate::core::error::JodinResult;
use crate::passes::analysis::ResolvedIdentityTag;

/// Compiles top level declarations in Jodin
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
            ImportIdentifiers {
                import_data: _,
                affected,
            } => {
                ret.extend(self.create_compilable(affected)?);
            }
            TopLevelDeclarations { decs } => {
                for dec in decs {
                    ret.extend(self.create_compilable(dec)?);
                }
            }
            VarDeclarations {
                var_type,
                names,
                values,
            } => {
                let c_type = CType::from(var_type);
                let iterator = names.iter().zip(values.iter());

                for (name, _maybe_value) in iterator {
                    let c_name = CValidIdentifier::new(
                        name.get_tag::<ResolvedIdentityTag>()?.absolute_id().clone(),
                    );

                    let translation_unit = TranslationUnit::Declaration {
                        c_type: c_type.clone(),
                        identifier: c_name,
                    };

                    ret.push(translation_unit);
                }
            }
            FunctionDefinition {
                name,
                return_type,
                arguments,
                generic_parameters,
                block,
            } => {}
            StructureDefinition { .. } => {
                let mut type_compiler = CTypeCompiler;
                ret.extend(type_compiler.create_compilable(tree)?);
            }
            other => {
                //panic!("Couldn't compile {:?}", other)
            }
        }

        Ok(ret)
    }
}
