use crate::core::error::{JodinErrorType, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::identifier_resolution::{IdentifierResolver, Registry};

use crate::ast::JodinNode;
use crate::ast::JodinNodeInner;

use crate::ast::tags::Tag;
use crate::core::privacy::{Visibility, VisibilityTag};
use std::any::Any;

/// A toolchain that assigns identities to every node that needs to be resolved. For example, the
/// types must all be resolved.
pub struct IdentityResolutionTool {
    creator: IdentifierCreator,
    setter: IdentifierSetter,
}

impl IdentityResolutionTool {
    /// Creates a new id resolution tool.
    pub fn new() -> Self {
        Self {
            creator: IdentifierCreator::new(),
            setter: IdentifierSetter,
        }
    }

    /// Resolve identifiers
    pub fn resolve_identities(
        &mut self,
        input: JodinNode,
    ) -> JodinResult<(JodinNode, IdentifierResolver)> {
        let mid = self.creator.invoke(input)?;
        self.setter.invoke(mid)
    }
}

/// This tag adds a resolved [Identifier](crate::core::identifier::Identifier) to a node. This resolved
/// identifier is absolute.
#[derive(Debug, Clone)]
pub struct ResolvedIdentityTag(Identifier);

impl ResolvedIdentityTag {
    /// The absolute identifier of the tag.
    pub fn absolute_id(&self) -> &Identifier {
        &self.0
    }

    /// Creates a new tag from an identifier-like value.
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

/// A tag that assigns an identifier to an individual block.
#[derive(Debug)]
pub struct BlockIdentifierTag(usize);

impl BlockIdentifierTag {
    /// Creates a new block identifier
    ///
    /// # Arguments
    ///
    /// * `val`: The value to use as the base for the identifier
    ///
    /// returns: BlockIdentifierTag
    pub fn new(val: usize) -> Self {
        Self(val)
    }

    /// Gets the block number of the tag
    pub fn block_num(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct IdentifierCreator {
    block_num: Vec<usize>,
}

impl Tag for BlockIdentifierTag {
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
        Self { block_num: vec![0] }
    }

    fn get_block_num(&mut self) -> usize {
        let block_num = self.block_num.last_mut().unwrap();
        let ret = *block_num;
        *block_num += 1;
        ret
    }

    fn create_identities(
        &mut self,
        tree: &mut JodinNode,
        id_resolver: &mut IdentifierResolver,
        visibility_registry: &mut Registry<Visibility>,
    ) -> JodinResult<()> {
        match tree.inner_mut() {
            // This one only occurs when requested
            JodinNodeInner::Identifier(id) => {
                match id_resolver
                    .resolve_path(id.clone())
                    .map_err(|e| e.error_type)
                {
                    Ok(path) => {
                        if visibility_registry.get(&path).unwrap() > &Visibility::Private {
                            return Err(JodinErrorType::IdentifierAlreadyExists(id.clone()).into());
                        }
                    }
                    Err(JodinErrorType::AmbiguousIdentifierError { given: _, found }) => {
                        for found in found {
                            if visibility_registry.get(&found).unwrap() > &Visibility::Private {
                                return Err(
                                    JodinErrorType::IdentifierAlreadyExists(id.clone()).into()
                                );
                            }
                        }
                    }
                    _ => {}
                }

                let abs = id_resolver.create_absolute_path(id);
                visibility_registry.insert_with_identifier(Visibility::Protected, abs.clone())?;
                if let Ok(tag) = tree.get_tag::<VisibilityTag>() {
                    let vis = tag.visibility().clone();
                    visibility_registry.update_absolute_identity(&abs, vis)?;
                }
                let tag = ResolvedIdentityTag(abs);
                tree.add_tag(tag)?;
            }
            JodinNodeInner::VarDeclarations {
                var_type: _, names, ..
            } => {
                for name in names {
                    self.create_identities(name, id_resolver, visibility_registry)?;
                }
            }
            JodinNodeInner::FunctionDefinition {
                name,
                return_type: _,
                arguments,
                generic_parameters,
                block,
            } => {
                self.create_identities(name, id_resolver, visibility_registry)?;
                let tag = name.get_tag::<ResolvedIdentityTag>()?.clone();
                let name = Identifier::from(tag.absolute_id().this());
                id_resolver.push_namespace(name);

                for argument in arguments {
                    self.create_identities(argument, id_resolver, visibility_registry)?;
                }

                for generic in generic_parameters {
                    self.create_identities(generic, id_resolver, visibility_registry)?;
                }

                self.create_identities(block, id_resolver, visibility_registry)?;

                id_resolver.pop_namespace();
            }
            JodinNodeInner::Block { expressions } => {
                self.start_block(id_resolver);

                let mut blocks = vec![];

                for expression in expressions {
                    if let JodinNodeInner::VarDeclarations { .. } = expression.inner() {
                        self.create_identities(expression, id_resolver, visibility_registry)?;
                    } else {
                        blocks.push(expression);
                    }
                }

                // Allows for forwards and backwards scope in blocks
                for block in blocks {
                    self.create_identities(block, id_resolver, visibility_registry)?;
                }

                self.end_block(id_resolver);
            }
            JodinNodeInner::StructureDefinition { name, members } => {
                self.create_identities(name, id_resolver, visibility_registry)?;

                let tag = name.get_tag::<ResolvedIdentityTag>()?.clone();
                // tags_to_add.push(Box::new(tag.clone()));
                let name = Identifier::from(tag.absolute_id().this());
                id_resolver.push_namespace(name);

                for member in members {
                    self.create_identities(member, id_resolver, visibility_registry)?;
                }

                id_resolver.pop_namespace();
            }
            JodinNodeInner::NamedValue { name, .. } => {
                self.create_identities(name, id_resolver, visibility_registry)?
            }
            JodinNodeInner::InNamespace { namespace, inner } => {
                self.create_identities(namespace, id_resolver, visibility_registry)?;
                let tag = namespace.get_tag::<ResolvedIdentityTag>()?.clone();
                let name = Identifier::from(tag.absolute_id().this());
                id_resolver.push_namespace(name);
                self.create_identities(inner, id_resolver, visibility_registry)?;
                id_resolver.pop_namespace();
            }
            JodinNodeInner::TopLevelDeclarations { decs } => {
                for child in decs {
                    self.create_identities(child, id_resolver, visibility_registry)?;
                }
            }
            JodinNodeInner::WhileStatement { cond: _, statement } => {
                self.start_block(id_resolver);
                self.create_identities(statement, id_resolver, visibility_registry)?;
                self.end_block(id_resolver);
            }
            JodinNodeInner::IfStatement {
                cond: _,
                statement,
                else_statement,
            } => {
                self.start_block(id_resolver);
                self.create_identities(statement, id_resolver, visibility_registry)?;
                self.end_block(id_resolver);

                if let Some(statement) = else_statement {
                    self.start_block(id_resolver);
                    self.create_identities(statement, id_resolver, visibility_registry)?;
                    self.end_block(id_resolver);
                }
            }
            JodinNodeInner::SwitchStatement {
                to_switch: _,
                labeled_statements,
            } => {
                self.start_block(id_resolver);
                for statement in labeled_statements {
                    self.create_identities(statement, id_resolver, visibility_registry)?;
                }
                self.end_block(id_resolver);
            }
            JodinNodeInner::ForStatement {
                init,
                cond: _,
                delta: _,
                statement,
            } => {
                self.start_block(id_resolver);

                if let Some(init) = init {
                    self.create_identities(init, id_resolver, visibility_registry)?;
                }
                self.create_identities(statement, id_resolver, visibility_registry)?;

                self.end_block(id_resolver);
            }
            other => {
                println!("Unsupported: {:?}", other);
            }
        }
        Ok(())
    }

    fn start_block(&mut self, id_resolver: &mut IdentifierResolver) {
        let block_num = self.get_block_num();
        let string = Identifier::from(format!("{{block {}}}", block_num));
        let last = id_resolver.current_namespace_with_base();
        self.block_num.push(0);

        id_resolver.push_namespace(string.clone());
        //id_resolver.create_absolute_path(&Identifier::from(""));
        id_resolver.use_namespace(last).unwrap();
        println!("{:#?}", id_resolver);
    }

    fn end_block(&mut self, id_resolver: &mut IdentifierResolver) {
        id_resolver.pop_namespace();
        self.block_num.pop();
        let current = id_resolver.current_namespace_with_base();
        id_resolver.stop_use_namespace(&current).unwrap();
    }

    fn invoke(&mut self, mut input: JodinNode) -> JodinResult<(JodinNode, IdentifierResolver)> {
        let mut resolver = IdentifierResolver::new();
        let mut registry = Registry::new();
        self.create_identities(&mut input, &mut resolver, &mut registry)?;
        println!("{:#?}", registry);
        Ok((input, resolver))
    }
}

fn find_first_tag<T: 'static + Tag>(node: &JodinNode) -> Option<&T> {
    if let Ok(ret) = node.get_tag() {
        return Some(ret);
    } else {
        for child in node {
            if let Some(ret) = find_first_tag(child) {
                return Some(ret);
            }
        }
        None
    }
}

pub struct IdentifierSetter;

impl IdentifierSetter {
    fn invoke(
        &mut self,
        input: (JodinNode, IdentifierResolver),
    ) -> JodinResult<(JodinNode, IdentifierResolver)> {
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
