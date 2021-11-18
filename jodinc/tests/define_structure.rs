use jodin_rs::ast::tags::TagTools;
use jodin_rs::core::error::JodinResult;
use jodin_rs::core::identifier::Identifier;
use jodin_rs::core::privacy::Visibility;
use jodin_rs::core::types::big_object::JBigObjectFactory;
use jodin_rs::core::types::primitives::Primitive;
use jodin_rs::core::types::{AsIntermediate, Field, GetResolvedMember, JodinType};
use jodin_rs::parsing::parse_program;
use jodin_rs::utility::Visitor;
use jodin_rs::{default_logging, process_jodin_node};
use logos::internal::CallbackResult;

static JODIN_STRING: &str = r"
public struct Square {
    sides: int
}

let square: Square;

";

#[test]
fn define_a_structure() -> JodinResult<()> {
    default_logging();
    let declaration = parse_program(JODIN_STRING)?;
    let (processed, env) = process_jodin_node(declaration)?;
    println!("{:#?}", processed);

    let square_ty = env.get_type_by_name(&Identifier::from("Square"))?;
    println!("{:#?}", square_ty);

    let factory = JBigObjectFactory::new(&env);

    let square_ty_o = factory.new_instance(square_ty)?;

    println!("{:#?}", square_ty_o);

    let field: &Field = square_ty_o.get_member(&Identifier::from("sides"))?;

    assert_eq!(&field.vis, &Visibility::Private);
    assert_eq!(&field.jtype, &Primitive::Int.intermediate_type());

    Ok(())
}
