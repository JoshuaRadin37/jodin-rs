use crate::compilation::c_compiler::{
    CType, CTypeDeclarator, CTypeSpecifier, CValidIdentifier, TranslationUnit,
};
use crate::compilation::C99;
use jodin_common::ast::{JodinNode, JodinNodeType};
use jodin_common::compilation::MicroCompiler;
use jodin_common::core::tags::ResolvedIdentityTag;
use jodin_common::error::{JodinErrorType, JodinResult};

/// Compile any type declaration into a c type declaration
pub struct CTypeCompiler;

impl MicroCompiler<C99, Vec<TranslationUnit>> for CTypeCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<Vec<TranslationUnit>> {
        match tree.inner() {
            JodinNodeType::StructureDefinition { .. } => {
                let mut struct_compiler = StructCompiler;
                struct_compiler.create_compilable(tree)
            }
            _ => {
                panic!("{:?} can not be compiled into a c type", tree)
            }
        }
    }
}

/// Compiles specifically Structure definitions
pub struct StructCompiler;

impl MicroCompiler<C99, Vec<TranslationUnit>> for StructCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<Vec<TranslationUnit>> {
        if let JodinNodeType::StructureDefinition { name, members } = tree.inner() {
            let name_id = name.get_tag::<ResolvedIdentityTag>()?.absolute_id();
            let c_name = CValidIdentifier::new(name_id.clone());

            let mut fields = vec![];
            for member in members {
                if let JodinNodeType::NamedValue { name, var_type } = member.inner() {
                    let name_id = name.get_tag::<ResolvedIdentityTag>()?.absolute_id();
                    let c_name = CValidIdentifier::no_mangle(name_id.clone())?;

                    let field_type = CType::from(var_type);

                    fields.push((field_type, c_name));
                } else {
                    return Err(JodinErrorType::IllegalTreeType.into());
                }
            }

            let declaration = TranslationUnit::StructureDeclaration {
                name: c_name.clone(),
                fields,
            };

            let type_def = TranslationUnit::Typedef {
                c_type: CType::new(
                    false,
                    CTypeSpecifier::NamedStruct {
                        name: c_name.clone(),
                    },
                    CTypeDeclarator::Identifier,
                ),
                identifier: c_name,
            };

            Ok(vec![declaration, type_def])
        } else {
            Err(JodinErrorType::IllegalTreeType.into())
        }
    }
}
