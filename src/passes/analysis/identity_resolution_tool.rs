use crate::core::registry::Registry;
use crate::core::types::{JodinTypeReference, Type};
use crate::parsing::ast::jodin_node::JodinNode;
use crate::parsing::ast::node_type::JodinNodeInner;
use crate::passes::toolchain::{Tool, Toolchain, ToolchainUtilities};

pub struct IdentityResolutionTool {
    chain: Toolchain<JodinNode, (JodinNode, Registry<JodinTypeReference>)>,
}

impl IdentityResolutionTool {
    pub fn new() -> Self {
        Self {
            chain: IdentityCreator.append_tool(IdentitySetter),
        }
    }
}

impl Tool for IdentityResolutionTool {
    type Input = JodinNode;
    type Output = (JodinNode, Registry<JodinTypeReference>);

    fn invoke(&mut self, input: Self::Input) -> Self::Output {
        self.chain.invoke(input)
    }
}

pub struct IdentityCreator;

impl IdentityCreator {
    fn find_types(&self, tree: &JodinNode, registry: &mut Registry<JodinTypeReference>) {
        match tree.inner() {
            JodinNodeInner::Type(_) => {}
            JodinNodeInner::Keyword(_) => {}
            JodinNodeInner::Literal(_) => {}
            JodinNodeInner::Identifier(_) => {}
            JodinNodeInner::VarDeclaration { .. } => {}
            JodinNodeInner::Function { .. } => {}
            JodinNodeInner::Block { .. } => {}
            JodinNodeInner::StructureDefinition { name, members } => {}
            JodinNodeInner::NamedValue { .. } => {}
        }
    }
}

impl Tool for IdentityCreator {
    type Input = JodinNode;
    type Output = (JodinNode, Registry<JodinTypeReference>);

    fn invoke(&mut self, input: Self::Input) -> Self::Output {
        todo!()
    }
}

pub struct IdentitySetter;

impl Tool for IdentitySetter {
    type Input = (JodinNode, Registry<JodinTypeReference>);
    type Output = (JodinNode, Registry<JodinTypeReference>);

    fn invoke(&mut self, input: Self::Input) -> Self::Output {
        todo!()
    }
}
