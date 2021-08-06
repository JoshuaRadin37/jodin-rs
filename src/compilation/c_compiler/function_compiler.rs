use std::iter::FromIterator;

use crate::{ast::{JodinNode, JodinNodeInner}, compilation::{MicroCompiler, c_compiler::{CType, CValidIdentifier}}, core::{error::{JodinErrorType, JodinResult}, generic_context::GenericContext}, passes::analysis::ResolvedIdentityTag};

use super::{C99, CompoundStatement, FunctionInfo};

/// Compiles functions
pub struct FunctionCompiler(GenericContext);

impl MicroCompiler<C99, FunctionInfo> for FunctionCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<FunctionInfo> {
        if let JodinNodeInner::FunctionDefinition {
            name,
            return_type,
            arguments,
            generic_parameters,
            block,
        } = tree.inner()
        {
            let c_valid_id: CValidIdentifier = CValidIdentifier::new(name.get_tag::<ResolvedIdentityTag>()?.absolute_id().clone());
            let return_type = CType::from(return_type);
            let arguments = vec![];
            
            Ok(FunctionInfo::new(c_valid_id, return_type, arguments, CompoundStatement::empty()))
        } else {
            Err(JodinErrorType::IllegalTreeType.into())
        }
    }
}

/// Compile methods
pub struct MethodCompiler<'a> {
    this: &'a CValidIdentifier,
    generics: GenericContext
}

impl<'a> MethodCompiler<'a> {
    /// Create a new method compiler, using a c valid identifier to represent the
    /// this type
    pub fn new(this: &'a CValidIdentifier) -> Self {
        MethodCompiler { this, generics: GenericContext::from_iter(vec![]) }
    }
}
