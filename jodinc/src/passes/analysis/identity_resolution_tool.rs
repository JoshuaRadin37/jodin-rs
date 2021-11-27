use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::identifier_resolution::{IdentifierResolver, Registry};

use crate::ast::JodinNode;
use crate::ast::JodinNodeType;

use crate::ast::tags::Tag;
use crate::ast::tags::TagTools;
use crate::core::import::{Import, ImportType};
use crate::core::privacy::{Visibility, VisibilityTag};
use crate::core::types::intermediate_type::{IntermediateType, TypeSpecifier, TypeTail};
use crate::utility::Tree;
use std::any::Any;
use std::cmp::Ordering;
use std::process::id;

/// A toolchain that assigns identities to every node that needs to be resolved. For example, the
/// types must all be resolved.
pub struct IdentityResolutionTool {
    creator: IdentifierCreator,
    setter: IdentifierSetter,
    visibility: Registry<Visibility>,
}

impl IdentityResolutionTool {
    /// Creates a new id resolution tool.
    pub fn new() -> Self {
        Self {
            creator: IdentifierCreator::new(),
            setter: IdentifierSetter::new(),
            visibility: Registry::new(),
        }
    }

    /// Resolve identifiers
    pub fn resolve_identities(
        &mut self,
        input: JodinNode,
    ) -> JodinResult<(JodinNode, IdentifierResolver)> {
        info!("Creating absolute identifiers...");
        let (mut tree, mut resolver) = self.creator.start(input, &mut self.visibility)?;
        //println!("Visibilities: {:#?}", self.visibility);
        info!("Resolving identifiers...");
        match self
            .setter
            .set_identities(&mut tree, &mut resolver, &self.visibility)
        {
            Err(e) => {
                error!("Id resolver:\n{:#?}", resolver);
                tree.set_property("id_resolver", resolver);
                Err(e)
            }
            Ok(()) => {
                tree.set_property("id_resolver", resolver.clone());
                Ok((tree, resolver))
            }
        }
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
        format!("[Resolved '{}']", self.absolute_id())
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

/// Attach to a tag to disable identifier mangling
#[derive(Debug)]
pub struct NoMangle;

impl Tag for NoMangle {
    fn tag_type(&self) -> String {
        "No Mangle".to_string()
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
        let visiblity_tag = tree.get_tag::<VisibilityTag>().ok().cloned();
        match tree.inner_mut() {
            // This one only occurs when requested
            JodinNodeType::Identifier(id) => {
                match id_resolver
                    .resolve_path(id.clone(), false)
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
                info!("Created absolute identifier for use: {:?}", abs);
                visibility_registry.insert_with_identifier(Visibility::Protected, abs.clone())?;
                if let Ok(tag) = tree.get_tag::<VisibilityTag>() {
                    let vis = tag.visibility().clone();
                    visibility_registry.update_absolute_identity(&abs, vis)?;
                }
                let tag = ResolvedIdentityTag(abs);
                tree.add_tag(tag)?;
            }
            JodinNodeType::VarDeclarations {
                var_type: _, names, ..
            } => {
                for name in names {
                    self.create_identities(name, id_resolver, visibility_registry)?;
                }
            }
            JodinNodeType::StoreVariable {
                storage_type: _,
                name,
                ..
            } => {
                self.create_identities(name, id_resolver, visibility_registry)?;
            }
            JodinNodeType::FunctionDefinition {
                name,
                return_type: _,
                arguments,
                block,
            } => {
                self.create_identities(name, id_resolver, visibility_registry)?;
                let tag = name.get_tag::<ResolvedIdentityTag>()?.clone();
                let name = Identifier::from(tag.absolute_id().this());
                id_resolver.push_namespace(name);

                for argument in arguments {
                    self.create_identities(argument, id_resolver, visibility_registry)?;
                }

                self.create_identities(block, id_resolver, visibility_registry)?;

                id_resolver.pop_namespace();
            }
            JodinNodeType::Block { expressions } => {
                self.start_block(id_resolver);

                let mut blocks = vec![];

                for expression in expressions {
                    if let JodinNodeType::VarDeclarations { .. } = expression.inner() {
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
            JodinNodeType::StructureDefinition { name, members } => {
                if let Some(vis) = visiblity_tag {
                    name.add_tag(vis);
                }

                self.create_identities(name, id_resolver, visibility_registry)?;
                debug!("Set structure name to {}", name.resolved_id().unwrap());

                let tag = name.get_tag::<ResolvedIdentityTag>()?.clone();
                // tags_to_add.push(Box::new(tag.clone()));
                let name = Identifier::from(tag.absolute_id().this());

                // id_resolver.push_namespace(name.clone());
                debug!("Using semi-push for {}", name);
                id_resolver.semi_push(name.clone());
                for member in members {
                    self.create_identities(member, id_resolver, visibility_registry)?;
                    debug!(
                        "Structure {} member registered: {}",
                        name,
                        member.direct_children()[0].resolved_id().unwrap()
                    );
                }
                id_resolver.semi_pop();
            }
            JodinNodeType::NamedValue { name, .. } => {
                self.create_identities(name, id_resolver, visibility_registry)?
            }
            JodinNodeType::InNamespace { namespace, inner } => {
                self.create_identities(namespace, id_resolver, visibility_registry)?;
                let tag = namespace.get_tag::<ResolvedIdentityTag>()?.clone();
                let name = Identifier::from(tag.absolute_id().this());
                let parts: Vec<_> = name.iter().map(|s| Identifier::from(s)).collect();
                let count = parts.len();
                for name in parts {
                    id_resolver.push_namespace(name);
                }
                self.create_identities(inner, id_resolver, visibility_registry)?;
                for _ in 0..count {
                    id_resolver.pop_namespace();
                }
            }
            JodinNodeType::ImportIdentifiers {
                import_data: _,
                affected,
            } => {
                self.create_identities(affected, id_resolver, visibility_registry)?;
            }
            JodinNodeType::TopLevelDeclarations { decs } => {
                for child in decs {
                    self.create_identities(child, id_resolver, visibility_registry)?;
                }
            }
            JodinNodeType::WhileStatement { cond: _, statement } => {
                self.start_block(id_resolver);
                self.create_identities(statement, id_resolver, visibility_registry)?;
                self.end_block(id_resolver);
            }
            JodinNodeType::IfStatement {
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
            JodinNodeType::SwitchStatement {
                to_switch: _,
                labeled_statements,
            } => {
                self.start_block(id_resolver);
                for statement in labeled_statements {
                    self.create_identities(statement, id_resolver, visibility_registry)?;
                }
                self.end_block(id_resolver);
            }
            JodinNodeType::ForStatement {
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
            JodinNodeType::ExternDeclaration {
                declaration: delcaration,
            } => {
                self.create_identities(delcaration, id_resolver, visibility_registry)?;
            }
            _other => {}
        }
        Ok(())
    }

    fn start_block(&mut self, id_resolver: &mut IdentifierResolver) {
        let block_num = self.get_block_num();
        let string = Identifier::from(format!("{{block {}}}", block_num));
        let last = id_resolver.current_namespace().clone();
        self.block_num.push(0);

        id_resolver.push_namespace(string.clone());
        //id_resolver.create_absolute_path(&Identifier::from(""));
        if !last.is_empty() {
            id_resolver.use_namespace(last).unwrap();
        }
        //println!("{:#?}", id_resolver);
    }

    fn end_block(&mut self, id_resolver: &mut IdentifierResolver) {
        id_resolver.pop_namespace();
        self.block_num.pop();
        if !id_resolver.current_namespace().is_empty() {
            id_resolver
                .stop_use_namespace(&id_resolver.current_namespace())
                .unwrap();
        }
    }

    /// Pushes a namespace as the current namespace, while saving the current namespace
    /// as a used namespace, for the duration of the closure.
    pub fn semi_push_namespace<F, R>(
        &mut self,
        id: Identifier,
        resolver: &mut IdentifierResolver,
        mut closure: F,
    ) -> R
    where
        F: Fn(&mut IdentifierResolver) -> R,
    {
        let original_current = resolver.current_namespace();
        if !original_current.is_empty() {
            resolver.use_namespace(original_current.clone());
        }
        resolver.push_namespace(id);
        let output = closure(resolver);
        resolver.pop_namespace();
        if !original_current.is_empty() {
            resolver.stop_use_namespace(&original_current);
        }

        output
    }

    fn start(
        &mut self,
        mut input: JodinNode,
        registry: &mut Registry<Visibility>,
    ) -> JodinResult<(JodinNode, IdentifierResolver)> {
        let mut resolver = IdentifierResolver::new();
        self.create_identities(&mut input, &mut resolver, registry)?;
        //println!("{:#?}", registry);
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

pub struct IdentifierSetter {
    aliases: Registry<Identifier>,
    block_num: Vec<usize>,
}

impl IdentifierSetter {
    fn new() -> Self {
        Self {
            aliases: Registry::new(),
            block_num: vec![0],
        }
    }

    fn set_identities(
        &mut self,
        tree: &mut JodinNode,
        id_resolver: &mut IdentifierResolver,
        visibility_resolver: &Registry<Visibility>,
    ) -> JodinResult<()> {
        let has_id = tree.get_tag::<ResolvedIdentityTag>().is_ok();

        match tree.inner_mut() {
            JodinNodeType::InNamespace { namespace, inner } => {
                let namespace = namespace
                    .get_tag::<ResolvedIdentityTag>()
                    .unwrap()
                    .absolute_id()
                    .this_as_id();
                let parts: Vec<_> = namespace.iter().map(|s| Identifier::from(s)).collect();
                let count = parts.len();
                for name in parts {
                    id_resolver.push_namespace(name);
                }
                self.set_identities(inner, id_resolver, visibility_resolver)?;
                for _ in 0..count {
                    id_resolver.pop_namespace();
                }
            }
            JodinNodeType::ImportIdentifiers {
                import_data,
                affected,
            } => {
                let imports =
                    self.add_import_data(import_data, id_resolver, visibility_resolver)?;
                // println!("Imports: {:#?}", self.aliases);
                self.set_identities(affected, id_resolver, visibility_resolver)?;
                for import in imports {
                    self.aliases.remove_absolute_identity(&import)?;
                }
            }
            JodinNodeType::Identifier(id) => {
                if !has_id {
                    info!(
                        "Attempting to find {} from {}",
                        id,
                        id_resolver.current_namespace()
                    );
                    let resolved =
                        self.try_get_absolute_identifier(id, id_resolver, visibility_resolver)?;
                    info!("Found {}", resolved);
                    let resolved_tag = ResolvedIdentityTag::new(resolved);

                    tree.add_tag(resolved_tag)?;
                }
            }
            JodinNodeType::FunctionDefinition {
                name,
                return_type,
                arguments,
                block,
            } => {
                self.resolve_type(return_type, id_resolver, visibility_resolver)?;
                let tag = name.get_tag::<ResolvedIdentityTag>()?.clone();
                let name = Identifier::from(tag.absolute_id().this());
                id_resolver.push_namespace(name);

                for argument in arguments {
                    self.set_identities(argument, id_resolver, visibility_resolver)?;
                }

                self.set_identities(block, id_resolver, visibility_resolver)?;

                id_resolver.pop_namespace();
            }
            JodinNodeType::Block { expressions } => {
                self.start_block(id_resolver);

                let mut blocks = vec![];

                for expression in expressions {
                    if let JodinNodeType::VarDeclarations { .. } = expression.inner() {
                        self.set_identities(expression, id_resolver, visibility_resolver)?;
                    } else {
                        blocks.push(expression);
                    }
                }

                // Allows for forwards and backwards scope in blocks
                for block in blocks {
                    self.set_identities(block, id_resolver, visibility_resolver)?;
                }

                self.end_block(id_resolver);
            }
            JodinNodeType::StructureDefinition { name, members } => {
                self.set_identities(name, id_resolver, visibility_resolver)?;

                let tag = name.get_tag::<ResolvedIdentityTag>()?.clone();
                // tags_to_add.push(Box::new(tag.clone()));
                let name = Identifier::from(tag.absolute_id().this());
                id_resolver.push_namespace(name);

                for member in members {
                    self.set_identities(member, id_resolver, visibility_resolver)?;
                }

                id_resolver.pop_namespace();
            }
            JodinNodeType::WhileStatement { cond: _, statement } => {
                self.start_block(id_resolver);
                self.set_identities(statement, id_resolver, visibility_resolver)?;
                self.end_block(id_resolver);
            }
            JodinNodeType::IfStatement {
                cond: _,
                statement,
                else_statement,
            } => {
                self.start_block(id_resolver);
                self.set_identities(statement, id_resolver, visibility_resolver)?;
                self.end_block(id_resolver);

                if let Some(statement) = else_statement {
                    self.start_block(id_resolver);
                    self.set_identities(statement, id_resolver, visibility_resolver)?;
                    self.end_block(id_resolver);
                }
            }
            JodinNodeType::SwitchStatement {
                to_switch: _,
                labeled_statements,
            } => {
                self.start_block(id_resolver);
                for statement in labeled_statements {
                    self.set_identities(statement, id_resolver, visibility_resolver)?;
                }
                self.end_block(id_resolver);
            }
            JodinNodeType::ForStatement {
                init,
                cond: _,
                delta: _,
                statement,
            } => {
                self.start_block(id_resolver);

                if let Some(init) = init {
                    self.set_identities(init, id_resolver, visibility_resolver)?;
                }
                self.set_identities(statement, id_resolver, visibility_resolver)?;

                self.end_block(id_resolver);
            }
            JodinNodeType::NamedValue { name: _, var_type } => {
                self.resolve_type(var_type, id_resolver, visibility_resolver)?;
            }
            other => {
                for child in other.children_mut() {
                    self.set_identities(child, id_resolver, visibility_resolver)?;
                }
            }
        }
        Ok(())
    }

    fn try_get_absolute_identifier(
        &self,
        id: &Identifier,
        id_resolver: &IdentifierResolver,
        visibility: &Registry<Visibility>,
    ) -> JodinResult<Identifier> {
        let ref id_with_base = *id;
        info!("Attempting to find absolute path {:?}", id);
        // first get alias if it exist
        let alias = self
            .aliases
            .get(id_with_base)
            .ok()
            .filter(|&alias_id| {
                let visibility = visibility.get(alias_id).ok();
                info!(
                    "Found alias {:?} with visibility {:?}",
                    alias_id, visibility
                );
                match visibility {
                    None => true,
                    Some(visibility) => {
                        if visibility.is_visible(alias_id, &id_resolver.current_namespace()) {
                            true
                        } else {
                            warn!(
                                "Path {:?} ({:?}) is not visible from {:?}",
                                alias_id,
                                visibility,
                                id_resolver.current_namespace()
                            );
                            false
                        }
                    }
                }
            })
            .cloned();
        let as_normal = id_resolver
            .resolve_path(id.clone(), false)
            .ok()
            .filter(|resolved| {
                let visibility = visibility.get(resolved).ok();
                info!("Found path {:?} with visibility {:?}", resolved, visibility);
                match visibility {
                    None => true,
                    Some(visibility) => {
                        if visibility.is_visible(resolved, &id_resolver.current_namespace()) {
                            true
                        } else {
                            warn!(
                                "Path {:?} ({:?}) is not visible from {:?}",
                                resolved,
                                visibility,
                                id_resolver.current_namespace()
                            );
                            false
                        }
                    }
                }
            });
        debug!(
            "Found {:?} while trying to find identifier {:?}",
            (&alias, &as_normal),
            id
        );
        match (alias, as_normal) {
            (Some(alias), None) => Ok(alias),
            (None, Some(as_normal)) => Ok(as_normal),
            (Some(a), Some(n)) => Err(JodinErrorType::AmbiguousIdentifierError {
                given: id.clone(),
                found: vec![a, n],
            }
            .into()),
            (None, None) => {
                error!(
                    "{id} does not exist as either an alias or as a direct path",
                    id = id
                );
                Err(JodinErrorType::IdentifierDoesNotExist(id.clone()).into())
            }
        }
    }

    /// Add imports from an import data, returning a list of created identifiers
    fn add_import_data(
        &mut self,
        import: &Import,
        id_resolver: &IdentifierResolver,
        visibility: &Registry<Visibility>,
    ) -> JodinResult<Vec<Identifier>> {
        trace!("import base = {}", import.id());
        let mut aliases = vec![];
        let resolved = &id_resolver.resolve_path(import.id().clone(), true)?;
        let current = id_resolver.current_namespace();
        if !identifier_is_visible_from(&current, resolved, visibility)? {
            return Err(JodinErrorType::IdentifierProtected {
                target: import.id().clone(),
                origin_namespace: current.strip_highest_parent().unwrap(),
            }
            .into());
        }

        match import.import_type() {
            ImportType::Direct => {
                self.aliases
                    .insert_with_identifier(resolved.clone(), &current + &resolved.this_as_id())?;
                aliases.push(current + resolved.this_as_id());
            }
            ImportType::Aliased { alias } => {
                self.aliases
                    .insert_with_identifier(resolved.clone(), &current + alias)?;
                aliases.push(&current + alias);
            }
            ImportType::Wildcard => {
                let tree = id_resolver.namespace_tree();
                let path = resolved.clone();
                let relevant = tree.get_relevant_objects(&path).ok_or(JodinError::from(
                    JodinErrorType::IdentifierDoesNotExist(path),
                ))?;
                for relevant_object in relevant {
                    let target = relevant_object.clone();
                    trace!(
                        "Checking if {} is visible from {} for wildcard",
                        target,
                        current
                    );
                    if identifier_is_visible_from(&current, &target, visibility)? {
                        /*
                        return Err(JodinErrorType::IdentifierProtected {
                            target: import.id().clone(),
                            origin_namespace: current.strip_highest_parent().unwrap(),
                        }
                        .into());



                         */

                        let alias = relevant_object.this_as_id();
                        trace!("Found in wildcard: {}", alias);
                        self.aliases
                            .insert_with_identifier(target.clone(), &current + &alias)?;
                        aliases.push(&current + &alias);
                    }
                }
            }
            ImportType::Children { children } => {
                for child in children {
                    let true_child_import = child
                        .concat_parent_to_id(&resolved.clone().strip_highest_parent().unwrap());
                    let imports =
                        self.add_import_data(&true_child_import, id_resolver, visibility)?;
                    aliases.extend(imports);
                }
            }
        }
        info!("Imported {:?}", aliases);
        Ok(aliases)
    }

    fn start_block(&mut self, id_resolver: &mut IdentifierResolver) {
        let block_num = self.get_block_num();
        let string = Identifier::from(format!("{{block {}}}", block_num));
        let last = id_resolver.current_namespace();
        self.block_num.push(0);

        id_resolver.push_namespace(string.clone());
        //id_resolver.create_absolute_path(&Identifier::from(""));
        if !last.is_empty() {
            id_resolver.use_namespace(last).unwrap();
        }
    }

    fn end_block(&mut self, id_resolver: &mut IdentifierResolver) {
        id_resolver.pop_namespace();
        self.block_num.pop();
        let current = id_resolver.current_namespace();
        if !current.is_empty() {
            id_resolver.stop_use_namespace(&current).unwrap();
        }
    }

    fn get_block_num(&mut self) -> usize {
        let block_num = self.block_num.last_mut().unwrap();
        let ret = *block_num;
        *block_num += 1;
        ret
    }

    /// Attempt to resolve identifiers within an intermediate type
    pub fn resolve_type(
        &mut self,
        my_type: &mut IntermediateType,
        id_resolver: &IdentifierResolver,
        visibility: &Registry<Visibility>,
    ) -> JodinResult<()> {
        if let TypeSpecifier::Id(ref mut identifier) = my_type.type_specifier {
            let resolved = self.try_get_absolute_identifier(identifier, id_resolver, visibility)?;
            *identifier = resolved;
        }

        for generic in &mut my_type.generics {
            self.resolve_type(generic, id_resolver, visibility)?;
        }

        for tail in &mut my_type.tails {
            match tail {
                TypeTail::Pointer => {}
                TypeTail::Array(_) => {}
                TypeTail::Function(args) => {
                    for arg in args {
                        self.resolve_type(arg, id_resolver, visibility)?;
                    }
                }
            }
        }

        Ok(())
    }
}

/// Check whether an identifier can be see from the original namespace. Both origin and destination
/// should be absolute paths.
pub fn identifier_is_visible_from(
    origin_namespace: &Identifier,
    target: &Identifier,
    visibility: &Registry<Visibility>,
) -> JodinResult<bool> {
    println!(
        "Checking if {} is visible from {}",
        target, origin_namespace
    );
    if target.iter().count() == 0 {
        return Err(JodinErrorType::IdentifierDoesNotExist(target.clone()).into());
    }
    let mut target_iter = target.iter();
    let mut target = Identifier::from(target_iter.next().unwrap());

    loop {
        let target_visibility = visibility.get(&target)?;
        println!("Visibility of target {} is {:?}", target, target_visibility);

        match target_visibility {
            Visibility::Public => {
                if let Some(next) = target_iter.next() {
                    target = target + Identifier::from(next);
                } else {
                    break;
                }
            }
            Visibility::Protected => {
                let target_parent = target.parent().unwrap();
                let comparison = target_parent.partial_cmp(origin_namespace);
                println!(
                    "Comparison of {} and {} = {:?}",
                    target_parent, origin_namespace, comparison
                );

                match comparison {
                    Some(Ordering::Greater) => {
                        if let Some(next) = target_iter.next() {
                            target = target + Identifier::from(next);
                        } else {
                            break;
                        }
                    }
                    _ => return Ok(false),
                }
            }
            Visibility::Private => return Ok(false),
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use crate::core::error::JodinResult;

    #[test]
    fn label_structure_members() -> JodinResult<()> {
        Ok(())
    }
}
