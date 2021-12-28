//! This module for the implementation of the incremental compilation of the jodin vm target
//! See feature [#74] for more information on the implementation
//!
//! # Basics:
//!The compiler should allow for incremental compilation.
//!
//!The compiler should allow for directories to be specified that contains compiler jodin code that can be used by later compilation units. This should allow for incremental compilation as well. Unsure how to fully implement this yet.
//!
//! # Implementation
//! The output file type should be `.jobj`, and should output based on namespace and not declaration file.
//! Precompiled units can either be given as individual `.jobj` files, or as zip files using `DEFLATE`
//! with the extension `.jdp`.
//!
//! All `.jobj` files should contain a magic number at the beginning of the file that:
//!
//! Confirms that this is a jodin object file
//! Can be interpreted by the target Jodin Virtual Machine.
//! The compiler should be given directories or .jdp files as inputs for compilation.
//!
//! # Considerations
//! When a file already exists that is being used for compilation, the compiler should rewrite the target file.
//!
//! [#74]: https://github.com/joshradin/jodin-rs/issues/74

pub mod incremental_compiler;
