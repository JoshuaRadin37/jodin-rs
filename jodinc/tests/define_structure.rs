use jodin_rs::ast::tags::TagTools;
use jodin_rs::core::error::JodinResult;
use jodin_rs::parsing::parse_program;
use jodin_rs::{process_jodin_node,default_logging};

static JODIN_STRING: &str = r"
struct Square {
    sides: int
}

let square: Square;

";

#[test]
fn define_a_structure() -> JodinResult<()> {
    default_logging();
    let declaration = parse_program(JODIN_STRING)?;
    let mut processed = process_jodin_node(declaration)?;
    println!("{:#?}", processed);

    Ok(())
}
