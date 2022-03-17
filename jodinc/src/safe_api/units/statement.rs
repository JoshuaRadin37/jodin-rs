use crate::safe_api::error::CompilationError;
use crate::safe_api::units::Expr;
use crate::safe_api::JodinCompilable;
use anyhow::anyhow;
use jodin_common::ast::{JodinNode, TryJodinNode};
use jodin_common::core::privacy::Visibility;
use jodin_common::identifier::Identifier;

pub enum Statement {
    Block(Block),
    Return(Expr),
    VarDec(VarDec),
    VarSet(VarSet),
    Expr(Expr),
    If(If),
    While(While),
    For(For),
    Empty,
}

pub struct Block {
    statements: Vec<Statement>,
}

pub struct VarDec {
    visibility: Option<Visibility>,
    var_id: Identifier,
    value: Option<Expr>,
}

pub struct VarSet {
    var_id: Identifier,
    value: Expr,
}

pub struct If {
    cond: Expr,
    if_true: Box<Statement>,
    if_false: Option<Box<Statement>>,
}

pub struct While {
    cond: Expr,
    if_true: Box<Statement>,
}

pub struct For {
    init: Option<Expr>,
    cond: Option<Expr>,
    delta: Option<Expr>,
    statement: Box<Statement>,
}

impl JodinCompilable<Statement> for JodinNode {
    type Error = CompilationError;

    fn compile_to(self) -> Result<Statement, Self::Error> {
        self.tryer()
            .try_with(|node, helper| helper.err(node, anyhow!("Error")))?
    }
}
