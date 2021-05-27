use crate::parsing::ast::node_type::JodinNodeInner;
use crate::parsing::ast::tags::Tag;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

pub struct JodinNode {
    jodin_node_type: Box<JodinNodeInner>,
    attributes: Vec<Box<dyn Tag>>,
}

impl JodinNode {
    pub fn new(jodin_node_type: JodinNodeInner) -> Self {
        JodinNode {
            jodin_node_type: Box::new(jodin_node_type),
            attributes: vec![],
        }
    }

    pub fn inner(&self) -> &JodinNodeInner {
        &*self.jodin_node_type
    }
}

impl Debug for JodinNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "JodinNode {{\nattributes: {:?}\ninner: {:?}\n}}",
                self.attributes
                    .iter()
                    .map(|a| a.tag_info())
                    .collect::<Vec<_>>(),
                self.jodin_node_type,
            )
        } else {
            write!(
                f,
                "JodinNode {{ attributes: {:?} inner: {:?} }}",
                self.attributes
                    .iter()
                    .map(|a| a.tag_info())
                    .collect::<Vec<_>>(),
                self.jodin_node_type,
            )
        }
    }
}
