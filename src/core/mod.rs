//! Contains all of the core components of the compiler. This is where any part that is repeatedly
//! used in the project is stored.

pub mod dependency_graph;
pub mod error;
pub mod external;
pub mod generic_context;
pub mod identifier;
pub mod identifier_resolution;
pub mod import;
pub mod literal;
pub mod operator;
pub mod privacy;
pub mod types;
