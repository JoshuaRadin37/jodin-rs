use crate::error::PluginError;
use jodin_common::assembly::value::Value;
use libloading::{Library, Symbol};
use std::any::Any;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ffi::{CStr, OsStr};
use std::path::Path;
use uuid::Uuid;

/// A reference object for pushing, popping, and peeking the stack
pub trait Stack {
    /// Checks whether the stack is empty
    fn empty(&self) -> bool;

    /// Push a value to the stack.
    ///
    /// # Panic
    /// panics if the stack is full
    fn push(&mut self, value: Value);
    /// Pop a value from the stack
    fn pop(&mut self, output: &mut Option<Value>);
}

pub trait VMHandle {
    fn native(&mut self, method: &str, values: &[Value], output: &mut Option<Value>);
}

/// A plugin which allows you to add functionality to the jodin VM
pub trait Plugin: Any + Send + Sync {
    fn labels(&self, buffer: &mut [&'static str]);

    fn labels_count(&self) -> i32;

    fn call_label(
        &self,
        label: &str,
        stack: &mut dyn Stack,
        handle: &mut dyn VMHandle,
        output: &mut Option<Result<Value, String>>,
    );
}

pub trait LoadablePlugin: Plugin {
    fn new() -> Self;
}

impl<D: Default + Plugin> LoadablePlugin for D {
    fn new() -> Self {
        D::default()
    }
}

/// The plugin manager for VMS
pub struct PluginManager {
    plugins: HashMap<Uuid, Box<dyn Plugin>>,
    loaded_labels: HashMap<String, Uuid>,
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            loaded_labels: HashMap::new(),
            loaded_libraries: Vec::new(),
        }
    }

    /// Loads a plugin for the virtual machine
    ///
    /// If successful, returns the count of created labels
    ///
    /// # Error
    ///
    /// Errors if the library was invalid or a plugin couldn't be created.
    pub unsafe fn load_plugin<P: AsRef<OsStr>>(
        &mut self,
        filename: P,
    ) -> Result<usize, PluginError> {
        type PluginCreate = unsafe fn() -> *mut dyn Plugin;

        let filename = filename.as_ref();
        let path = Path::new(filename);
        if !path.exists() {
            panic!("{:?} does not exist!", path)
        }
        let lib = Library::new(filename)?;
        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();
        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")?;
        let boxed_raw = (*constructor)();
        let plugin = Box::from_raw(boxed_raw);
        Ok(self.register_plugin(plugin))
    }

    pub fn with_plugin<P: Plugin>(&mut self, plugin: P) -> usize {
        let plugin: Box<dyn Plugin> = Box::new(plugin);
        self.register_plugin(plugin)
    }

    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> usize {
        let uuid = loop {
            let uuid = Uuid::new_v4();
            if !self.plugins.contains_key(&uuid) {
                break uuid;
            }
        };
        self.plugins.insert(uuid, plugin);
        let plugin = self.plugins.get(&uuid).unwrap();
        let mut count = plugin.labels_count() as usize;
        let mut buffer = vec![""; count];
        plugin.labels(&mut buffer);
        for label in buffer {
            self.loaded_labels.insert(label.to_string(), uuid);
            count += 1;
        }
        count
    }

    pub fn loaded_label<S: AsRef<str>>(&self, label: S) -> bool {
        self.loaded_labels.contains_key(label.as_ref())
    }

    pub fn call_function<S: Stack, V: VMHandle>(
        &self,
        label: &str,
        stack: &mut S,
        handle: &mut V,
    ) -> Result<Value, PluginError> {
        let uuid = self
            .loaded_labels
            .get(label)
            .ok_or(PluginError::LabelNotRegister(label.to_string()))?;

        let plugin = self.plugins.get(uuid).unwrap();
        let stack: &mut dyn Stack = stack;
        let mut output: Option<Result<_, _>> = None;
        plugin.call_label(label, stack, handle, &mut output);
        output
            .unwrap()
            .map_err(|s| PluginError::FunctionError(unsafe { s }))
    }
}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn $crate::Plugin {
            let constructor: fn() -> $plugin_type = $constructor;
            let object = (constructor)();
            let boxed: Box<dyn $crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
