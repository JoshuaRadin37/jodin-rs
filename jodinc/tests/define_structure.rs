use jodinc::ast::tags::TagTools;
use jodinc::core::error::JodinResult;
use jodinc::core::identifier::Identifier;
use jodinc::core::identifier_resolution::IdentifierResolver;
use jodinc::core::privacy::Visibility;
use jodinc::core::types::intermediate_type::IntermediateType;
use jodinc::core::types::primitives::Primitive;
use jodinc::core::types::resolved_type::{
    ResolveType, ResolvedType, ResolvedTypeFactory, WeakResolvedType,
};
use jodinc::core::types::{AsIntermediate, Field, GetResolvedMember, JodinType};
use jodinc::parsing::parse_program;
use jodinc::utility::Visitor;
use jodinc::{default_logging, init_logging, process_jodin_node};
use log::LevelFilter;
use logos::internal::CallbackResult;
use std::error::Error;

static JODIN_STRING: &str = r#"
public struct Square {
    sides: int
}

struct TripleSquare {
    sq1: Square,
    sq2: Square,
    sq3: Square
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
        &IntermediateType::from(Identifier::from("Square"))
    );

    let sides = field.jtype.get_member(&Identifier::new("sides"))?;
    assert_eq!(
        &sides.jtype.intermediate_type(),
        &Primitive::Int.intermediate_type()
    );
    Ok(())
}
