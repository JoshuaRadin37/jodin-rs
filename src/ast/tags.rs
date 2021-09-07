//! The tag system allows for a much more modular AST system where more metadata can be added to
//! individual nodes with fewer restrictions.

use std::any::Any;

use crate::core::error::{JodinErrorType, JodinResult};

/// An attribute is an addition bit of information that can be attached to
/// an ast node
pub trait Tag {
    /// Gets the type of the attribute based on the name
    fn tag_type(&self) -> String;

    /// Gets debug information about the tag.
    fn tag_info(&self) -> String {
        self.tag_type()
    }
    /// The maximum allowed number of tags of this type on a single node.
    fn max_of_this_tag(&self) -> u32;

    /// Check if this tag is of a tag type.
    fn is_tag(&self, other: &str) -> bool {
        self.tag_type() == other
    }

    /// Cast this tag to an Any reference.
    fn as_any(&self) -> &dyn Any;

    /// Cast this tag to a mutable Any reference.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A helper trait to add common methods for all tags.
pub trait TagUtilities {
    /// Try to get this tag as a certain type.
    fn as_tag_type<T: 'static + Tag>(&self) -> JodinResult<&T>;
    /// Try to get this tag as a certain type, but it's a mutable reference.
    fn as_tag_type_mut<T: 'static + Tag>(&mut self) -> JodinResult<&mut T>;
}

impl TagUtilities for Box<dyn Tag> {
    fn as_tag_type<T: 'static + Tag>(&self) -> JodinResult<&T> {
        let boxed_any: &dyn Any = self.as_any();
        boxed_any
            .downcast_ref::<T>()
            .ok_or(JodinErrorType::TagCastError.into())
    }

    fn as_tag_type_mut<T: 'static + Tag>(&mut self) -> JodinResult<&mut T> {
        let boxed_any: &mut dyn Any = self.as_any_mut();
        boxed_any
            .downcast_mut::<T>()
            .ok_or(JodinErrorType::TagCastError.into())
    }
}

#[doc(hidden)]
pub struct DummyTag;

impl Tag for DummyTag {
    fn tag_type(&self) -> String {
        "dummy".to_string()
    }

    fn max_of_this_tag(&self) -> u32 {
        u32::MAX
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Marks that this statement is labeled
#[derive(Debug)]
pub struct LabeledStatementTag {
    /// The label
    pub label: String
}

impl Tag for LabeledStatementTag {
    fn tag_type(&self) -> String {
        "labeled_statement".to_string()
    }

    fn max_of_this_tag(&self) -> u32 {
        u32::MAX
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
       self
    }
}

impl LabeledStatementTag {
    /// Create a new labeled statement tag
    pub fn new(label: String) -> Self {
        LabeledStatementTag { label }
    }
}



#[cfg(test)]
mod tests {
    use crate::ast::tags::{DummyTag, Tag, TagUtilities};
    use crate::passes::analysis::BlockIdentifierTag;

    #[test]
    fn dynamic_tag_typing() {
        let tag: Box<dyn Tag> = Box::new(DummyTag);
        assert!(tag.as_tag_type::<DummyTag>().is_ok());
        assert!(tag.as_tag_type::<BlockIdentifierTag>().is_err());
    }
}
