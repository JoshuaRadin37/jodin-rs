//! Contains plugins that are useful for debugging

use crate::plugins::{load_plugin, Stack};
use crate::{Plugin, VMHandle};
use jodin_common::assembly::value::Value;
use log::{info, warn, Level};

/// The logging plugin overrides the default print and write function
#[derive(Default)]
pub struct LoggingPlugin;

impl Plugin for LoggingPlugin {
    fn labels(&self, buffer: &mut [&'static str]) {
        buffer[0] = "print";
        buffer[1] = "write";
    }

    fn labels_count(&self) -> i32 {
        2
    }

    fn call_label(
        &self,
        label: &str,
        stack: &mut dyn Stack,
        handle: &mut dyn VMHandle,
        output: &mut Option<Result<Value, String>>,
    ) {
        match label {
            "print" => {
                let mut top: Option<Value> = None;
                stack.pop(&mut top);
                warn!("{}", top.unwrap());
                *output = Some(Ok(Value::Empty));
            }
            _ => unreachable!(),
        }
    }
}
