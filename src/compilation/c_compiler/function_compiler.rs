use crate::compilation::c_compiler::CValidIdentifier;

/// Compiles functions
pub struct FunctionCompiler;

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
