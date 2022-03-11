use std::cell::RefCell;
use std::rc::Rc;
use jodin_common::assembly::asm_block::AssemblyBlock;
use jodin_common::ast::JodinNode;
use jodin_common::compilation::MicroCompiler;
use jodin_common::error::JodinResult;
use jodin_common::types::type_environment::{TypeEnvironment, TypeEnvironmentManager};
use jodin_common::types::TypeTag;
use crate::compilation::jodin_vm_compiler::VariableUseTracker;
use crate::compilation::JodinVM;

pub struct ClassCompiler<'types> {
    type_environment: &'types TypeEnvironment,
    variable_use_tracker: Rc<RefCell<VariableUseTracker>>
}

impl<'types> ClassCompiler<'types> {

}

impl<'types> MicroCompiler<JodinVM, AssemblyBlock> for ClassCompiler<'types> {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<AssemblyBlock> {
        let tag = tree.get_tag::<TypeTag>()?;
        // tag.jodin_type()
    }
}