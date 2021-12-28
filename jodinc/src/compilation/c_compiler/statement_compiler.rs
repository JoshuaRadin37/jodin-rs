use crate::compilation::c_compiler::{CType, CValidIdentifier, Expression, Statement};
use crate::compilation::{MicroCompiler, C99};
use jodin_common::ast::{JodinNode, JodinNodeType};
use jodin_common::core::literal::Literal;
use jodin_common::core::tags::ResolvedIdentityTag;
use jodin_common::core::types::TypeTag;
use jodin_common::error::{JodinErrorType, JodinResult};
use jodin_common::identifier::Identifier;

/// Compiles a statement into a list of c statements
pub struct StatementCompiler {
    temp_count: Vec<usize>,
}

impl StatementCompiler {
    /// Create a new statement compiler
    pub fn new() -> Self {
        StatementCompiler {
            temp_count: vec![0],
        }
    }

    fn create_temp_identifier(&mut self) -> CValidIdentifier {
        let count = self.temp_count.last_mut().unwrap();
        let id_num = *count;
        *count += 1;
        let identifier =
            CValidIdentifier::no_mangle(Identifier::from(format!("__temp_var_{}", id_num)))
                .expect("Should not fail to create a c valid identifier");
        identifier
    }

    fn compile_block(&mut self, tree: &JodinNode) -> JodinResult<Vec<Statement>> {
        if let JodinNodeType::Block { expressions } = tree.inner() {
            self.temp_count.push(0);
            let mut ret = vec![];
            for exp in expressions {
                let created = self.create_compilable(exp)?;
                ret.extend(created);
            }
            self.temp_count.pop();
            Ok(ret)
        } else {
            Err(JodinErrorType::IllegalTreeType.into())
        }
    }

    fn compile_expression(
        &mut self,
        tree: &JodinNode,
    ) -> JodinResult<(Vec<Statement>, Expression)> {
        match tree.inner() {
            JodinNodeType::Identifier(_) => {
                let id = tree.get_tag::<ResolvedIdentityTag>()?.absolute_id();
                let expression = Expression::Variable(CValidIdentifier::from(id));
                return Ok((vec![], expression));
            }
            JodinNodeType::Literal(literal) => {
                let expression = Expression::Literal(literal.clone());
                return Ok((vec![], expression));
            }
            JodinNodeType::Uniop { .. } => {}
            JodinNodeType::CastExpression { .. } => {}
            JodinNodeType::Postop { .. } => {}
            JodinNodeType::Binop { op, lhs, rhs } => {
                let lhs_type = lhs.get_tag::<TypeTag>()?.jodin_type();
                let rhs_type = rhs.get_tag::<TypeTag>()?.jodin_type();
                let (before_1, lhs) = self.compile_expression(lhs)?;
                let (before_2, rhs) = self.compile_expression(rhs)?;
                let mut statements = vec![];
                statements.extend(before_1);
                statements.extend(before_2);

                let lhs = match lhs {
                    Expression::Variable(_) | Expression::Literal(_) => lhs,
                    other => {
                        let temp = self.create_temp_identifier();
                        let statement = Statement::VariableDeclaration {
                            c_type: lhs_type.into(),
                            identifier: temp.clone(),
                            maybe_init: Some(other),
                        };
                        statements.push(statement);
                        Expression::Variable(temp)
                    }
                };

                let rhs = match rhs {
                    Expression::Variable(_) | Expression::Literal(_) => rhs,
                    other => {
                        let temp = self.create_temp_identifier();
                        let statement = Statement::VariableDeclaration {
                            c_type: rhs_type.into(),
                            identifier: temp.clone(),
                            maybe_init: Some(other),
                        };
                        statements.push(statement);
                        Expression::Variable(temp)
                    }
                };

                let expression = Expression::Binop {
                    lhs: Box::new(lhs),
                    op: *op,
                    rhs: Box::new(rhs),
                };
                return Ok((statements, expression));
            }
            JodinNodeType::Ternary { .. } => {}
            JodinNodeType::Index { .. } => {}
            JodinNodeType::Call { .. } => {}
            JodinNodeType::GetMember { .. } => {}
            JodinNodeType::ConstructorCall { .. } => {}
            JodinNodeType::Dereference { .. } => {}
            JodinNodeType::GetReference { .. } => {}
            JodinNodeType::StructInitializer { .. } => {}
            _ => {
                println!("IllegalTreeType: {:?}", tree);
                return Err(JodinErrorType::IllegalTreeType.into());
            }
        }
        return Ok((vec![], Expression::Literal(Literal::Int(0))));
    }
}

impl MicroCompiler<C99, Vec<Statement>> for StatementCompiler {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<Vec<Statement>> {
        let mut ret = vec![];
        match tree.inner() {
            JodinNodeType::VarDeclarations {
                var_type,
                names,
                values,
            } => {
                let c_type = CType::from(var_type);

                let decs_iter = names.iter().zip(values);

                for (name, value) in decs_iter {
                    let id = name.get_tag::<ResolvedIdentityTag>()?.absolute_id().into();
                    let maybe_init = if let Some(value) = value {
                        let (statements, expression) = self.compile_expression(value)?;
                        ret.extend(statements);
                        Some(expression)
                    } else {
                        None
                    };
                    let statement = Statement::VariableDeclaration {
                        c_type: c_type.clone(),
                        identifier: id,
                        maybe_init,
                    };
                    ret.push(statement);
                }
            }
            JodinNodeType::Block { .. } => {
                ret.extend(self.compile_block(tree)?);
            }
            JodinNodeType::Uniop { .. } => {}
            JodinNodeType::CastExpression { .. } => {}
            JodinNodeType::Postop { .. } => {}
            JodinNodeType::Binop { .. } => {}
            JodinNodeType::Ternary { .. } => {}
            JodinNodeType::Index { .. } => {}
            JodinNodeType::Call { .. } => {}
            JodinNodeType::GetMember { .. } => {}
            JodinNodeType::ReturnValue { .. } => {}
            JodinNodeType::Continue => {}
            JodinNodeType::Break { .. } => {}
            JodinNodeType::ConstructorCall { .. } => {}
            JodinNodeType::Dereference { .. } => {}
            JodinNodeType::GetReference { .. } => {}
            JodinNodeType::IfStatement { .. } => {}
            JodinNodeType::WhileStatement { .. } => {}
            JodinNodeType::ForStatement { .. } => {}
            JodinNodeType::SwitchStatement { .. } => {}
            JodinNodeType::DoStatement { .. } => {}
            JodinNodeType::AssignmentExpression { .. } => {}
            _ => return Err(JodinErrorType::IllegalTreeType.into()),
        }
        println!("Created Statements: {:#?}", ret);
        Ok(ret)
    }
}
