//! Plugins for the VM!

pub mod error;
pub use error::PluginError as Error;
pub use jodin_common::assembly::value::Value;

pub mod plugins;
use crate::plugins::VMHandle;
pub use plugins::Plugin;

pub use libloading::{Library, Symbol};
