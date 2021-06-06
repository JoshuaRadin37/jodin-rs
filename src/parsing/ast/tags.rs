use std::any::Any;

use crate::core::error::{JodinErrorType, JodinResult};
use crate::core::types::{JodinTypeReference, Type};

/// An attribute is an addition bit of information that can be attached to
/// an ast node
pub trait Tag {
    /// Gets the type of the attribute based on the name
    fn tag_type(&self) -> String;

    fn tag_info(&self) -> String {
        format!("{:?}", self.tag_type())
    }
    fn max_of_this_tag(&self) -> u32;

    fn is_tag(&self, other: &str) -> bool {
        self.tag_type() == other
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait TagUtilities {
    fn as_tag_type<T: 'static + Tag>(&self) -> JodinResult<&T>;
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

#[cfg(test)]
pub(crate) struct DummyTag;

#[cfg(test)]
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

#[cfg(test)]
mod tests {
    use crate::parsing::ast::tags::{DummyTag, Tag, TagUtilities};
    use crate::passes::analysis::identity_resolution_tool::BlockIdentifier;

    #[test]
    fn dynamic_tag_typing() {
        let tag: Box<dyn Tag> = Box::new(DummyTag);
        assert!(tag.as_tag_type::<DummyTag>().is_ok());
        assert!(tag.as_tag_type::<BlockIdentifier>().is_err());
    }
}
