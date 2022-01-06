//! Contains all of the core components of the compiler. This is where any part that is repeatedly
//! used in the project is stored.

pub mod external;
pub mod identifier_resolution;
pub mod import;
pub mod literal;
pub mod operator;
pub mod privacy;
pub mod tags;

/// This should be used as the name of the callable native object that can be used within jodin programs.
///
/// The native object is special because unlike other functions, the only type-checked parameter
/// is the first, which must be a [`Value::String`](crate::mvp::value::Value) value.
pub static NATIVE_OBJECT: &str = "__NATIVE";
