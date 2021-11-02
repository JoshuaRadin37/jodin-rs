//! Replace all constant expressions

use std::collections::HashMap;
use crate::ast::JodinNode;
use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::literal::Literal;

fn find_constant_expressions(node_tree: &JodinNode) -> JodinResult<HashMap<Identifier, Literal>> {
    let mut output = HashMap::new();

    Ok(output)
}

pub fn replace_constant_expressions(mut input: JodinNode) -> JodinResult<JodinNode> {
    let map = find_constant_expressions(&input)?;
    _replace_constant_expressions(&mut input, &map)?;
    Ok(input)
}

fn _replace_constant_expressions(input: &mut JodinNode, ids: &HashMap<Identifier, Literal>) -> JodinResult<()> {



    Ok(())
}