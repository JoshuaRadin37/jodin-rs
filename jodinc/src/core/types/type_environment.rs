//! Responsible for managing types and translations from intermediate types.
//!
//! Used to determine type checking.

use crate::ast::JodinNode;
use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use crate::core::identifier::{Identifier, IdentifierChain, IdentifierChainIterator};
use crate::core::types::big_object::JBigObjectBuilder;
use crate::core::types::intermediate_type::{IntermediateType, TypeSpecifier, TypeTail};
use crate::core::types::primitives::Primitive;
use crate::core::types::traits::JTrait;
use crate::core::types::{JodinType, Type};
use crate::utility::Visitor;
use std::collections::HashMap;
use std::ops::{Deref, Index};
use std::sync::Arc;

/// Stores a lot of information about types and related identifier
#[derive(Debug)]
pub struct TypeEnvironment<'node> {
    types: HashMap<Identifier, TypeInfo<'node>>,
    base_type_id: Identifier,
    impl_types_to_trait_obj: HashMap<Vec<Identifier>, Identifier>,
}

#[derive(Debug)]
pub struct TypeInfo<'node> {
    /// The actual jodin type
    pub jtype: Arc<JodinType>,
    /// The declaring node (if relevant)
    pub decl_node: Option<&'node JodinNode>,
}

impl<'n> TypeEnvironment<'n> {
    /// Create a new type environment
    pub fn new() -> Self {
        let mut output = Self {
            types: Default::default(),
            base_type_id: Identifier::empty(),
            impl_types_to_trait_obj: Default::default(),
        };

        todo!("Add base type and primitives")
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
        self.get_type(&self.base_type_id)
            .expect("The base type should always be available")
    }

    fn set_base_type(&mut self, base_type: JodinType) {
        let id = base_type.type_name();
        self.add(base_type, None)
            .expect("Should not be adding the base type multiple times");
        self.base_type_id = id;
    }

    pub fn get_type(&self, id: &Identifier) -> JodinResult<&JodinType> {
        self.types
            .get(id)
            .as_ref()
            .map(|info| info.jtype.deref())
            .ok_or(JodinError::new(JodinErrorType::IdentifierDoesNotExist(
                id.clone(),
            )))
    }

    pub fn is_child_type(&self, child: &Identifier, parent: &Identifier) -> bool {
        todo!()
    }

    pub fn big_object_builder<'t>(&self, jtype: &'t JodinType) -> JBigObjectBuilder<'_, 't> {
        JBigObjectBuilder::new(jtype, self)
    }

    /// Adds a jodin type declaration into the environment
    pub fn add<'t, T: Type<'n, 't>>(
        &mut self,
        jty: T,
        node: Option<&'n JodinNode>,
    ) -> JodinResult<()> {
        let jtype: JodinType = jty.into();
        let type_info = TypeInfo {
            jtype: Arc::new(jtype),
            decl_node: node,
        };
        let id = type_info.jtype.type_name();
        match self.types.insert(id, type_info) {
            None => Ok(()),
            Some(old) => Err(JodinErrorType::IdentifierAlreadyExists(old.jtype.type_name()).into()),
        }
    }
}

impl<'jtype> Index<&Identifier> for TypeEnvironment<'jtype> {
    type Output = JodinType;

    fn index(&self, index: &Identifier) -> &Self::Output {
        self.get_type(index).unwrap()
    }
}

impl<'jtype> Index<&IdentifierChain> for TypeEnvironment<'jtype> {
    type Output = ();

    fn index(&self, index: &IdentifierChain) -> &Self::Output {
        todo!()
    }
}
