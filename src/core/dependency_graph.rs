//! Allows the tracking of dependencies and produce an in order representation

use crate::core::error::{JodinErrorType, JodinResult, JodinError};
use crate::core::identifier::{Identifier, Namespaced};
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use crate::ast::tags::Tag;
use std::any::Any;
use std::convert::TryFrom;
use crate::ast::JodinNode;
use crate::passes::analysis::ResolvedIdentityTag;

/// Create a graph of dependencies between identifiers. An edge
/// represents that this id has a dependency
pub struct DependencyGraph {
    nodes: HashMap<Identifier, u64>,
    key_to_id: HashMap<u64, Identifier>,
    edges: HashMap<u64, HashSet<u64>>,
    id: u64,
}

impl DependencyGraph {
    /// Create a new dependency graph
    pub fn new() -> Self {
        DependencyGraph {
            nodes: HashMap::new(),
            key_to_id: HashMap::new(),
            edges: HashMap::new(),
            id: 0,
        }
    }

    fn new_primary_key(&mut self) -> u64 {
        let ret = self.id;
        self.id += 1;
        ret
    }

    fn add_id_and_deps(&mut self, id: Identifier, dependencies: Vec<&Identifier>) -> u64 {
        let mut deps = HashSet::new();
        for dependency in dependencies {
            let dep_key = if self.nodes.contains_key(dependency) {
                *self.nodes.get(dependency).unwrap()
            } else {
                self.add_id_and_deps(dependency.clone(), vec![])
            };
            deps.insert(dep_key);
        }

        let key = if !self.nodes.contains_key(&id) {
            let key = self.new_primary_key();
            self.nodes.insert(id.clone(), key);
            self.key_to_id.insert(key, id.clone());
            key
        } else {
            *self.nodes.get(&id).unwrap()
        };
        self.edges.entry(key).or_insert(HashSet::new()).extend(deps);
        key
    }

    /// Add an object that has dependencies to the dependency graph
    pub fn add<T: HasDependencies>(&mut self, instance: &T) {
        self.add_id_and_deps(instance.get_identifier().clone(), instance.dependencies());
    }

    /// Add a dependency for an ID
    pub fn add_dependency<N: Namespaced, N2: Namespaced>(&mut self, id: N, dependency: N2) {
        let id = id.get_identifier().clone();
        let dependency = dependency.get_identifier();

        self.add_id_and_deps(id, vec![dependency]);
    }

    /// Add dependencies for an ID
    pub fn add_dependencies<N, N2, I>(&mut self, id: N, dependency: I)
    where
        N: Namespaced,
        N2: Namespaced,
        for<'a> &'a I: IntoIterator<Item = &'a N2>,
    {
        let id = id.get_identifier().clone();
        let dependencies: Vec<&Identifier> = dependency
            .into_iter()
            .map(|n| n.get_identifier())
            .collect::<Vec<_>>();

        self.add_id_and_deps(id, dependencies);
    }

    /// Gets the identifiers in a list in order such no identifier given
    /// is dependent on a following identifier.
    ///
    /// # Error
    ///
    /// Will return an error if there is a circular dependency
    pub fn dependence_order(&self) -> JodinResult<Vec<&Identifier>> {
        let mut dep_order = vec![];

        let mut identifier_keys: Vec<(u64, HashSet<u64>)> = self
            .key_to_id
            .keys()
            .map(|key| (*key, self.edges[key].clone()))
            .collect();
        identifier_keys.sort_by_cached_key(|(_, set)| Reverse(set.len()));

        while let Some((key, set)) = identifier_keys.pop() {
            if set.len() > 0 {
                return Err(JodinErrorType::CircularDependencyDetected.into());
            }

            let id = &self.key_to_id[&key];
            dep_order.push(id);

            for (_, set) in &mut identifier_keys {
                set.remove(&key);
            }

            identifier_keys.sort_by_cached_key(|(_, set)| Reverse(set.len()));
        }

        Ok(dep_order)
    }
}

/// Represent a type that has dependencies
pub trait HasDependencies: Namespaced {
    /// The identifiers to which this identifier is dependent on.
    fn dependencies(&self) -> Vec<&Identifier>;
}

/// A tag that contains the identifiers that this node is dependent on
pub struct DependencyTag {
    /// What this node is dependent on
    pub dependencies: Vec<Identifier>
}

impl Tag for DependencyTag {
    fn tag_type(&self) -> String {
        "Dependencies".to_string()
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

/// Stores dependency information about a node
pub struct DependencyInfo<'a> {
    /// This identifier for this node
    pub this: &'a Identifier,
    /// The identifiers that this is dependent on
    pub dependencies: Vec<&'a Identifier>
}

impl<'a> TryFrom<&'a JodinNode> for DependencyInfo<'a> {
    type Error = JodinError;

    fn try_from(value: &'a JodinNode) -> Result<Self, Self::Error> {
        let this = value.get_tag::<ResolvedIdentityTag>()?.absolute_id();
        let dependencies: Vec<_> = value.get_tag::<DependencyTag>().map_or(vec![], |tag| tag.dependencies.iter().collect());

        Ok(Self {
            this,
            dependencies
        })
    }
}

impl Namespaced for DependencyInfo<'_> {
    fn get_identifier(&self) -> &Identifier {
        self.this
    }
}

impl HasDependencies for DependencyInfo<'_> {
    fn dependencies(&self) -> Vec<&Identifier> {
        self.dependencies.clone()
    }
}


#[cfg(test)]
mod tests {
    use crate::core::dependency_graph::DependencyGraph;
    use crate::core::identifier::Identifier;

    #[test]
    fn basic_dependency_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(Identifier::from("a2"), Identifier::from("a3"));
        graph.add_dependencies(Identifier::from("a1"), vec![Identifier::from("a2")]);

        let order: Vec<_> = graph
            .dependence_order()
            .unwrap()
            .into_iter()
            .map(|m| m.to_string())
            .collect();

        assert_eq!(order, vec!["a3", "a2", "a1"]);
    }
}
