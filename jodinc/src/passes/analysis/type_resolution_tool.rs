use std::collections::HashMap;
use std::env::var;

use itertools::Itertools;

use jodin_common::ast::JodinNodeType;
use jodin_common::ast::{CompoundType, JodinNode};
use jodin_common::core::privacy::{Visibility, VisibilityTag};
use jodin_common::core::tags::TagTools;
use jodin_common::error::{JodinErrorType, JodinResult};
use jodin_common::identifier::Identifier;
use jodin_common::types::intermediate_type::IntermediateType;
use jodin_common::types::structure::Structure;
use jodin_common::types::type_environment::{TypeEnvironment, TypeEnvironmentManager};
use jodin_common::types::Field;
use jodin_common::unit::TranslationUnit;
use jodin_common::utility::Tree;

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd)]
enum AdvTypeResolutionTarget {
    Structure,
    JTrait,
    JTraitObject,
    JObject,
}

/// The tool that builds and resolves all types
pub struct TypeResolutionTool {
    /// The environment that gets adapted
    env: TypeEnvironmentManager,
}

impl<'nodes> TypeResolutionTool {
    /// Create a new type resolution tool
    pub fn new() -> Self {
        Self {
            env: TypeEnvironmentManager::new(),
        }
    }

    /// Create a new type resolution tool
    pub fn with_translation_units(units: &[TranslationUnit]) -> Self {
        let mut tool = Self {
            env: TypeEnvironmentManager::new(),
        };
        for unit in units {
            let type_id = &unit.name;
            let jtype = &unit.jtype;
            tool.env.set_variable_type(type_id, jtype.clone());
        }
        tool
    }

    pub fn visit(&mut self, tree: &'nodes mut JodinNode) -> JodinResult<()> {
        self.visit_type_definitions(tree)?;

        Ok(())
    }

    fn visit_type_definitions(&mut self, tree: &'nodes JodinNode) -> JodinResult<()> {
        match tree.inner() {
            JodinNodeType::CompoundTypeDefinition { .. } => self.build_structure(tree)?,
            _ => {
                for child in tree.direct_children() {
                    self.visit_type_definitions(child)?;
                }
            }
        }
        Ok(())
    }

    fn build_field(&self, field_node: &'nodes JodinNode) -> JodinResult<Field<IntermediateType>> {
        if let JodinNodeType::NamedValue { name, var_type } = field_node.inner() {
            let name = name.resolved_id()?;
            let visibility = field_node
                .get_tag::<VisibilityTag>()
                .map_or(Visibility::Public, |tag| tag.visibility().clone());
            let field = Field::new(visibility, var_type.clone(), name.clone());
            Ok(field)
        } else {
            Err(JodinErrorType::IllegalTreeType.into())
        }
    }

    fn build_structure(&mut self, structure_node: &'nodes JodinNode) -> JodinResult<()> {
        if let JodinNodeType::CompoundTypeDefinition {
            compound_type: CompoundType::Structure,
            name,
            inheritance: Option::None,
            members,
        } = structure_node.inner()
        {
            let name = name.resolved_id()?.clone();
            let mut fields = vec![];
            for member in members {
                let field = self.build_field(member)?;
                fields.push(field);
            }
            let structure = Structure::new(name, fields);
            self.env.store_type(structure, Some(structure_node))?;
            Ok(())
        } else {
            Err(JodinErrorType::IllegalTreeType.into())
        }
    }

    /// Finishes the resolution tool, returning the generated type environemnt
    pub fn finish(self) -> TypeEnvironment {
        self.env.finish()
    }
}
