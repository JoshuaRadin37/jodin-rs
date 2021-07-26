use crate::core::error::{JodinError, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::identifier_resolution::IdentifierResolver;

use crate::parsing::ast::JodinNode;
use crate::parsing::ast::JodinNodeInner;

use crate::parsing::ast::tags::Tag;
use crate::passes::toolchain::{
    FallibleTool, FallibleToolchain, FallibleToolchainUtilities, JodinFallibleTool,
};
use std::any::Any;

pub struct IdentityResolutionTool {
    chain: FallibleToolchain<JodinError, JodinNode, (JodinNode, IdentifierResolver)>,
}

impl IdentityResolutionTool {
    pub fn new() -> Self {
        let chain =
            FallibleToolchainUtilities::append_tool(IdentifierCreator::new(), IdentifierSetter);
        Self { chain }
    }
}

impl JodinFallibleTool for IdentityResolutionTool {
    type Input = JodinNode;
    type Output = (JodinNode, IdentifierResolver);

    fn invoke(&mut self, input: Self::Input) -> JodinResult<Self::Output> {
        FallibleTool::invoke(&mut self.chain, input)
    }
}
#[derive(Debug)]
pub struct IdentifierCreator {
    block_num: usize,
}

#[derive(Debug, Clone)]
pub struct ResolvedIdentityTag(Identifier);

impl ResolvedIdentityTag {
    pub fn absolute_id(&self) -> &Identifier {
        &self.0
    }

    pub fn new<I: Into<Identifier>>(id: I) -> Self {
        ResolvedIdentityTag(id.into())
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl IdentifierCreator {
    fn new() -> Self {
        Self { block_num: 0 }
    }

    fn get_block_num(&mut self) -> usize {
        let ret = self.block_num;
        self.block_num += 1;
        ret
    }

    fn create_identities(
        &mut self,
        tree: &mut JodinNode,
        id_resolver: &mut IdentifierResolver,
    ) -> JodinResult<()> {
        match tree.inner_mut() {
            // This one only occurs when requested
            JodinNodeInner::Identifier(id) => {
                let abs = id_resolver.create_absolute_path(id);
                let tag = ResolvedIdentityTag(abs);
                tree.add_tag(tag)?;
            }
            JodinNodeInner::VarDeclarations {
                var_type: _, names, ..
            } => {
                for name in names {
                    self.create_identities(name, id_resolver)?;
                }
            }
            JodinNodeInner::FunctionDefinition { .. } => {}
            JodinNodeInner::Block { expressions } => {
                let block_num = self.get_block_num();
                let tag = BlockIdentifier::new(block_num);

                id_resolver.push_namespace(Identifier::from(format!("{{block {}}}", block_num)));
                for expression in expressions {
                    self.create_identities(expression, id_resolver)?;
                }
                tree.add_tag(tag)?;
                id_resolver.pop_namespace();
            }
            JodinNodeInner::StructureDefinition {
                name,
                generic_parameters: _,
                members,
            } => {
                self.create_identities(name, id_resolver)?;
                let tag = name.get_tag::<ResolvedIdentityTag>()?;
                // tags_to_add.push(Box::new(tag.clone()));
                let name = tag.absolute_id();
                id_resolver.push_namespace(name.clone());
                todo!("Add generic parameters identity resolution");
                /*
                for member in members {
                    self.create_identities(member, id_resolver)?;
                }
                tree.add_tag(tag.clone())?;
                id_resolver.pop_namespace();

                 */
            }
            JodinNodeInner::NamedValue { name, .. } => {
                self.create_identities(name, id_resolver);
            }
            _ => {}
        }
        Ok(())
    }
}

impl JodinFallibleTool for IdentifierCreator {
    type Input = JodinNode;
    type Output = (JodinNode, IdentifierResolver);

    fn invoke(&mut self, mut input: Self::Input) -> JodinResult<Self::Output> {
        let mut resolver = IdentifierResolver::new();
        self.create_identities(&mut input, &mut resolver)?;
        Ok((input, resolver))
    }
}

pub struct IdentifierSetter;

impl JodinFallibleTool for IdentifierSetter {
    type Input = (JodinNode, IdentifierResolver);
    type Output = (JodinNode, IdentifierResolver);

    fn invoke(&mut self, input: Self::Input) -> JodinResult<Self::Output> {
        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::error::JodinResult;

    #[test]
    fn label_structure_members() -> JodinResult<()> {
        Ok(())
    }
}
