use jodin_common::assembly::value::Value;
use jodin_vm_plugins::declare_plugin;
use jodin_vm_plugins::plugins::{call, LoadablePlugin, Stack, VMHandle};
use jodin_vm_plugins::Plugin;
use std::ffi::CStr;

pub struct KernelPlugin;
impl KernelPlugin {
    pub fn start(&self, handle: &mut dyn VMHandle) -> Result<Value, String> {
        println!("Hello, World!");
        handle.native("@print_stack", &[], &mut None);
        call(handle, "print", &[Value::from("hello")]);
        Ok(Value::UInteger(0))
    }
}

impl LoadablePlugin for KernelPlugin {
    fn new() -> Self {
        Self
    }
}

impl Plugin for KernelPlugin {
    fn labels(&self, buffer: &mut [&'static str]) {
        buffer[0] = "__start";
    }

    fn labels_count(&self) -> i32 {
        1
    }

    fn call_label(
        &self,
        label: &str,
        _stack: &mut dyn Stack,
        handle: &mut dyn VMHandle,
        output: &mut Option<Result<Value, String>>,
    ) {
        match label {
            "__start" => *output = Some(self.start(handle)),
            _ => *output = Some(Err("Invalid Label, expecting __start".to_string())),
        }
    }
}

declare_plugin!(KernelPlugin, KernelPlugin::new);
