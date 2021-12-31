//! Replace all constant expressions

use jodin_common::ast::JodinNode;
use jodin_common::core::literal::Literal;
use jodin_common::error::JodinResult;
use jodin_common::identifier::Identifier;
use std::collections::HashMap;

fn find_constant_expressions(node_tree: &JodinNode) -> JodinResult<HashMap<Identifier, Literal>> {
    let mut output = HashMap::new();

    Ok(output)
}

pub fn replace_constant_expressions(mut input: JodinNode) -> JodinResult<JodinNode> {
    let map = find_constant_expressions(&input)?;
    _replace_constant_expressions(&mut input, &map)?;
    Ok(input)
}

fn _replace_constant_expressions(
    input: &mut JodinNode,
    ids: &HashMap<Identifier, Literal>,
) -> JodinResult<()> {
    Ok(())
}
