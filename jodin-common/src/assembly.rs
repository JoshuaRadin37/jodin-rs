//! The minimum viable product for compilation

pub mod asm_block;
pub mod asm_macros;
pub mod error;
pub mod instructions;
pub mod location;
pub mod value;

pub mod prelude {
    pub use crate::block;

    pub use super::instructions::*;
}
