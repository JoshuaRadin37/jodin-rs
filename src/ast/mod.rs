//! This module contains all of the relevant parts for how ASTs are created.
//!
//! The abstract syntax tree should be made up of as few different types as possible, and when
//! instead of adding more fields to a variant of the [node type] enum, instead tags should be added.
//!
//! Tags are a way of adding information to the AST without needing to have many different fields for
//! every single instance of a [JodinNode]
//!
//! [node type]: self::node_type::JodinNodeType
//! [JodinNode]: self::jodin_node::JodinNode

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::path::PathBuf;

use crate::ast::intermediate_type::{IntermediateType, TypeSpecifier, TypeTail};
pub use crate::ast::jodin_node::JodinNode;
pub use crate::ast::node_type::JodinNodeType;
use crate::compilation_settings::CompilationSettings;
use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::import::Import;
use crate::core::literal::Literal;
use crate::core::operator::{Operator, TryConstEvaluation};
use crate::core::privacy::{Visibility, VisibilityTag};
use crate::core::types::primitives::Primitive;
use crate::utility::{Bytes, HumanReadable, IntoBox};

pub mod intermediate_type;
mod jodin_node;
mod node_type;
pub mod tags;

/// parse ids
pub fn parse_identifier() -> JodinResult<Identifier> {
    todo!()
}
