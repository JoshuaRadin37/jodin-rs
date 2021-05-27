use crate::core::error::{JodinError, JodinResult};
use crate::parsing::ast::node_type::JodinNodeInner;
use crate::parsing::ast::tags::Tag;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

pub struct JodinNode {
    jodin_node_type: Box<JodinNodeInner>,
    tags: Vec<Box<dyn Tag>>,
}

impl JodinNode {
    pub fn new(jodin_node_type: JodinNodeInner) -> Self {
        JodinNode {
            jodin_node_type: Box::new(jodin_node_type),
            tags: vec![],
        }
    }

    pub fn inner(&self) -> &JodinNodeInner {
        &*self.jodin_node_type
    }

    pub fn inner_mut(&mut self) -> &mut JodinNodeInner {
        &mut *self.jodin_node_type
    }

    pub fn add_tag<T: 'static + Tag>(&mut self, tag: T) -> JodinResult<()> {
        if self.get_tags(&*tag.tag_type()).len() as u32 == tag.max_of_this_tag() {
            return Err(JodinError::MaxNumOfTagExceeded {
                tag_type: tag.tag_type(),
                max: tag.max_of_this_tag(),
            });
        }

        self.tags.push(Box::new(tag));
        Ok(())
    }

    /// Gets the first tag it finds for a tag type
    pub fn get_tag(&self, tag_type: &str) -> Option<&dyn Tag> {
        self.get_tags(tag_type).first().map(|t| *t)
    }

    pub fn get_tags(&self, tag_type: &str) -> Vec<&dyn Tag> {
        self.tags
            .iter()
            .filter(|tag| tag.is_tag(tag_type))
            .map(|tag| &**tag)
            .collect()
    }

    /// Gets the first tag it finds for a tag type
    pub fn get_tag_mut(&mut self, tag_type: &str) -> Option<&mut Box<dyn Tag>> {
        self.get_tags_mut(tag_type).into_iter().next()
    }

    pub fn get_tags_mut(&mut self, tag_type: &str) -> Vec<&mut Box<dyn Tag>> {
        self.tags
            .iter_mut()
            .filter(|tag| tag.is_tag(tag_type))
            .collect()
    }
}

impl Debug for JodinNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "JodinNode {{\nattributes: {:?}\ninner: {:?}\n}}",
                self.tags.iter().map(|a| a.tag_info()).collect::<Vec<_>>(),
                self.jodin_node_type,
            )
        } else {
            write!(
                f,
                "JodinNode {{ attributes: {:?} inner: {:?} }}",
                self.tags.iter().map(|a| a.tag_info()).collect::<Vec<_>>(),
                self.jodin_node_type,
            )
        }
    }
}
