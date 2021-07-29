//! The main building block for the Abstract Syntax Tree

use crate::ast::node_type::JodinNodeInner;
use crate::ast::tags::{Tag, TagUtilities};
use crate::core::error::{JodinErrorType, JodinResult};
use crate::utility::Tree;

use std::fmt::{Debug, Formatter};

/// Contains the inner jodin node type and it's tags
pub struct JodinNode {
    jodin_node_type: Box<JodinNodeInner>,
    tags: Vec<Box<dyn 'static + Tag>>,
}

impl JodinNode {
    /// Create a new `JodinNode` from an inner type.
    pub fn new(jodin_node_type: JodinNodeInner) -> Self {
        JodinNode {
            jodin_node_type: Box::new(jodin_node_type),
            tags: vec![],
        }
    }

    /// Consume the JodinNode to get it's inner type.
    pub fn into_inner(self) -> JodinNodeInner {
        *self.jodin_node_type
    }

    /// The inner type of the node.
    pub fn inner(&self) -> &JodinNodeInner {
        &*self.jodin_node_type
    }

    /// A mutable reference to the inner type of the node.
    pub fn inner_mut(&mut self) -> &mut JodinNodeInner {
        &mut *self.jodin_node_type
    }

    /// Add a tag to the jodin node.
    ///
    /// # Arguments
    ///
    /// * `tag`: The tag to add to the node
    ///
    /// returns: Result<(), JodinError> Whether the tag was added successfully.
    ///
    /// # Error
    ///
    /// Will return an error if the maximum amount of tags of this type were already added to this node.
    ///
    /// # Examples
    ///
    /// ```
    /// use jodin_rs::ast::JodinNode;
    /// use jodin_rs::ast::JodinNodeInner;
    /// use jodin_rs::core::identifier::Identifier;
    /// use jodin_rs::passes::analysis::identity_resolution::ResolvedIdentityTag;
    /// use jodin_rs::passes::analysis::ResolvedIdentityTag;
    /// let mut node = JodinNode::new(JodinNodeInner::Identifier(Identifier::from("id")));
    /// node.add_tag(ResolvedIdentityTag::new(Identifier::from_array(["namespace", "id"]))).unwrap();
    /// ```
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

    /// Adds an iterator of tags to the jodin node.
    ///
    /// These tags must be boxed so that they can be of different tag types. Otherwise, only one type
    /// of tags can be added.
    ///
    /// # Arguments
    ///
    /// * `iter`: The iterator of tags
    ///
    /// returns: Result<(), JodinError> Whether the tags were all added successfully.
    ///
    /// # Error
    ///
    /// Will error out if any tag fails to be added. Will not revert to before operation was completed, but will
    /// not also add any more tags after a failure occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// use jodin_rs::ast::JodinNode;
    /// use jodin_rs::ast::JodinNodeInner;
    /// use jodin_rs::core::identifier::Identifier;
    /// use jodin_rs::ast::tags::DummyTag;
    /// let mut node = JodinNode::new(JodinNodeInner::Identifier(Identifier::from("id")));
    /// node.add_boxed_tags(
    ///     vec![Box::new(DummyTag), Box::new(DummyTag)]
    /// ).unwrap();
    /// ```
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

    /// Gets all tags for a certain tag type
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

    /// Gets all tags for a certain tag type
    pub fn get_tags_mut_by_type(&mut self, tag_type: &str) -> Vec<&mut Box<dyn Tag>> {
        self.tags
            .iter_mut()
            .filter(|tag| tag.is_tag(tag_type))
            .collect()
    }

    /// Get a tag using the type of the tag.
    ///
    /// # Example
    ///
    /// ```
    /// use jodin_rs::ast::JodinNode;
    /// use jodin_rs::ast::JodinNodeInner;
    /// use jodin_rs::core::identifier::Identifier;
    /// use jodin_rs::passes::analysis::identity_resolution::ResolvedIdentityTag;
    /// use jodin_rs::ast::tags::DummyTag;
    /// let mut node = JodinNode::new(JodinNodeInner::Identifier(Identifier::from("id")));
    /// node.add_tag(DummyTag);
    ///
    /// node.get_tag::<DummyTag>().unwrap();
    /// let tag: &DummyTag = node.get_tag().unwrap();
    /// ```
    pub fn get_tag<T: 'static + Tag>(&self) -> JodinResult<&T> {
        self.get_tags::<T>()
            .into_iter()
            .nth(0)
            .ok_or(JodinErrorType::TagNotPresent.into())
    }

    /// Get all tags using the type of the tag.
    ///
    /// # Example
    ///
    /// ```
    /// use jodin_rs::ast::JodinNode;
    /// use jodin_rs::ast::JodinNodeInner;
    /// use jodin_rs::core::identifier::Identifier;
    /// use jodin_rs::passes::analysis::identity_resolution::ResolvedIdentityTag;
    /// use jodin_rs::ast::tags::DummyTag;
    /// let mut node = JodinNode::new(JodinNodeInner::Identifier(Identifier::from("id")));
    /// node.add_tag(DummyTag);
    ///
    /// let tags: Vec<&DummyTag> = node.get_tags();
    pub fn get_tags<T: 'static + Tag>(&self) -> Vec<&T> {
        self.tags
            .iter()
            .filter_map(|tag| tag.as_tag_type::<T>().ok())
            .collect()
    }

    /// Get a mutable tag reference using the type of the tag.
    ///
    /// # Example
    ///
    /// ```
    /// use jodin_rs::ast::JodinNode;
    /// use jodin_rs::ast::JodinNodeInner;
    /// use jodin_rs::core::identifier::Identifier;
    /// use jodin_rs::passes::analysis::identity_resolution::ResolvedIdentityTag;
    /// use jodin_rs::ast::tags::DummyTag;
    /// let mut node = JodinNode::new(JodinNodeInner::Identifier(Identifier::from("id")));
    /// node.add_tag(DummyTag);
    ///
    /// let tag: &mut DummyTag = node.get_tag_mut().unwrap();
    /// ```
    pub fn get_tag_mut<T: 'static + Tag>(&mut self) -> Option<&mut T> {
        self.get_tags_mut::<T>().into_iter().nth(0)
    }

    /// Get all mutable tag references using the type of the tag.
    ///
    /// # Example
    ///
    /// ```
    /// use jodin_rs::ast::JodinNode;
    /// use jodin_rs::ast::JodinNodeInner;
    /// use jodin_rs::core::identifier::Identifier;
    /// use jodin_rs::passes::analysis::identity_resolution::ResolvedIdentityTag;
    /// use jodin_rs::ast::tags::DummyTag;
    /// let mut node = JodinNode::new(JodinNodeInner::Identifier(Identifier::from("id")));
    /// node.add_tag(DummyTag);
    ///
    /// let tags: Vec<&mut DummyTag> = node.get_tags_mut();
    pub fn get_tags_mut<T: 'static + Tag>(&mut self) -> Vec<&mut T> {
        self.tags
            .iter_mut()
            .filter_map(|tag| tag.as_tag_type_mut::<T>().ok())
            .collect()
    }

    /// Gets all of the tags within the node.
    pub fn tags(&self) -> &Vec<Box<dyn Tag>> {
        &self.tags
    }

    /// Creates an empty JodinNode
    pub fn empty() -> Self {
        JodinNodeInner::Empty.into()
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

impl<'a> IntoIterator for &'a JodinNode {
    type Item = &'a JodinNode;
    type IntoIter = <Vec<&'a JodinNode> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner()
            .children()
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::jodin_node::JodinNode;
    use crate::ast::node_type::JodinNodeInner;
    use crate::ast::tags::DummyTag;
    use crate::core::identifier::Identifier;
    use crate::passes::analysis::{BlockIdentifierTag, ResolvedIdentityTag};

    #[test]
    fn get_tags_of_type() {
        let mut node = JodinNode::new(JodinNodeInner::Identifier(Identifier::from("hi")));
        node.add_tag(DummyTag);
        node.add_tag(BlockIdentifierTag::new(5));
        node.add_tag(DummyTag);
        assert_eq!(node.tags().len(), 3);
        let id_tag = node.get_tag::<BlockIdentifierTag>().unwrap();
        assert_eq!(id_tag.block_num(), 5);
        let dummy_tags = node.get_tags::<DummyTag>();
        assert_eq!(dummy_tags.len(), 2);
        assert!(node.get_tag::<ResolvedIdentityTag>().is_err());
    }
}
