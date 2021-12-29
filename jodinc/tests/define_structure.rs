use jodin_asm::init_logging;
use jodin_common::core::identifier_resolution::IdentifierResolver;
use jodin_common::core::privacy::Visibility;
use jodin_common::core::tags::TagTools;
use jodin_common::error::JodinResult;
use jodin_common::identifier::Identifier;
use jodin_common::types::intermediate_type::IntermediateType;
use jodin_common::types::primitives::Primitive;
use jodin_common::types::resolved_type::{
    ResolveType, ResolvedType, ResolvedTypeFactory, WeakResolvedType,
};
use jodin_common::types::{AsIntermediate, Field, GetResolvedMember, JodinType};
use jodin_common::utility::Visitor;
use jodinc::parsing::parse_program;
use jodinc::process_jodin_node;
use log::LevelFilter;
use logos::internal::CallbackResult;
use std::error::Error;

static JODIN_STRING: &str = r#"
in shapes
public struct Square {
    sides: int
}

struct TripleSquare {
    sq1: shapes::Square,
    sq2: shapes::Square,
    sq3: shapes::Square
}

"#;

#[test]
fn define_a_structure() -> Result<(), Box<dyn Error>> {
    init_logging(LevelFilter::Trace);
    let declaration = parse_program(JODIN_STRING)?;
    let (processed, env) = match process_jodin_node(declaration) {
        Err(e) => {
            return Err(Box::new(e));
        }
        Ok(v) => v,
    };

    println!("{:#?}", processed);

    let id_resolver = processed
        .property::<IdentifierResolver>("id_resolver")
        .ok_or("id resolver not available")?;

    println!("id resolver: {:#?}", id_resolver);

    let tri_square_ty = env.get_type_by_name(&Identifier::from("TripleSquare"))?;
    println!("{:#?}", tri_square_ty);

    let factory = ResolvedTypeFactory::new(&env);

    let tri_square_ty_o = factory.new_instance(tri_square_ty).upgrade()?;

    println!("{:#?}", tri_square_ty_o);

    let field: &Field<ResolvedType> = tri_square_ty_o.get_member(&Identifier::from("sq1"))?;

    assert_eq!(&field.vis, &Visibility::Public);
    assert_eq!(
        &field.jtype.intermediate_type(),
        &IntermediateType::from(Identifier::from_iter(&["shapes", "Square"]))
    );

    let sides = field.jtype.get_member(&Identifier::new("sides"))?;
    assert_eq!(
        &sides.jtype.intermediate_type(),
        &Primitive::Int.intermediate_type()
    );
    Ok(())
}
