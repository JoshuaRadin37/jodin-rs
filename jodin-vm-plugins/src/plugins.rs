use crate::error::PluginError;
use jodin_common::assembly::instructions::Assembly;
use jodin_common::assembly::value::Value;
use jodin_common::core::function_names::CALL;
use libloading::{Library, Symbol};
use std::any::Any;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ffi::{CStr, OsStr};
use std::ops::Deref;
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

    fn call_function(&mut self, method: &str, values: &[Value], output: &mut Option<Value>) {
        let mut full_vals = vec![Value::location(method), Value::from(CALL)];
        full_vals.push(Value::from_iter(values.iter().cloned()));
        full_vals.reverse();
        self.native("invoke", &full_vals[..], output);
    }

    fn load_plugin(&mut self, plugin: Box<dyn Plugin>);
}

pub fn native<V: VMHandle + ?Sized>(
    handle: &mut V,
    method: &str,
    values: &[Value],
) -> Option<Value> {
    let mut output = None;
    handle.native(method, values, &mut output);
    output
}

pub fn call<V: VMHandle + ?Sized>(handle: &mut V, method: &str, values: &[Value]) -> Value {
    let mut output = None;
    handle.call_function(method, values, &mut output);
    output.unwrap()
}

pub fn load_plugin<V: VMHandle + ?Sized, P: IntoPlugin>(handle: &mut V, plugin: P) {
    let plugin = plugin.into_plugin();
    let plugin_boxed: Box<dyn Plugin> = Box::new(plugin);
    handle.load_plugin(plugin_boxed);
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

    fn assembly(&self) -> Assembly {
        vec![]
    }

    fn on_load(&self, handle: &mut dyn VMHandle) {}
}

impl Plugin for Box<dyn Plugin> {
    fn labels(&self, buffer: &mut [&'static str]) {
        (**self).labels(buffer)
    }

    fn labels_count(&self) -> i32 {
        (**self).labels_count()
    }

    fn call_label(
        &self,
        label: &str,
        stack: &mut dyn Stack,
        handle: &mut dyn VMHandle,
        output: &mut Option<Result<Value, String>>,
    ) {
        (**self).call_label(label, stack, handle, output)
    }

    fn assembly(&self) -> Assembly {
        (**self).assembly()
    }

    fn on_load(&self, handle: &mut dyn VMHandle) {
        (**self).on_load(handle)
    }
}

pub trait LoadablePlugin: Plugin {
    fn new() -> Self;
}

pub trait IntoPlugin {
    type Target: Plugin;

    fn into_plugin(self) -> Self::Target;
}

impl<P: Plugin> IntoPlugin for P {
    type Target = P;

    fn into_plugin(self) -> Self::Target {
        self
    }
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
    ) -> Result<&dyn Plugin, PluginError> {
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

    pub fn with_plugin<P: Plugin>(&mut self, plugin: P) -> &dyn Plugin {
        let plugin: Box<dyn Plugin> = Box::new(plugin);
        self.register_plugin(plugin)
    }

    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> &dyn Plugin {
        let uuid = loop {
            let uuid = Uuid::new_v4();
            if !self.plugins.contains_key(&uuid) {
                break uuid;
            }
        };
        self.plugins.insert(uuid, plugin);
        let plugin = self.plugins.get(&uuid).unwrap();
        let mut buffer = vec![""; plugin.labels_count() as usize];
        plugin.labels(&mut buffer);
        for label in buffer {
            self.loaded_labels.insert(label.to_string(), uuid);
        }
        &**plugin
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

#[macro_export]
macro_rules! dynamic_plugin {
    ($id:ident, $location:literal) => {
        pub struct $id {
            inner: Box<dyn $crate::plugins::Plugin>,
        }

        impl $crate::plugins::Plugin for $id {
            fn labels(&self, buffer: &mut [&'static str]) {
                self.inner.labels(buffer);
            }

            fn labels_count(&self) -> i32 {
                self.inner.labels_count()
            }

            fn call_label(
                &self,
                label: &str,
                stack: &mut dyn $crate::plugins::Stack,
                handle: &mut dyn $crate::plugins::VMHandle,
                output: &mut Option<Result<$crate::Value, String>>,
            ) {
                self.inner.call_label(label, stack, handle, output);
            }
        }

        impl $crate::plugins::LoadablePlugin for $id {
            fn new() -> Self {
                use std::path::Path;
                use $crate::plugins::Plugin;
                use $crate::{Library, Symbol};

                type PluginCreate = unsafe fn() -> *mut dyn Plugin;

                let location: &'static str = $location;
                let path = Path::new(location);
                if !path.exists() {
                    panic!("{:?} does not exist!", path)
                }
                unsafe {
                    let lib = Library::new(location)
                        .expect(format!("No library could be loaded at {location}").as_ref());

                    let constructor: Symbol<PluginCreate> = lib
                        .get(b"_plugin_create")
                        .expect("No symbol with name _plugin_create");
                    let boxed_raw = (*constructor)();
                    let plugin = Box::from_raw(boxed_raw);
                    Self { inner: plugin }
                }
            }
        }
    };
}
