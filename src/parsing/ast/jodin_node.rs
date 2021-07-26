use crate::core::error::{JodinErrorType, JodinResult};
use crate::parsing::ast::node_type::JodinNodeInner;
use crate::parsing::ast::tags::{Tag, TagUtilities};
use crate::utility::Tree;

use std::fmt::{Debug, Formatter};

pub struct JodinNode {
    jodin_node_type: Box<JodinNodeInner>,
    tags: Vec<Box<dyn 'static + Tag>>,
}

impl JodinNode {
    pub fn new(jodin_node_type: JodinNodeInner) -> Self {
        JodinNode {
            jodin_node_type: Box::new(jodin_node_type),
            tags: vec![],
        }
    }

    pub fn into_inner(self) -> JodinNodeInner {
        *self.jodin_node_type
    }
    pub fn inner(&self) -> &JodinNodeInner {
        &*self.jodin_node_type
    }

    pub fn inner_mut(&mut self) -> &mut JodinNodeInner {
        &mut *self.jodin_node_type
    }

    pub fn add_tag<T: 'static + Tag>(&mut self, tag: T) -> JodinResult<()> {
        if self.get_tags_by_type(&*tag.tag_type()).len() as u32 == tag.max_of_this_tag() {
            return Err(JodinErrorType::MaxNumOfTagExceeded {
                tag_type: tag.tag_type(),
                max: tag.max_of_this_tag(),
            }
            .into());
        }

        self.tags.push(Box::new(tag));
        Ok(())
    }

    pub fn add_boxed_tags<I: IntoIterator<Item = Box<dyn Tag>>>(
        &mut self,
        iter: I,
    ) -> JodinResult<()> {
        for tag in iter.into_iter() {
            if self.get_tags_by_type(&*tag.tag_type()).len() as u32 == tag.max_of_this_tag() {
                return Err(JodinErrorType::MaxNumOfTagExceeded {
                    tag_type: tag.tag_type(),
                    max: tag.max_of_this_tag(),
                }
                .into());
            }
            self.tags.push(tag);
        }
        Ok(())
    }

    /// Gets the first tag it finds for a tag type
    pub fn get_tag_by_type(&self, tag_type: &str) -> JodinResult<&dyn Tag> {
        self.get_tags_by_type(tag_type)
            .into_iter()
            .nth(0)
            .ok_or(JodinErrorType::TagNotPresent.into())
    }

    pub fn get_tags_by_type(&self, tag_type: &str) -> Vec<&dyn Tag> {
        self.tags
            .iter()
            .filter(|tag| tag.is_tag(tag_type))
            .map(|tag| &**tag)
            .collect()
    }

    /// Gets the first tag it finds for a tag type
    pub fn get_tag_mut_by_type(&mut self, tag_type: &str) -> Option<&mut Box<dyn Tag>> {
        self.get_tags_mut_by_type(tag_type).into_iter().next()
    }

    pub fn get_tags_mut_by_type(&mut self, tag_type: &str) -> Vec<&mut Box<dyn Tag>> {
        self.tags
            .iter_mut()
            .filter(|tag| tag.is_tag(tag_type))
            .collect()
    }

    pub fn get_tag<T: 'static + Tag>(&self) -> JodinResult<&T> {
        self.get_tags::<T>()
            .into_iter()
            .nth(0)
            .ok_or(JodinErrorType::TagNotPresent.into())
    }

    pub fn get_tags<T: 'static + Tag>(&self) -> Vec<&T> {
        self.tags
            .iter()
            .filter_map(|tag| tag.as_tag_type::<T>().ok())
            .collect()
    }

    pub fn get_tag_mut<T: 'static + Tag>(&mut self) -> Option<&mut T> {
        self.get_tags_mut::<T>().into_iter().nth(0)
    }

    pub fn get_tags_mut<T: 'static + Tag>(&mut self) -> Vec<&mut T> {
        self.tags
            .iter_mut()
            .filter_map(|tag| tag.as_tag_type_mut::<T>().ok())
            .collect()
    }

    pub fn tags(&self) -> &Vec<Box<dyn Tag>> {
        &self.tags
    }
}

impl Debug for JodinNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut node_type = format!("{:?}", self.jodin_node_type);
        if node_type.contains(" ") {
            node_type = node_type[..node_type.find(" ").unwrap()].to_string();
        }

        if f.alternate() {
            write!(
                f,
                "JodinNode {{\n\tattributes: {:?}\n\tinner: {:#?}\n}}",
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

impl Tree for JodinNode {
    fn direct_children(&self) -> Vec<&Self> {
        self.inner().children().into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::identifier::Identifier;
    use crate::parsing::ast::jodin_node::JodinNode;
    use crate::parsing::ast::node_type::JodinNodeInner;
    use crate::parsing::ast::tags::DummyTag;
    use crate::passes::analysis::identity_resolution_tool::{BlockIdentifier, ResolvedIdentityTag};

    #[test]
    fn get_tags_of_type() {
        let mut node = JodinNode::new(JodinNodeInner::Identifier(Identifier::from("hi")));
        node.add_tag(DummyTag);
        node.add_tag(BlockIdentifier::new(5));
        node.add_tag(DummyTag);
        assert_eq!(node.tags().len(), 3);
        let id_tag = node.get_tag::<BlockIdentifier>().unwrap();
        assert_eq!(id_tag.block_num(), 5);
        let dummy_tags = node.get_tags::<DummyTag>();
        assert_eq!(dummy_tags.len(), 2);
        assert!(node.get_tag::<ResolvedIdentityTag>().is_err());
    }
}
