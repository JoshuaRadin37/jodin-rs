use crate::core::identifier::Identifier;
use crate::core::identifier_resolution::IdentifierResolver;
use crate::core::registry::Registry;
use crate::core::types::{JodinTypeReference, Type};
use crate::parsing::ast::jodin_node::JodinNode;
use crate::parsing::ast::node_type::JodinNodeInner;
use crate::parsing::ast::node_type::JodinNodeInner::Block;
use crate::parsing::ast::tags::Tag;
use crate::passes::toolchain::{Tool, Toolchain, ToolchainUtilities};

pub struct IdentityResolutionTool {
    chain: Toolchain<JodinNode, (JodinNode, IdentifierResolver)>,
}

impl IdentityResolutionTool {
    pub fn new() -> Self {
        Self {
            chain: IdentifierCreator.append_tool(IdentifierSetter),
        }
    }
}

impl Tool for IdentityResolutionTool {
    type Input = JodinNode;
    type Output = (JodinNode, IdentifierResolver);

    fn invoke(&mut self, input: Self::Input) -> Self::Output {
        self.chain.invoke(input)
    }
}
#[derive(Debug)]
pub struct IdentifierCreator {
    block_num: usize,
}

#[derive(Debug)]
pub struct ResolvedIdentityTag(Identifier);

impl ResolvedIdentityTag {
    pub fn absolute_id(&self) -> &Identifier {
        &self.0
    }
}

impl Tag for ResolvedIdentityTag {
    fn tag_type(&self) -> String {
        String::from("ResolvedId")
    }

    fn tag_info(&self) -> String {
        format!("[Resolved {}]", self.absolute_id())
    }

    fn max_of_this_tag(&self) -> u32 {
        1
    }
}

#[derive(Debug)]
pub struct BlockIdentifier(usize);

impl BlockIdentifier {
    pub fn new(val: usize) -> Self {
        Self(val)
    }

    pub fn block_num(&self) -> usize {
        self.0
    }
}

impl Tag for BlockIdentifier {
    fn tag_type(&self) -> String {
        "BlockNum".to_string()
    }

    fn max_of_this_tag(&self) -> u32 {
        1
    }
}

impl IdentifierCreator {
    fn get_block_num(&mut self) -> usize {
        let ret = self.block_num;
        self.block_num += 1;
        ret
    }

    fn create_identities(&mut self, tree: &mut JodinNode, id_resolver: &mut IdentifierResolver) {
        match tree.inner_mut() {
            JodinNodeInner::Type(_) => {}
            JodinNodeInner::Keyword(_) => {}
            JodinNodeInner::Literal(_) => {}
            // This one only occurs when requested
            JodinNodeInner::Identifier(id) => {
                let abs = id_resolver.create_absolute_path(id);
                let tag = ResolvedIdentityTag(abs);
                tree.add_tag(tag);
            }
            JodinNodeInner::VarDeclaration { name, .. } => {
                self.create_identities(name, id_resolver);
            }
            JodinNodeInner::Function { .. } => {}
            JodinNodeInner::Block { expressions } => {
                let block_num = self.get_block_num();
                let tag = BlockIdentifier::new(block_num);
                tree.add_tag(tag);
                id_resolver.push_namespace(Identifier::from(format!("{{block {}}}", block_num)));
                for expression in expressions {
                    self.create_identities(expression, id_resolver);
                }
                id_resolver.pop_namespace();
            }
            JodinNodeInner::StructureDefinition { name, members } => {}
            JodinNodeInner::NamedValue { .. } => {}
        }
    }
}

impl Tool for IdentifierCreator {
    type Input = JodinNode;
    type Output = (JodinNode, IdentifierResolver);

    fn invoke(&mut self, mut input: Self::Input) -> Self::Output {
        let mut resolver = IdentifierResolver::new();
        self.create_identities(&mut input, &mut resolver);
        (input, resolver)
    }
}

pub struct IdentifierSetter;

impl Tool for IdentifierSetter {
    type Input = (JodinNode, IdentifierResolver);
    type Output = (JodinNode, IdentifierResolver);

    fn invoke(&mut self, input: Self::Input) -> Self::Output {
        input
    }
}
