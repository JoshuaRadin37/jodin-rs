//! The compilation graph represents the order that files given in the input should be compiled

use jodin_common::ast::{JodinNode, JodinNodeType};
use jodin_common::unit::TranslationUnit;
use petgraph::Graph;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::ops::{Deref, Index};
use std::path::{Path, PathBuf};

use crate::compilation::incremental::variant::{
    ExposingVariant, IncomingExposedUnit, IncomingVariant, OutgoingVariant, Variant,
};
use crate::passes::analysis::with_own_identities;
use crate::{CompilerError, Result};
use jodin_common::core::identifier_resolution::{IdentifierResolver, Registry};
use jodin_common::core::import::Import;
use jodin_common::core::privacy::Visibility;
use jodin_common::identifier::Identifier;
use jodin_common::parsing::parse_program;
use jodin_common::types::base_type::base_type;
use jodin_common::utility::PathUtils;
use more_collection_macros::map;
use petgraph::algo::{greedy_feedback_arc_set, is_cyclic_directed, toposort};
use petgraph::graph::{DefaultIx, DiGraph, NodeIndex};
use petgraph::graphmap::{DiGraphMap, GraphMap};
use petgraph::visit::NodeIndexable;
use std::result::Result as StdResult;

/// A view of a part of the compilation process. Should hold access to a single file path.
pub struct CompilationNode {
    pub path: PathBuf,
    pub parsed_node: JodinNode,

    identifier_resolver: Cell<Option<IdentifierResolver>>,
    visibility: Cell<Option<Registry<Visibility>>>,

    incoming: IncomingVariant,
    outgoing: OutgoingVariant,
}

impl CompilationNode {}

impl ExposingVariant for CompilationNode {
    fn exposed_units(&self) -> &OutgoingVariant {
        &self.outgoing
    }
}

impl Variant for CompilationNode {
    fn incoming_units(&self) -> &IncomingVariant {
        &self.incoming
    }

    fn match_to_outgoing(&mut self, out: &OutgoingVariant) {
        self.incoming.match_outgoing(out);
    }
}

/// Builds compilation nodes
pub struct CompilationNodeFactory {
    base_path: PathBuf,
}

impl CompilationNodeFactory {
    /// Create a new compilation node factory with a given base path
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    pub fn base_path(&self) -> &Path {
        &*self.base_path
    }

    pub fn build_node(&mut self, file_path: impl AsRef<Path>) -> Result<CompilationNode> {
        let path = file_path.as_ref();
        let mut node = self.parse_file(path)?;

        let (node, resolver, visibility) = with_own_identities(node)?;
        info!("{:?} created identifiers: {:#?}", path, visibility);
        let imported = imported_modules(&node);
        info!("{:?} imports = {:#?}", path.file_name().unwrap(), imported);

        let outgoing_variant: OutgoingVariant = OutgoingVariant::new(path, visibility.ids());
        info!("{:?} outgoing: {:#?}", path, outgoing_variant);
        let incoming_variant: IncomingVariant = imported.into_iter().collect();

        Ok(CompilationNode {
            path: path.to_path_buf(),
            parsed_node: node,
            identifier_resolver: Cell::new(Some(resolver)),
            visibility: Cell::new(Some(visibility)),
            incoming: incoming_variant,
            outgoing: outgoing_variant,
        })
    }

    fn parse_file(&self, path: &Path) -> Result<JodinNode> {
        let read_file = std::fs::read_to_string(path)?;
        Ok(parse_program(read_file)?)
    }
}

/// Gets the imports of a jodin node
pub fn imported_modules(node: &JodinNode) -> Vec<&Import> {
    let mut output = vec![];
    match node.inner() {
        JodinNodeType::ImportIdentifiers { import_data } => output.push(import_data),
        other => {
            for child in other.children() {
                output.extend(imported_modules(child));
            }
        }
    }
    output
}

/// The compilation graph builder.
pub struct CompilationGraphBuilder {
    node_graph: DiGraph<CompilationNode, ()>,
    factory: CompilationNodeFactory,
    base_path: PathBuf,
    path_to_node: HashMap<PathBuf, NodeIndex>,
    package_to_file: HashMap<Identifier, PathBuf>,
}

impl CompilationGraphBuilder {
    /// Create a new compilation graph
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            node_graph: Graph::new(),
            factory: CompilationNodeFactory::new(base_path.as_ref()),
            base_path: base_path.as_ref().to_path_buf(),
            path_to_node: HashMap::new(),
            package_to_file: HashMap::new(),
        }
    }
    /// attempt to find a file using an identifier
    fn find_file(&self, id: &Identifier) -> Result<PathBuf> {
        let mut direct_path = self.base_path.clone();
        direct_path.push(PathBuf::from(id));
        if direct_path.exists() {
            Ok(direct_path)
        } else {
            Err(CompilerError::InvalidObjectPath(direct_path))
        }
    }

    /// Add a file to the compilation graph.
    pub fn add_file(&mut self, file: impl AsRef<Path>) -> Result<()> {
        let path = file.as_ref().to_path_buf();
        let compilation_node = self.factory.build_node(&path)?;

        let index = self.node_graph.add_node(compilation_node);
        self.path_to_node.insert(path, index);
        Ok(())
    }

    /// Adds multiple files to the compilation graph
    pub fn add_files<P: AsRef<Path>, I: IntoIterator<Item = P>>(&mut self, files: I) -> Result<()> {
        for file in files {
            self.add_file(file)?;
        }
        Ok(())
    }

    fn get_dependent_files(&self, node: &CompilationNode) -> Vec<PathBuf> {
        todo!()
    }

    /// Attempts to build the fully resolved compilation graph. Will fail if there's a cyclical file
    /// dependency. Will not fail if there's a missing identifier, as it will be assumed this is provided later.
    pub fn build(mut self) -> Result<CompilationGraph> {
        let mut constructed_dependencies = map!();
        for &idx in self.path_to_node.values() {
            let node = &self.node_graph[idx];
            let dependent_node_indexes = self
                .get_dependent_files(node)
                .into_iter()
                .filter_map(|path| self.path_to_node.get(&path).copied())
                .collect::<Vec<_>>();
            constructed_dependencies.insert(idx, dependent_node_indexes);
        }

        let CompilationGraphBuilder {
            node_graph: mut graph,
            factory: _,
            base_path: _,
            path_to_node,
            package_to_file: _,
        } = self;

        for (idx, to_idxs) in constructed_dependencies {
            for to_idx in to_idxs {
                graph.add_edge(idx, to_idx, ());
            }
        }

        if let Err(cycle) = toposort(&graph, None) {
            let id: NodeIndex = cycle.node_id();
            let path = graph[id].path.clone();
            return Err(CompilerError::CyclicalDependencyError(path));
        }

        graph.reverse();

        Ok(CompilationGraph {
            graph,
            map: path_to_node,
        })
    }
}

/// The completed compilation graph
pub struct CompilationGraph {
    graph: DiGraph<CompilationNode, ()>,
    map: HashMap<PathBuf, NodeIndex>,
}

impl CompilationGraph {
    /// Gets a list of all dependent files of this file
    pub fn dependents(&self, path: impl AsRef<Path>) -> Vec<&Path> {
        todo!()
    }

    /// Get a list of files this file is dependent on
    pub fn dependencies(&self, path: impl AsRef<Path>) -> Vec<&Path> {
        todo!()
    }

    pub fn topological_order(&self) -> Vec<&CompilationNode> {
        toposort(&self.graph, None)
            .expect("can only be built by a non-cyclical graph")
            .into_iter()
            .filter_map(|idx: NodeIndex| self.graph.node_weight(idx))
            .collect()
    }

    pub fn into_topological_order(mut self) -> Vec<CompilationNode> {
        toposort(&self.graph, None)
            .expect("can only be built by a non-cyclical graph")
            .into_iter()
            .filter_map(|idx: NodeIndex| self.graph.remove_node(idx))
            .collect()
    }
}
