use crate::core::types::{JodinTypeReference, Type};

/// An attribute is an addition bit of information that can be attached to
/// an ast node
pub trait Tag {
    /// Gets the type of the attribute based on the name
    fn tag_type(&self) -> String;
    fn tag_info(&self) -> String {
        format!("{:?}", self.tag_type())
    }
    fn max_of_this_tag(&self) -> i32;

    fn is_attribute(&self, other: &str) -> bool {
        self.tag_type() == other
    }
}

pub struct TypeTag {
    jodin_type: JodinTypeReference,
}

impl Tag for TypeTag {
    fn tag_type(&self) -> String {
        "Type".to_string()
    }

    fn tag_info(&self) -> String {
        format!("[{}]", self.jodin_type.borrow().type_name())
    }

    fn max_of_this_tag(&self) -> i32 {
        1
    }
}
