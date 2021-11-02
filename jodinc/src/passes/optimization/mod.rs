//! Contains different passes that can affect the structure of the AST

use crate::ast::JodinNode;
use crate::core::error::JodinResult;

/// Runs optimizations on a tree
pub fn optimize(node: JodinNode) -> JodinResult<JodinNode> {
    Ok(node)
}

/// Represents some struct that can optimize something
pub trait Optimizer<T, R = T> {
    /// The optimization function
    fn optimize(&self, node: T) -> JodinResult<R>;
}

// /// Represents some struct that optimizes a ref specifically. Automatically implements [Optimizer](Optimizer).
// pub trait RefOptimizer<T> {
//     /// Optimization function for the mutable ref.
//     fn optimize_ref(&self, node: &mut T) -> JodinResult<()>;
// }
//
// impl<T, O> Optimizer<&mut T, ()> for O
//     where O : RefOptimizer<T> {
//     fn optimize(&self, node: &mut T) -> JodinResult<()> {
//         RefOptimizer::optimize_ref(self, node)
//     }
// }

impl<F, T, R> Optimizer<T, R> for F
where
    F: Fn(T) -> JodinResult<R>,
{
    fn optimize(&self, node: T) -> JodinResult<R> {
        (self)(node)
    }
}
