//! Plugins for the VM!

pub mod error;
pub use error::PluginError as Error;
use jodin_common::assembly::value::Value;

pub mod plugins;
use crate::plugins::VMHandle;
pub use plugins::Plugin;
