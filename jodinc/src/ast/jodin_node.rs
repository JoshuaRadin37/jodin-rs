//! The main building block for the Abstract Syntax Tree

use crate::ast::node_type::JodinNodeType;
use crate::ast::tags::{ExtraProperties, Tag, TagUtilities};
use crate::core::error::{JodinErrorType, JodinResult};
use crate::utility::{Acceptor, AcceptorMut, Tree, Visitor};
use std::any::Any;
use std::cell::{RefCell, RefMut};

use num_traits::AsPrimitive;
use std::fmt::{Debug, Formatter, Pointer};
use std::ops::{Deref, Index};
use std::rc::{Rc, Weak};

#[derive(Default)]
struct JodinNodeIndex {
    value: RefCell<usize>,
    parent: RefCell<Option<Weak<JodinNodeIndex>>>,
}

impl Debug for JodinNodeIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Index = {}", self.value.borrow())
    }
}

impl JodinNodeIndex {
    fn directions(&self) -> Vec<usize> {
        let _borrow = self.parent.borrow();
        let p = _borrow.deref();
        let index = *self.value.borrow();
        match p {
            None => {
                vec![index]
            }
            Some(parent) => {
                let parent = parent
                    .upgrade()
                    .expect("Trying to get jodin node after cleanup");
                let mut dirs = parent.directions();
                dirs.push(index);
                dirs
            }
        }
    }
}

/// Contains the inner jodin node type and it's tags
pub struct JodinNode {
    jodin_node_type: Box<JodinNodeType>,
    tags: Vec<Box<dyn 'static + Tag>>,
    index: Rc<JodinNodeIndex>,
}

impl JodinNode {
    /// Create a new `JodinNode` from an inner type.
    pub fn new(jodin_node_type: JodinNodeType) -> Self {
        let mut node = JodinNode {
            jodin_node_type: Box::new(jodin_node_type),
            tags: vec![],
            index: Default::default(),
        };
        node.add_tag(ExtraProperties::new());

        let parent_ptr = node.index.clone();

        let mut index = 0;
        for child in node.direct_children() {
            let child_index_ptr = child.index.clone();
            let parent = Rc::downgrade(&parent_ptr);

            *child_index_ptr.value.borrow_mut() = index;
            *child_index_ptr.parent.borrow_mut() = Some(parent);

            index += 1;
        }

        let tag = NodeReferenceTag::new(&node);
        node.add_tag(tag);

        node
    }

    fn get_directions(&self) -> Vec<usize> {
        self.index.directions()[1..].to_vec()
    }

    /// Consume the JodinNode to get it's inner type.
    pub fn into_inner(self) -> JodinNodeType {
        *self.jodin_node_type
    }

    /// The inner type of the node.
    pub fn inner(&self) -> &JodinNodeType {
        &*self.jodin_node_type
    }

    /// A mutable reference to the inner type of the node.
    pub fn inner_mut(&mut self) -> &mut JodinNodeType {
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
    /// use jodinc:ast::JodinNode;
    /// use jodinc:ast::JodinNodeType;
    /// use jodinc:core::identifier::Identifier;
    /// use jodinc:passes::analysis::ResolvedIdentityTag;
    /// let mut node = JodinNode::new(JodinNodeType::Identifier(Identifier::from("id")));
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
    /// use jodinc:ast::JodinNode;
    /// use jodinc:ast::JodinNodeType;
    /// use jodinc:core::identifier::Identifier;
    /// use jodinc:ast::tags::DummyTag;
    /// let mut node = JodinNode::new(JodinNodeType::Identifier(Identifier::from("id")));
    /// node.add_tag(DummyTag);
    ///
    /// node.get_tag::<DummyTag>().unwrap();
    /// let tag: &DummyTag = node.get_tag().unwrap();
    /// ```
    pub fn get_tag<T: 'static + Tag>(&self) -> JodinResult<&T> {
        trace!(
            "Attempting to get tag of type {}",
            std::any::type_name::<T>()
        );
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
    /// use jodinc:ast::JodinNode;
    /// use jodinc:ast::JodinNodeType;
    /// use jodinc:core::identifier::Identifier;
    /// use jodinc:ast::tags::DummyTag;
    /// let mut node = JodinNode::new(JodinNodeType::Identifier(Identifier::from("id")));
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
    /// use jodinc:ast::JodinNode;
    /// use jodinc:ast::JodinNodeType;
    /// use jodinc:core::identifier::Identifier;
    /// use jodinc:ast::tags::DummyTag;
    /// let mut node = JodinNode::new(JodinNodeType::Identifier(Identifier::from("id")));
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
    /// use jodinc:ast::JodinNode;
    /// use jodinc:ast::JodinNodeType;
    /// use jodinc:core::identifier::Identifier;
    /// use jodinc:ast::tags::DummyTag;
    /// let mut node = JodinNode::new(JodinNodeType::Identifier(Identifier::from("id")));
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
        JodinNodeType::Empty.into()
    }

    /// Returns None if this is empty, or Some(self) if not empty
    pub fn not_empty(self) -> Option<Self> {
        if let JodinNodeType::Empty = self.inner() {
            None
        } else {
            Some(self)
        }
    }

    /// Gets a reference to this node.
    pub fn get_reference(&self) -> NodeReference {
        let tag: &NodeReferenceTag = self.get_tag().expect("Added at startup, should never fail");
        let info = tag.info;
        let directions = self.get_directions();
        NodeReference { directions, info }
    }
}

impl Debug for JodinNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // if f.alternate() {
        //     write!(
        //         f,
        //         "JodinNode {{\n\tattributes: {:?}\n\tinner: {:#?}\n}}",
        //         self.tags.iter().map(|a| a.tag_info()).collect::<Vec<_>>(),
        //         self.jodin_node_type,
        //     )
        // } else {
        //     write!(
        //         f,
        //         "JodinNode {{ attributes: {:?} inner: {:?} }}",
        //         self.tags.iter().map(|a| a.tag_info()).collect::<Vec<_>>(),
        //         self.jodin_node_type,
        //     )
        // }
        f.debug_struct("JodinNode")
            .field("index", &*self.index.value.borrow())
            .field(
                "attributes",
                &self.tags.iter().map(|a| a.tag_info()).collect::<Vec<_>>(),
            )
            .field("inner", &self.jodin_node_type)
            .finish()
    }
}

impl Tree for JodinNode {
    fn direct_children(&self) -> Vec<&Self> {
        self.inner().children().into_iter().collect()
    }
}

impl Index<usize> for JodinNode {
    type Output = JodinNode;

    fn index(&self, index: usize) -> &Self::Output {
        self.direct_children().get(index).unwrap()
    }
}

impl Index<&[usize]> for JodinNode {
    type Output = JodinNode;

    fn index(&self, index: &[usize]) -> &Self::Output {
        let mut iter = index.into_iter();
        let mut ptr = self;
        while let Some(next) = iter.next() {
            ptr = ptr.index(*next);
        }
        ptr
    }
}

impl<const N: usize> Index<&[usize; N]> for JodinNode {
    type Output = JodinNode;

    fn index(&self, index: &[usize; N]) -> &Self::Output {
        let mut iter = index.into_iter();
        let mut ptr = self;
        while let Some(next) = iter.next() {
            ptr = ptr.index(*next);
        }
        ptr
    }
}

impl Index<Vec<usize>> for JodinNode {
    type Output = JodinNode;

    fn index(&self, index: Vec<usize>) -> &Self::Output {
        self.index(index.as_slice())
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

impl From<JodinNodeType> for JodinNode {
    fn from(i: JodinNodeType) -> Self {
        JodinNode::new(i)
    }
}

impl Acceptor<'_, NodeReferenceInfo> for JodinNode {
    type Output = bool;

    fn accept(&self, visitor: NodeReferenceInfo) -> Self::Output {
        match self.get_tag::<NodeReferenceTag>() {
            Ok(tag) => tag.info == visitor,
            Err(_) => false,
        }
    }
}

impl<'n> Acceptor<'n, &NodeReference> for JodinNode {
    type Output = Option<&'n JodinNode>;

    fn accept(&'n self, visitor: &NodeReference) -> Self::Output {
        let mut ptr = self;
        for index in &visitor.directions {
            let children = ptr.direct_children();
            ptr = children.get(*index)?;
        }
        let ptr_info = ptr.get_reference().info;
        if ptr_info == visitor.info {
            Some(ptr)
        } else {
            None
        }
    }
}

impl<'n> Acceptor<'n, NodeReference> for JodinNode {
    type Output = Option<&'n JodinNode>;

    fn accept(&'n self, visitor: NodeReference) -> Self::Output {
        self.accept(&visitor)
    }
}

/// Stores a value to help ensure that a reference is correct
#[derive(Debug)]
pub struct NodeReferenceTag {
    info: NodeReferenceInfo,
}

impl NodeReferenceTag {
    fn new(node: &JodinNode) -> Self {
        let rand_code: u64 = rand::random();
        let hashcode: u64 = format!("{:?}", node)
            .char_indices()
            .map(|(index, ch)| (index as u64 * <char as AsPrimitive<u64>>::as_(ch)))
            .sum();
        let info = NodeReferenceInfo {
            random_code: rand_code,
            checksum: hashcode,
        };
        Self { info }
    }

    fn info(&self) -> &NodeReferenceInfo {
        &self.info
    }
}

impl Tag for NodeReferenceTag {
    fn tag_type(&self) -> String {
        format!("[NodeRef]")
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct NodeReferenceInfo {
    pub random_code: u64,
    pub checksum: u64,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NodeReference {
    directions: Vec<usize>,
    info: NodeReferenceInfo,
}

impl NodeReference {
    /// Get the node referred to by this reference from a tree
    pub fn node<'a>(&self, tree: &'a JodinNode) -> Option<&'a JodinNode> {
        tree.accept(self)
    }
}

/// Creates a node tree of empty nodes as leaves
///
/// # Arguments
///
/// * `depth`: The number of levels within the tree
/// * `children_per`: The amount of children at a tree node
///
/// returns: The constructed node
pub fn node_tree(depth: usize, children_per: usize) -> JodinNode {
    match depth {
        0 => panic!("Can't have a node tree of 0 depth"),
        1 => JodinNode::empty(),
        _more => {
            let mut children = Vec::with_capacity(children_per);
            for _ in 0..children_per {
                children.push(node_tree(depth - 1, children_per));
            }
            JodinNodeType::NodeVector { vec: children }.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::jodin_node::JodinNode;
    use crate::ast::node_type::JodinNodeType;
    use crate::ast::tags::DummyTag;
    use crate::core::identifier::Identifier;
    use crate::passes::analysis::{BlockIdentifierTag, ResolvedIdentityTag};

    #[test]
    fn get_tags_of_type() {
        let mut node = JodinNode::new(JodinNodeType::Identifier(Identifier::from("hi")));
        node.add_tag(DummyTag);
        node.add_tag(BlockIdentifierTag::new(5));
        node.add_tag(DummyTag);
        let id_tag = node.get_tag::<BlockIdentifierTag>().unwrap();
        assert_eq!(id_tag.block_num(), 5);
        let dummy_tags = node.get_tags::<DummyTag>();
        assert_eq!(dummy_tags.len(), 2);
        assert!(node.get_tag::<ResolvedIdentityTag>().is_err());
    }

    mod node_refs {
        use crate::ast::jodin_node::node_tree;
        use crate::ast::{JodinNode, JodinNodeType};
        use crate::utility::{node_count, Acceptor, AcceptorMut, Tree};

        #[test]
        fn base_node_ref() {
            let node: JodinNode = JodinNodeType::Empty.into();
            let reference = node.get_reference();
            let found = node
                .accept(&reference)
                .expect("Couldn't get node from node reference");
            let found_reference = found.get_reference();
            assert_eq!(found_reference, reference, "The found reference should be equivalent to the original, because they're the same node")
        }

        #[test]
        fn get_nodes_with_1_child() {
            let tree = node_tree(5, 1);
            let bottom = &tree[&[0usize; 4]];
            let bottom_ref = bottom.get_reference();
            println!("ref: {:?}", bottom_ref);
            let gotten = tree.accept(&bottom_ref).unwrap();
            assert_eq!(gotten.get_reference(), bottom.get_reference());
        }

        #[test]
        fn get_nodes_with_many_children() {
            let tree = node_tree(5, 2);
            assert_eq!(node_count(&tree), 31, "Didn't create the correct size tree");
            println!("{:#?}", tree);
            let node = &tree[&[0, 1, 1]];
            let node_ref = node.get_reference();
            println!("ref: {:?}", node_ref);
            let gotten = tree.accept(&node_ref).unwrap();
            assert_eq!(gotten.get_reference(), node.get_reference());
        }
    }
}
