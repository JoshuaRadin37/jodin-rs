//! Responsible for managing types and translations from intermediate types.
//!
//! Used to determine type checking.

use crate::ast::{JodinNode, NodeReference};
use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use crate::core::identifier::{Identifier, IdentifierChain, IdentifierChainIterator};
use crate::core::types::base_type::base_type;
use crate::core::types::big_object::{JBigObject, JBigObjectBuilder};
use crate::core::types::intermediate_type::{IntermediateType, TypeSpecifier, TypeTail};
use crate::core::types::primitives::Primitive;
use crate::core::types::traits::JTrait;
use crate::core::types::{AsIntermediate, BuildType, JodinType, Type};
use crate::passes::analysis::ResolvedIdentityTag;
use crate::utility::Visitor;
use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, Index};
use std::sync::Arc;

/// Stores a lot of information about types and related identifier
#[derive(Debug)]
pub struct TypeEnvironment {
    types: HashMap<Identifier, TypeInfo>,
    symbol_to_type: HashMap<Identifier, IntermediateType>,
    base_type_id: Identifier,
    impl_types_to_trait_obj: HashMap<Vec<Identifier>, Identifier>,
    tlb: HashMap<IdentifierChain, Identifier>,
}

/// A type environment with no references
pub struct MinimalTypeEnvironment {

}

#[derive(Debug)]
pub struct TypeInfo {
    /// The actual jodin type
    pub jtype: JodinType,
    /// The declaring node (if relevant)
    pub decl_node: Option<NodeReference>,
}

impl TypeInfo {
    pub fn new(jtype: JodinType, decl_node: Option<&JodinNode>) -> Self {
        TypeInfo { jtype, decl_node: decl_node.map(|node| node.get_reference()) }
    }
}

/// A trait to define a way of getting a type from some sort of index
pub trait GetType<Idx: Eq + Hash> {
    /// Get a type from some type environment
    fn get_type(&self, index: &Idx) -> JodinResult<IntermediateType>;
}

impl TypeEnvironment {
    /// Create a new type environment
    pub fn new() -> Self {
        let mut output = Self {
            types: Default::default(),
            symbol_to_type: Default::default(),
            base_type_id: Identifier::empty(),
            impl_types_to_trait_obj: Default::default(),
            tlb: Default::default(),
        };

        let base_type = base_type().expect("Creating base type failed");
        output.set_base_type(base_type);

        output
    }

    /// Gets the universal type of the environment, meaning that every type should be equivalent to
    /// this. Currently, this is just the base type trait.
    pub fn universal_type(&self) -> IntermediateType {
        self.base_type().as_intermediate()
    }

    /// Checks whether the first argument can be considered the second type
    ///
    /// # Notable checks for type safety
    /// 1. void* is everything
    pub fn loosely_is(&self, my_type: &IntermediateType, target_type: &IntermediateType) -> bool {
        if Self::is_void_ptr(target_type) && Self::is_ptr(my_type) {
            return true;
        }

        if Self::is_ptr(my_type) && Self::is_ptr(target_type) {
            return self.loosely_is(
                &my_type.get_deref().unwrap(),
                &target_type.get_deref().unwrap(),
            );
        }

        false
    }

    /// Gets whether this a void*
    pub fn is_void_ptr(inter: &IntermediateType) -> bool {
        if let IntermediateType {
            is_const: false,
            type_specifier: TypeSpecifier::Primitive(Primitive::Void),
            generics,
            tails,
        } = inter
        {
            generics.is_empty() && &*tails == &[TypeTail::Pointer]
        } else {
            false
        }
    }

    /// Whether this a pointer
    pub fn is_ptr(inter: &IntermediateType) -> bool {
        match inter.tails.last() {
            Some(TypeTail::Pointer) => true,
            Some(TypeTail::Array(_)) => true,
            _ => false,
        }
    }

    pub fn base_type(&self) -> &JodinType {
        self.get_type_by_name(&self.base_type_id)
            .expect("The base type should always be available")
    }

    pub fn get_type_by_name(&self, name: &Identifier) -> JodinResult<&JodinType> {
        self.types
            .get(name)
            .map(|info| &info.jtype)
            .ok_or(JodinErrorType::IdentifierDoesNotExist(name.clone()).into())
    }

    fn set_base_type<T: Into<JodinType>>(&mut self, base_type: T) {
        let base_type = base_type.into();
        let id = base_type.type_identifier();
        self.add(base_type, None)
            .expect("Should not be adding the base type multiple times");
        self.base_type_id = id;
    }

    pub fn is_child_type(&self, child: &Identifier, parent: &Identifier) -> bool {
        todo!()
    }

    pub fn big_object_builder<'t>(&'t self, jtype: &'t JodinType) -> JBigObjectBuilder<'t> {
        JBigObjectBuilder::new(jtype, self)
    }

    /// Adds a jodin type declaration into the environment
    pub fn add<'t, T: Type<'t>>(
        &mut self,
        jty: T,
        node: Option<&JodinNode>,
    ) -> JodinResult<()> {
        let jtype: JodinType = jty.into();
        let type_info = TypeInfo::new(jtype, node);
        match self
            .types
            .insert(type_info.jtype.type_identifier(), type_info)
        {
            None => Ok(()),
            Some(old) => {
                Err(JodinErrorType::IdentifierAlreadyExists(old.jtype.type_identifier()).into())
            }
        }
    }

    pub fn set_variable_type<T: AsIntermediate, I: Into<Identifier>>(
        &mut self,
        var: I,
        jty: T,
    ) -> JodinResult<()> {
        self.symbol_to_type
            .insert(var.into(), jty.intermediate_type());
        Ok(())
    }

    pub fn variable_type<I: Into<Identifier>>(&self, id: I) -> JodinResult<&IntermediateType> {
        let identifier = id.into();
        self.symbol_to_type
            .get(&identifier)
            .ok_or(JodinErrorType::IdentifierDoesNotExist(identifier).into())
    }
}

pub struct TypeEnvironmentManager {
    env: TypeEnvironment,
}

impl TypeEnvironmentManager {
    /// Create a new manager
    pub fn new() -> Self {
        TypeEnvironmentManager {
            env: TypeEnvironment::new(),
        }
    }

    /// Create a new manager using a pre-built environment
    pub fn with_env(env: TypeEnvironment) -> Self {
        Self { env }
    }

    /// Finishes the manager and returns the environment
    pub fn finish(self) -> TypeEnvironment {
        self.env
    }

    /// Create a type from some jodin node
    pub fn create_type<'t, T: BuildType<'t>>(&self, node: &JodinNode) -> JodinResult<T> {
        T::build_type(node, &self.env, None)
    }

    /// Save the type in the environment
    pub fn store_type<'t, T: Type<'t>>(
        &mut self,
        ty: T,
        node: Option<&JodinNode>,
    ) -> JodinResult<()> {
        self.env.add(ty, node)
    }

    /// Load a type from the environment
    pub fn load_type(&self, identifier: &Identifier) -> JodinResult<&JodinType> {
        self.env.get_type_by_name(identifier)
    }

    /// Set the type for some variable
    pub fn set_variable_type<T: AsIntermediate>(&mut self, var_id: &Identifier, ty: T) {
        self._set_variable_type(var_id, ty.intermediate_type())
    }

    fn _set_variable_type(&mut self, var_id: &Identifier, ty: IntermediateType) {}

    /// Loads the big object version of some variable
    pub fn load_variable_type(&self, var_id: &Identifier) -> JodinResult<JBigObject> {
        todo!()
    }
}
