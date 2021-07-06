use crate::core::error::JodinErrorType::IdentifierDoesNotExist;
use crate::core::error::{JodinErrorType, JodinResult};
use crate::core::identifier::{Identifier, IdentifierIterator, Namespaced};
use crate::utility::Tree;
use ptree::{write_tree, Style, TreeBuilder, TreeItem};
use std::borrow::Cow;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::ops::Deref;

struct Node<T: Namespaced> {
    id: Identifier,
    children: Vec<Node<T>>,
    related_values: Vec<T>,
}

impl<T: Namespaced> Node<T> {
    fn new(id: Identifier) -> Self {
        Node {
            id,
            children: vec![],
            related_values: vec![],
        }
    }

    fn add_child(&mut self, node: Self) {
        self.children.push(node)
    }

    fn add_related_value(&mut self, related: T) {
        self.related_values.push(related)
    }

    pub fn id(&self) -> &Identifier {
        &self.id
    }
    pub fn children(&self) -> &Vec<Node<T>> {
        &self.children
    }
    pub fn related_values(&self) -> &Vec<T> {
        &self.related_values
    }
    pub fn children_mut(&mut self) -> &mut Vec<Node<T>> {
        &mut self.children
    }
    pub fn related_values_mut(&mut self) -> &mut Vec<T> {
        &mut self.related_values
    }
}

/// Creates a tree of namespaces that allow for resolution by searching
pub struct NamespaceTree<T: Namespaced> {
    head: Node<T>,
}

impl<T: Namespaced> NamespaceTree<T> {
    /// Creates a new namespace tree that's completely empty
    pub fn new() -> Self {
        Self {
            head: Node::new(Identifier::from("")),
        }
    }

    /// Creates a new namespace tree that's completely empty
    pub fn with_initial_namespace(id: Identifier) -> Self {
        Self {
            head: Node::new(id),
        }
    }

    fn top_namespaces(&self) -> &Vec<Node<T>> {
        self.head.children()
    }

    fn get_node(&self, namespace: &Identifier) -> Option<&Node<T>> {
        if let Some(parent) = namespace.parent() {
            let parent = self.get_node(parent);
            if parent.is_none() {
                return None;
            }
            for child in parent.unwrap().children() {
                if child.id() == namespace {
                    return Some(child);
                }
            }
        } else {
            for namespace_node in self.head.children() {
                if namespace_node.id() == namespace {
                    return Some(namespace_node);
                }
            }
        }
        None
    }

    fn get_node_mut(&mut self, namespace: &Identifier) -> Option<&mut Node<T>> {
        if let Some(parent) = namespace.parent() {
            let parent = self.get_node_mut(parent);
            if parent.is_none() {
                return None;
            }
            for child in parent.unwrap().children_mut() {
                if child.id() == namespace {
                    return Some(child);
                }
            }
        } else {
            for namespace_node in self.head.children_mut() {
                if namespace_node.id() == namespace {
                    return Some(namespace_node);
                }
            }
        }
        None
    }

    /// Checks if an absolute namespace exists
    pub fn namespace_exists(&self, namespace: &Identifier) -> bool {
        self.get_node(namespace).is_some()
    }

    /// Gets possible absolute namespaces given a path and a current absolute namespace. The queried path will
    /// be treated as both relative and absolute
    pub fn get_namespaces(
        &self,
        current_namespace: Option<&Identifier>,
        path: &Identifier,
    ) -> HashSet<&Identifier> {
        let mut output = HashSet::new();
        if let Some(abs) = self.get_namespace_absolute(path) {
            output.insert(abs.id());
        }
        if let Some(current) = current_namespace {
            if let Some(current_node) = self.get_namespace_absolute(current) {
                let mut iter: IdentifierIterator = path.into_iter();
                let mut ptr = current_node;
                let mut found = true;
                while let Some(lookahead) = iter.next() {
                    for child in ptr.children() {
                        if child.id().this() == lookahead {
                            ptr = child;
                            continue;
                        }
                    }
                    found = false;
                    break;
                }
                if found && iter.next().is_none() {
                    output.insert(ptr.id());
                }
            }
        }
        output
    }

    fn get_namespace_absolute(&self, namespace: &Identifier) -> Option<&Node<T>> {
        let mut iter: IdentifierIterator = namespace.into_iter();
        let mut ptr = &self.head;
        while let Some(lookahead) = iter.next() {
            for child in ptr.children() {
                if child.id().this() == lookahead {
                    ptr = child;
                    continue;
                }
            }
            return None;
        }
        if iter.next().is_none() {
            Some(ptr)
        } else {
            None
        }
    }

    pub fn get_relevant_objects(&self, absolute_path: &Identifier) -> Option<&Vec<T>> {
        self.get_node(absolute_path)
            .map(|node| node.related_values())
    }

    pub fn get_relevant_objects_mut(&mut self, absolute_path: &Identifier) -> Option<&mut Vec<T>> {
        self.get_node_mut(absolute_path)
            .map(|node| node.related_values_mut())
    }

    pub fn add_namespace(&mut self, namespace: Identifier) {
        if self.namespace_exists(&namespace) {
            return;
        }
        if let Some(parent) = namespace.parent() {
            if !self.namespace_exists(parent) {
                self.add_namespace(*parent.clone())
            }
            self.get_node_mut(parent)
                .unwrap()
                .add_child(Node::new(namespace))
        } else {
            self.head.add_child(Node::new(namespace))
        }
    }

    pub fn get_base_values(&self) -> &Vec<T> {
        &self.head.related_values
    }

    pub fn get_base_values_mut(&mut self) -> &mut Vec<T> {
        &mut self.head.related_values
    }

    pub fn get_from_identifier(
        &self,
        current_namespace: Option<&Identifier>,
        path: &Identifier,
    ) -> JodinResult<&T> {
        if path.parent().is_none() {
            for o in self.get_base_values() {
                if o.get_identifier() == path {
                    return Ok(o);
                }
            }
        }

        let mut output = None;
        for namespace in self.get_namespaces(current_namespace, path) {
            for object in self.get_relevant_objects(namespace).unwrap() {
                if object.get_identifier() == path {
                    if output.is_none() {
                        output = Some(object);
                    } else {
                        return Err(JodinErrorType::IdentifierDoesNotExist(path.clone()).into());
                    }
                }
            }
        }

        output.ok_or(JodinErrorType::IdentifierDoesNotExist(path.clone()).into())
    }

    pub fn get_from_absolute_identifier(&self, path: &Identifier) -> JodinResult<&T> {
        let mut ptr = &self.head;
        let names: Vec<String> = path.into_iter().collect();
        for name in &names[..names.len() - 1] {
            /*
            if ptr.id() != name {
                return Err(IdentifierDoesNotExist(path.clone()));
            }

             */
            let mut found = false;
            for child in ptr.children() {
                if child.id().this() == name {
                    ptr = child;
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(IdentifierDoesNotExist(path.clone()).into());
            }
        }
        let last_ptr = ptr;
        for value in last_ptr.related_values() {
            let full_id = value.get_identifier();
            if full_id == path {
                return Ok(value);
            }
        }
        Err(JodinErrorType::IdentifierDoesNotExist(path.clone()).into())
    }
}

impl<T: Namespaced> Tree for Node<T> {
    fn direct_children(&self) -> Vec<&Self> {
        self.children.iter().collect()
    }
}

#[derive(Clone)]
struct NodeInfo {
    id: Identifier,
    children: Vec<NodeInfo>,
    relevant: Vec<Identifier>,
    is_namespace: bool,
}

impl<T: Namespaced> From<&Node<T>> for NodeInfo {
    fn from(n: &Node<T>) -> Self {
        NodeInfo {
            id: n.id.clone(),
            children: n.children.iter().map(|node| NodeInfo::from(node)).collect(),
            relevant: n
                .related_values
                .iter()
                .map(|r| r.get_identifier().clone())
                .collect(),
            is_namespace: true,
        }
    }
}

impl TreeItem for NodeInfo {
    type Child = Self;

    fn write_self<W: std::io::Write>(&self, f: &mut W, style: &Style) -> std::io::Result<()> {
        if self.is_namespace {
            write!(f, "{}", style.paint(&self.id.this()),)
        } else {
            write!(f, "<{}>", style.paint(&self.id.this()),)
        }
    }

    fn children(&self) -> Cow<[Self::Child]> {
        let mut objects = vec![];
        objects.extend(self.relevant.iter().map(|id| NodeInfo {
            id: id.clone(),
            children: vec![],
            relevant: vec![],
            is_namespace: false,
        }));
        objects.extend(self.children.iter().cloned());
        Cow::from(objects)
    }
}

impl<T: Namespaced> Debug for NamespaceTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tree = NodeInfo::from(&self.head);
        let mut vec = vec![];
        write_tree(&tree, &mut vec).map_err(|e| std::fmt::Error::default())?;
        let string = String::from_utf8(vec).map_err(|e| std::fmt::Error::default())?;
        write!(f, "{}", string)
    }
}
