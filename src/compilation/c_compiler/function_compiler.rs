use crate::{
    ast::{JodinNode, JodinNodeType},
    compilation::{
        c_compiler::{CType, CValidIdentifier},
        MicroCompiler,
    },
    core::error::{JodinErrorType, JodinResult},
    passes::analysis::ResolvedIdentityTag,
};

use super::{CompoundStatement, FunctionInfo, C99};
use crate::compilation::c_compiler::StatementCompiler;

/// Compiles functions
pub struct FunctionCompiler;

impl MicroCompiler<C99, FunctionInfo> for FunctionCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<FunctionInfo> {
        if let JodinNodeType::FunctionDefinition {
            name,
            return_type,
            arguments,
            generic_parameters,
            block,
        } = tree.inner()
        {
            let c_valid_id: CValidIdentifier =
                CValidIdentifier::new(name.get_tag::<ResolvedIdentityTag>()?.absolute_id().clone());
            let return_type = CType::from(return_type);
            let mut c_arguments = vec![];

            for arg in arguments {
                if let JodinNodeType::NamedValue { name, var_type } = arg.inner() {
                    let c_type = CType::from(var_type);
                    let name = CValidIdentifier::new(
                        name.get_tag::<ResolvedIdentityTag>()?.absolute_id().clone(),
                    );

                    c_arguments.push((name, c_type));
                } else {
                    return Err(JodinErrorType::IllegalTreeType.into());
                }
            }

            let c_statements = StatementCompiler::new().create_compilable(block)?;

            Ok(FunctionInfo::new(
                c_valid_id,
                return_type,
                c_arguments,
                CompoundStatement::new(c_statements),
            ))
        } else {
            Err(JodinErrorType::IllegalTreeType.into())
        }
    }
}

/// Compile methods
pub struct MethodCompiler<'a> {
    this: &'a CValidIdentifier,
}

impl<'a> MethodCompiler<'a> {
    /// Create a new method compiler, using a c valid identifier to represent the
    /// this type
    pub fn new(this: &'a CValidIdentifier) -> Self {
        MethodCompiler { this }
    }
}
