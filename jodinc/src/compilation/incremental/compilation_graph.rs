//! The compilation graph represents the order that files given in the input should be compiled

use jodin_common::ast::JodinNode;
use jodin_common::unit::TranslationUnit;
use petgraph::Graph;
use std::path::{Path, PathBuf};

use crate::Result;
use std::result::Result as StdResult;

/// A view of a part of the compilation process. Should hold access to a single file path.
pub struct CompilationNode {
    path: PathBuf,
    parsed_node: JodinNode,
    imported_declarations: Vec<TranslationUnit>,
}

impl CompilationNode {
    fn new(path: impl AsRef<Path>, node: JodinNode, imported_decs: Vec<TranslationUnit>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            parsed_node: node,
            imported_declarations: imported_decs,
        }
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

    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    pub fn build_node(&mut self, file_path: impl AsRef<Path>) -> Result<CompilationNode> {
        todo!()
    }
}

/// The compilation graph structure
pub struct CompilationGraph {
    /// The backing graph
    pub graph: Graph<CompilationNode, ()>,
}
