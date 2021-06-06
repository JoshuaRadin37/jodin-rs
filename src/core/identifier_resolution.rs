use crate::core::error::{JodinErrorType, JodinResult};
use crate::core::identifier::{Identifier, Namespaced};
use crate::core::namespace_tree::NamespaceTree;
use std::collections::HashSet;
use std::iter::FromIterator;

pub const BASE_NAMESPACE: &str = "{base}";

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
        let full_path = Identifier::with_parent(self.current_namespace_with_base(), namespace);
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
        let full_path = Identifier::with_parent(self.current_namespace_with_base(), id);
        println!("Created path {}", full_path);
        let parent_path = &**full_path.parent().as_ref().unwrap();
        self.tree.add_namespace(parent_path.clone());
        let objects = self.tree.get_relevant_objects_mut(parent_path).unwrap();
        if !objects.contains(&full_path) {
            objects.push(full_path.clone())
        }
        full_path.strip_highest_parent().unwrap()
    }

    pub fn add_namespace<N: Into<Identifier>>(&mut self, namespace: N) {
        self.tree.add_namespace(Identifier::with_parent(
            self.current_namespace_with_base(),
            namespace,
        ));
    }

    pub fn resolve_path(&self, path: Identifier) -> JodinResult<Identifier> {
        let mut output = HashSet::new();

        let absolute_path = Identifier::with_parent(&self.base_namespace, &path);
        //println!("Absolute path = {}", absolute_path);
        let relative_path = Identifier::with_parent(self.current_namespace_with_base(), &path);
        //println!("Relative path = {}", relative_path);
        if let Ok(val) = self.tree.get_from_absolute_identifier(&absolute_path) {
            output.insert(val);
        }

        if let Ok(val) = self.tree.get_from_absolute_identifier(&relative_path) {
            output.insert(val);
        }

        for using in &self.using_namespaces {
            let using_path = Identifier::with_parent(
                Identifier::with_parent(&self.base_namespace, using),
                &path,
            );
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

    pub fn current_namespace(&self) -> &Option<Identifier> {
        &self.current_namespace
    }

    pub fn current_namespace_with_base(&self) -> Identifier {
        let base = &self.base_namespace;
        if let Some(current) = &self.current_namespace {
            Identifier::with_parent(base, current)
        } else {
            base.clone()
        }
    }

    pub fn contains_absolute_identifier(&self, path: &Identifier) -> bool {
        self.tree.get_from_absolute_identifier(path).is_ok()
    }
}

#[cfg(test)]
mod test {
    use crate::core::error::JodinErrorType;
    use crate::core::identifier::Identifier;
    use crate::core::identifier_resolution::IdentifierResolver;
    use std::iter::FromIterator;

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
        if let Err(JodinErrorType::AmbiguousIdentifierError { given: _, found }) = result {
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
