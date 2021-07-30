//! The main method for tracking, then resolving identifiers.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

use ptree::{write_tree, Style, TreeItem};

use crate::core::error::JodinErrorType::IdentifierDoesNotExist;
use crate::core::error::{JodinErrorType, JodinResult};
use crate::core::identifier::{Identifier, IdentifierIterator, Namespaced};
use crate::utility::Tree;

/// The default base namespace that all identifiers added to the project will be part of.
pub const BASE_NAMESPACE: &str = "{base}";

/// Maintains a [NamespaceTree](self::NamespaceTree), the current namespace,
/// the base namespace, and all namespaces that are being "used".
#[derive(Debug)]
pub struct IdentifierResolver {
    tree: NamespaceTree<Identifier>,
    current_namespace: Option<Identifier>,
    using_namespaces: Vec<Identifier>,
    base_namespace: Identifier,
}

impl IdentifierResolver {
    /// Creates a new, empty identifier resolver
    pub fn new() -> Self {
        Self::with_base_namespace(BASE_NAMESPACE)
    }

    /// Creates a new, empty identifier resolver with a custom base namespace. This is used instead of
    /// the `BASE_NAMESPACE`
    pub fn with_base_namespace<S: AsRef<str>>(base_namespace: S) -> Self {
        let mut tree = NamespaceTree::new();
        tree.add_namespace(Identifier::from(&base_namespace));
        Self {
            tree,
            current_namespace: None,
            using_namespaces: vec![],
            base_namespace: base_namespace.as_ref().to_string().into(),
        }
    }

    /// Pushes a namespace onto the current namespace
    pub fn push_namespace(&mut self, namespace: Identifier) {
        let full_path = Identifier::new_concat(self.current_namespace_with_base(), namespace);
        self.tree.add_namespace(full_path.clone());
        self.current_namespace = Some(full_path.strip_highest_parent().unwrap());
    }

    /// Pops the outermost namespace from the current namespace
    pub fn pop_namespace(&mut self) {
        let old = std::mem::replace(&mut self.current_namespace, None);
        if let Some(old) = old {
            self.current_namespace = old.into_parent();
        }
    }

    /// Adds a namespace to search from relatively
    pub fn use_namespace(&mut self, namespace: Identifier) -> JodinResult<()> {
        let resolved_set = self
            .tree
            .get_namespaces(self.current_namespace.as_ref(), &namespace);
        if resolved_set.is_empty() {
            return Err(JodinErrorType::IdentifierDoesNotExist(namespace))?;
        } else if resolved_set.len() >= 2 {
            return Err(JodinErrorType::AmbiguousIdentifierError {
                given: namespace,
                found: Vec::from_iter(resolved_set.into_iter().map(|id| id.clone())),
            })?;
        }
        let resolved = resolved_set.into_iter().next().cloned().unwrap();
        self.using_namespaces.push(resolved);
        Ok(())
    }

    /// Removes a namespace to search from, if it exists
    pub fn stop_use_namespace(&mut self, namespace: &Identifier) -> JodinResult<()> {
        let namespace = namespace.clone();
        let resolved_set = self
            .tree
            .get_namespaces(self.current_namespace.as_ref(), &namespace);
        if resolved_set.is_empty() {
            return Err(JodinErrorType::IdentifierDoesNotExist(namespace).into());
        } else if resolved_set.len() >= 2 {
            return Err(JodinErrorType::AmbiguousIdentifierError {
                given: namespace,
                found: Vec::from_iter(resolved_set.into_iter().map(|id| id.clone())),
            }
            .into());
        }
        let resolved = resolved_set.into_iter().next().unwrap();
        self.using_namespaces.retain(|id| id != resolved);
        Ok(())
    }

    /// Creates an absolute path based off the current namesapce
    pub fn create_absolute_path(&mut self, id: &Identifier) -> Identifier {
        /*
        if self.current_namespace.is_none() {
            if !self.tree.get_base_values().contains(&id) {
                self.tree.get_base_values_mut().push(id.clone());
            }
            return id;
        }

         */
        let full_path = Identifier::new_concat(self.current_namespace_with_base(), id);
        println!("Created path {}", full_path);
        let parent_path = &**full_path.parent().as_ref().unwrap();
        self.tree.add_namespace(parent_path.clone());
        let objects = self.tree.get_relevant_objects_mut(parent_path).unwrap();
        if !objects.contains(&full_path) {
            objects.push(full_path.clone())
        }
        full_path.strip_highest_parent().unwrap()
    }

    /// Add a new namespace relative to the current namespace to the resolver
    pub fn add_namespace<N: Into<Identifier>>(&mut self, namespace: N) {
        self.tree.add_namespace(Identifier::new_concat(
            self.current_namespace_with_base(),
            namespace,
        ));
    }

    /// Attempts to resolve a single, absolute identifier out of a given path.
    pub fn resolve_path(&self, path: Identifier) -> JodinResult<Identifier> {
        let mut output = HashSet::new();

        let absolute_path = Identifier::new_concat(&self.base_namespace, &path);
        //println!("Absolute path = {}", absolute_path);
        let relative_path = Identifier::new_concat(self.current_namespace_with_base(), &path);
        //println!("Relative path = {}", relative_path);
        if let Ok(val) = self.tree.get_from_absolute_identifier(&absolute_path) {
            output.insert(val);
        }

        if let Ok(val) = self.tree.get_from_absolute_identifier(&relative_path) {
            output.insert(val);
        }

        for using in &self.using_namespaces {
            let using_path =
                Identifier::new_concat(Identifier::new_concat(&self.base_namespace, using), &path);
            //println!("Using path = {}", using_path);
            if let Ok(id) = self.tree.get_from_absolute_identifier(&using_path) {
                output.insert(id);
            }
        }

        match output.len() {
            0 => Err(JodinErrorType::IdentifierDoesNotExist(path))?,
            1 => Ok(output
                .into_iter()
                .next()
                .cloned()
                .unwrap()
                .strip_highest_parent()
                .unwrap()),
            _ => Err(JodinErrorType::AmbiguousIdentifierError {
                given: path,
                found: Vec::from_iter(
                    output
                        .into_iter()
                        .cloned()
                        .map(|id| id.strip_highest_parent().unwrap()),
                ),
            })?,
        }
    }

    /// the current namespace.
    pub fn current_namespace(&self) -> &Option<Identifier> {
        &self.current_namespace
    }

    /// The current namespace with the base namespace prepended to it.
    pub fn current_namespace_with_base(&self) -> Identifier {
        let base = &self.base_namespace;
        if let Some(current) = &self.current_namespace {
            Identifier::new_concat(base, current)
        } else {
            base.clone()
        }
    }

    /// Checks if the resolver contains the absolute identifier.
    pub fn contains_absolute_identifier(&self, path: &Identifier) -> bool {
        self.tree.get_from_absolute_identifier(path).is_ok()
    }
}

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
    pub fn new_with_initial_namespace(id: Identifier) -> Self {
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

    /// Get the associated, relevant objects for an absolute path
    pub fn get_relevant_objects(&self, absolute_path: &Identifier) -> Option<&Vec<T>> {
        self.get_node(absolute_path)
            .map(|node| node.related_values())
    }

    /// Gets mutable references to the associated, relevant objects for an absolute path
    pub fn get_relevant_objects_mut(&mut self, absolute_path: &Identifier) -> Option<&mut Vec<T>> {
        self.get_node_mut(absolute_path)
            .map(|node| node.related_values_mut())
    }

    /// Adds a new namespace to the namespace tree.
    pub fn add_namespace(&mut self, namespace: Identifier) {
        if self.namespace_exists(&namespace) {
            return;
        }
        if let Some(parent) = namespace.parent() {
            if !self.namespace_exists(parent) {
                self.add_namespace(parent.clone())
            }
            self.get_node_mut(parent)
                .unwrap()
                .add_child(Node::new(namespace))
        } else {
            self.head.add_child(Node::new(namespace))
        }
    }

    /// Gets the base associated objects
    pub fn get_base_values(&self) -> &Vec<T> {
        &self.head.related_values
    }

    /// Gets a mutable reference to the base associated objects.
    pub fn get_base_values_mut(&mut self) -> &mut Vec<T> {
        &mut self.head.related_values
    }

    /// Attempts to get the associated value by taking in a current path and a relative path. The path
    /// can either be relative or absolute. If more than one value is found, an error is returned.
    ///
    /// # Arguments
    ///
    /// * `current_namespace`: An optional, current namespace.
    /// * `path`: The path to search for.
    ///
    /// returns: `Result<&T, JodinError>` Either a reference to the associated value, or an Error
    #[deprecated]
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

    /// Attempts to get the associated value from an absolute path.
    ///
    /// # Arguments
    ///
    /// * `path`: The absolute path
    ///
    /// returns: Result<&T, JodinError> the associated value, or an error
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
        write_tree(&tree, &mut vec).map_err(|_e| std::fmt::Error::default())?;
        let string = String::from_utf8(vec).map_err(|_e| std::fmt::Error::default())?;
        write!(f, "{}", string)
    }
}

/// Contains an identifier resolver and a mapping between full identifiers and it's associated value.
pub struct Registry<T> {
    resolver: IdentifierResolver,
    mapping: HashMap<Identifier, T>,
}

impl<T> Registry<T> {
    /// Creates a new, empty registry
    pub fn new() -> Self {
        Self {
            resolver: IdentifierResolver::new(),
            mapping: Default::default(),
        }
    }

    /// Creates a new registry that contains identifiers within the tree already.
    pub fn new_with_resolver(resolver: IdentifierResolver) -> Self {
        Self {
            resolver,
            mapping: Default::default(),
        }
    }

    /// Insert a new value into the registry. This identifier should be relative.
    pub fn insert(&mut self, val: T) -> JodinResult<Identifier>
    where
        T: Namespaced,
    {
        let identifier = val.get_identifier().clone();
        self.insert_with_identifier(val, identifier)
    }

    /// Inserts a value into the registry associated to an identifier.
    pub fn insert_with_identifier(&mut self, val: T, path: Identifier) -> JodinResult<Identifier> {
        let path = self.resolver.create_absolute_path(&path);
        if self.mapping.contains_key(&path) {
            return Err(JodinErrorType::IdentifierAlreadyExists(path).into());
        }
        self.mapping.insert(path.clone(), val);
        Ok(path)
    }

    /// Updates the value of an identifier, but only if it already exists within the registry.
    pub fn update_absolute_identity(&mut self, absolute: &Identifier, val: T) -> JodinResult<&T> {
        if !self.resolver.contains_absolute_identifier(absolute) {
            return Err(JodinErrorType::IdentifierDoesNotExist(absolute.clone()).into());
        }
        self.mapping.insert(absolute.clone(), val);
        Ok(&self.mapping[absolute])
    }

    /// Pushes a namespace onto the current namespace
    pub fn push_namespace(&mut self, namespace: Identifier) {
        self.resolver.push_namespace(namespace);
    }

    /// Pops the outermost namespace from the current namespace
    pub fn pop_namespace(&mut self) {
        self.resolver.pop_namespace()
    }

    /// Adds a namespace to search from relatively
    pub fn use_namespace(&mut self, namespace: Identifier) -> JodinResult<()> {
        self.resolver.use_namespace(namespace)
    }

    /// Removes a namespace to search from, if it exists
    pub fn stop_use_namespace(&mut self, namespace: &Identifier) -> JodinResult<()> {
        self.resolver.stop_use_namespace(namespace)
    }

    /// Attempts to get a value from a given path.
    pub fn get(&self, path: &Identifier) -> JodinResult<&T> {
        let full_path = self.resolver.resolve_path(path.clone())?;
        self.mapping
            .get(&full_path)
            .ok_or(JodinErrorType::IdentifierDoesNotExist(path.clone()).into())
    }

    /// Attempts to get a mutable value from a given path.
    pub fn get_mut(&mut self, path: &Identifier) -> JodinResult<&mut T> {
        let full_path = self.resolver.resolve_path(path.clone())?;
        self.mapping
            .get_mut(&full_path)
            .ok_or(JodinErrorType::IdentifierDoesNotExist(path.clone()).into())
    }
}

impl<I: Into<Identifier>, T> Index<I> for Registry<T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.get(&index.into()).unwrap()
    }
}

impl<I: Into<Identifier>, T> IndexMut<I> for Registry<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(&index.into()).unwrap()
    }
}

/// Enables registration of an object to a proper registry. Implementations must include all children into
/// registration
pub trait Registrable<T = Self>: Sized {
    /// Registers both this item and all related children to this registry
    fn register(self, register: &mut Registry<T>) -> JodinResult<Identifier>;
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::core::error::JodinErrorType;
    use crate::core::identifier::Identifiable;
    use crate::core::identifier::Identifier;
    use crate::core::identifier_resolution::IdentifierResolver;
    use crate::core::identifier_resolution::Registry;

    #[test]
    fn insert_entries() {
        let mut register = Registry::new();
        register.push_namespace(Identifier::from("std"));
        register.insert_with_identifier(3, Identifier::from("best value"));
        let value = &register[Identifier::from_iter(&["std", "best value"])];
        assert_eq!(*value, 3);

        let mut registry = Registry::new();
        registry.insert(Identifiable::new("val1", 1)).unwrap();
        registry.insert(Identifiable::new("val2", 2)).unwrap();
        registry.insert(Identifiable::new("val3", 3)).unwrap();
    }

    #[test]
    fn id_resolution() {
        let mut resolver = IdentifierResolver::new();
        resolver.add_namespace(Identifier::from_iter("n1::n2::n3".split("::")));
        resolver.add_namespace(Identifier::from_iter("n1::n2::n4".split("::")));
        let path =
            resolver.create_absolute_path(&Identifier::from_iter("n1::n2::object".split("::")));
        println!("{:#?}", resolver);
        assert_eq!(path, "n1::n2::object");
        resolver.push_namespace(Identifier::from("n2"));
        println!("{:#?}", resolver);
        let path = resolver.create_absolute_path(&Identifier::from("object"));
        assert_eq!(path, "n2::object");
        println!("{:#?}", resolver);
        let result = resolver
            .resolve_path(Identifier::from_iter(&["n2", "object"]))
            .unwrap();
        assert_eq!(result, "n2::object");
        resolver.pop_namespace();
        resolver.push_namespace(Identifier::from("n1"));
        println!("{:#?}", resolver);
        let result = resolver.resolve_path(Identifier::from_iter(&["n2", "object"]));
        if let Err(JodinErrorType::AmbiguousIdentifierError { given: _, found }) =
            result.map_err(|err| err.into_err_and_bt().0)
        {
            assert!(
                (found
                    == vec![
                        Identifier::from_iter(&["n2", "object"]),
                        Identifier::from_iter(&["n1", "n2", "object"])
                    ])
                    || (found
                        == vec![
                            Identifier::from_iter(&["n1", "n2", "object"]),
                            Identifier::from_iter(&["n2", "object"]),
                        ])
            )
        } else {
            panic!("This should be ambiguous from this position, as both n1::n2::object (relative) and n2::object (absolute) exists");
        }
    }
}
