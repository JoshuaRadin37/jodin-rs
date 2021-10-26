//! The differentiation of jodin nodes in the AST that allow for more complex information to be
//! stored within the AST.

use crate::core::identifier::Identifier;
use crate::core::import::Import;
use crate::core::literal::Literal;
use crate::core::operator::Operator;

use crate::core::types::intermediate_type::IntermediateType;
use crate::ast::jodin_node::JodinNode;
use crate::core::types::StorageModifier;
#[cfg(feature = "pest_parser")]
use crate::parsing::JodinRule;

/// Contains JodinNode variant information.
#[derive(Debug)]
pub enum JodinNodeType {
    /// Store an intermediate type.
    Type(IntermediateType),
    /// Store a literal.
    Literal(Literal),
    /// Store an identifier.
    Identifier(Identifier),
    /// Store variable declarations, roughly translates to `<type> (<name> (= <value>)?)*`.
    VarDeclarations {
        /// The type for the variable.
        var_type: IntermediateType,
        /// The ids being declared.
        names: Vec<JodinNode>,
        /// The maybe values the variables are being initialized with.
        values: Vec<Option<JodinNode>>,
    },
    /// Alternative to VarDeclarations that only allows for one declaration, but easier parsing
    StoreVariable {
        /// How the variable will be stored
        storage_type: StorageModifier,
        /// The name of the variable
        name: JodinNode,
        /// The type of the variable
        var_type: IntermediateType,
        /// An option initial value for this variable
        maybe_initial_value: Option<JodinNode>,
    },
    /// Stores a function definition, such as `int fibonacci(int n) { ... }`.
    FunctionDefinition {
        /// The name of the function.
        name: JodinNode,
        /// The return type.
        return_type: IntermediateType,
        /// The arguments of the function.
        arguments: Vec<JodinNode>,
        /// The associated block of code.
        block: JodinNode,
    },
    /// An external function declaration
    ExternDeclaration {
        /// Declaration
        declaration: JodinNode,
    },
    /// Stores a block
    Block {
        /// The statements that make up the block.
        expressions: Vec<JodinNode>,
    },
    /// Contains a structure definition, such as `struct s1 { int i; }`.
    StructureDefinition {
        /// The id of the struct.
        name: JodinNode,
        /// The members of the struct.
        members: Vec<JodinNode>,
    },
    /// Represents a named value, usually used as a parameter or a member of a structure.
    NamedValue {
        /// The id
        name: JodinNode,
        /// The type
        var_type: IntermediateType,
    },
    /// An operator that takes in only one argument, such as `-1`.
    Uniop {
        /// The operator
        op: Operator,
        /// The argument
        inner: JodinNode,
    },
    /// An expression to cast a value to another type.
    CastExpression {
        /// The expression to be casted
        to_type: IntermediateType,
        /// The destination type
        factor: JodinNode,
    },
    /// An operator that takes in only one argument that occurs after the value, such as `x++`.
    Postop {
        /// The operator.
        op: Operator,
        /// The argument.
        inner: JodinNode,
    },
    /// An operator that takes in two arguments, such as `1+2`.
    Binop {
        /// The operator.
        op: Operator,
        /// The left had side argument.
        lhs: JodinNode,
        /// The right had side argument.
        rhs: JodinNode,
    },
    /// Represents a ternary expression, which is a shorthand for conditional expressions. Expressed
    /// as `<cond> ? <if_true> : <if_false>`.
    Ternary {
        /// The condition.
        cond: JodinNode,
        /// The value if the condition is true.
        yes: JodinNode,
        /// The value if the condition is false.
        no: JodinNode,
    },
    /// The index operator, used mainly when getting a member of an array.
    Index {
        /// The value being indexed.
        indexed: JodinNode,
        /// The expression that is the index.
        expression: JodinNode,
    },
    /// The call operator, used to call functions or methods.
    Call {
        /// The expression being called.
        called: JodinNode,
        /// The generic types to use in the call.
        generics_instance: Vec<JodinNode>,
        /// The arguments to pass in the call.
        arguments: Vec<JodinNode>,
    },
    /// Get a member of a compound type.
    GetMember {
        /// The instance of a compound type.
        compound: JodinNode,
        /// The id of the member.
        id: JodinNode,
    },
    /// A list of top level declarations
    TopLevelDeclarations {
        /// The declarations.
        decs: Vec<JodinNode>,
    },
    /// The `in <namespace>` expression.
    InNamespace {
        /// The namespace.
        namespace: JodinNode,
        /// The part of the AST that is within this namespace.
        inner: JodinNode,
    },
    /// Contains import data.
    ImportIdentifiers {
        /// The import data.
        import_data: Import,
        /// The impacted node
        affected: JodinNode,
    },
    /// A vector of nodes, not meant to be used except as a wrapper
    NodeVector {
        /// Stores the nodes
        vec: Vec<JodinNode>,
    },
    /// Return a value from a function
    ReturnValue {
        /// An optional expression to return
        expression: Option<JodinNode>,
    },
    /// A continue statement
    Continue,
    /// A break statement
    Break {
        /// break to a labeled statement
        id: Option<Identifier>,
    },
    /// An empty node
    Empty,
    /// Super
    Super,
    /// Contains information about a constructor call
    ConstructorCall {
        /// The type being called
        name: IntermediateType,
        /// The generic parameters of the function.
        generic_parameters: Vec<IntermediateType>,
        /// The arguments of the function.
        arguments: Vec<JodinNode>,
    },

    /// Unimplemented nodes represent parts of the parse tree that can't be converted into AST (yet).
    Unimplemented {
        /// The rule that wasn't converted.
        jodin_rule: String,
        /// The string from the original code that wasn't converted.
        affected_string: String,
    },
    /// Dereference a pointer
    Dereference {
        /// What's being dereferenced
        node: JodinNode,
    },
    /// Get a pointer to a place in memory
    GetReference {
        /// The place in memory
        node: JodinNode,
    },
    /// Create a new struct with fields initialized
    StructInitializer {
        /// The identifier of the struct
        struct_id: JodinNode,
        /// A list of fields and their values
        fields_and_values: Vec<(JodinNode, JodinNode)>,
    },
    /// A branching condition
    IfStatement {
        /// The condition to check.
        cond: JodinNode,
        /// The statement to execute if the condition is true.
        statement: JodinNode,
        /// An optional statement to execute if the condition is false.
        else_statement: Option<JodinNode>,
    },
    /// A repeating statement that only executes while the condition is true
    WhileStatement {
        /// The condition to check
        cond: JodinNode,
        /// The repeated statement
        statement: JodinNode,
    },
    /// A more complex version of a while loop
    ForStatement {
        /// An optional initializer
        init: Option<JodinNode>,
        /// An optional condition to check
        cond: Option<JodinNode>,
        /// An optional statement to run after the conclusion of the statement
        delta: Option<JodinNode>,
        /// The body of the for loop
        statement: JodinNode,
    },
    /// A larger branching set of instructions
    SwitchStatement {
        /// The value being checked for the switch
        to_switch: JodinNode,
        /// Labeled with case statements
        labeled_statements: Vec<JodinNode>,
    },
    /// A do-while loop, executes statement at least once
    DoStatement {
        /// The statement
        statement: JodinNode,
        /// The condition to check
        cond: JodinNode,
    },
    /// Assign an expression to a value
    AssignmentExpression {
        /// `None` means that it's a default assignment
        /// `Some(<op>)` is an operation assignment
        maybe_assignment_operator: Option<Operator>,
        /// The left hand side of the expression, an atom
        lhs: JodinNode,
        /// The right hand side of the expression
        rhs: JodinNode,
    },
    /// Creates a heap-allocated piece of memory
    NewPointer {
        /// The value being put on the heap
        inner: JodinNode,
    },
    /// A case statement
    Case {
        /// The case, None is default case
        case: Option<JodinNode>,
        /// The case statement
        statement: JodinNode,
    },
    /// An array initializer
    RepeatedArrayInitializer {
        /// The value to repeat
        to_repeat: JodinNode,
        /// The number of repeats
        repeats: JodinNode,
    },
    /// A list (array) initializer
    ListInitializer {
        /// The values in the list
        values: Vec<JodinNode>,
    },
}

impl JodinNodeType {
    /// Convert this value into an instance of Result.
    pub fn into_result<E>(self) -> Result<JodinNode, E> {
        Ok(self.into())
    }

    /// The child JodinNodes of this variant.
    pub fn children(&self) -> impl IntoIterator<Item = &JodinNode> {
        let vector: Vec<&JodinNode> = match self {
            JodinNodeType::Type(_) => {
                vec![]
            }
            JodinNodeType::Literal(_) => {
                vec![]
            }
            JodinNodeType::Identifier(_) => {
                vec![]
            }
            JodinNodeType::VarDeclarations {
                var_type: _,
                names,
                values,
            } => {
                let mut ret = vec![];
                ret.extend(names);
                ret.extend(values.iter().filter_map(|node| node.as_ref()));
                ret
            }
            JodinNodeType::FunctionDefinition {
                name,
                return_type: _,
                arguments: parameters,
                block,
            } => {
                let mut ret = vec![name];
                ret.extend(parameters);
                ret.push(block);
                ret
            }
            JodinNodeType::Block { expressions } => expressions.into_iter().collect(),
            JodinNodeType::StructureDefinition { name, members } => {
                let mut ret = vec![name];
                ret.extend(members);
                ret
            }
            JodinNodeType::NamedValue { name, var_type: _ } => {
                vec![name]
            }
            JodinNodeType::Uniop { op: _, inner } => {
                vec![inner]
            }
            JodinNodeType::CastExpression { to_type: _, factor } => {
                vec![factor]
            }
            JodinNodeType::Postop { op: _, inner } => {
                vec![inner]
            }
            JodinNodeType::Binop { op: _, lhs, rhs } => {
                vec![lhs, rhs]
            }
            JodinNodeType::Ternary { cond, yes, no } => {
                vec![cond, yes, no]
            }
            JodinNodeType::Index {
                indexed,
                expression,
            } => {
                vec![indexed, expression]
            }
            JodinNodeType::Call {
                called,
                generics_instance,
                arguments: parameters,
            } => {
                let mut ret = vec![called];
                ret.extend(generics_instance);
                ret.extend(parameters);
                ret
            }
            JodinNodeType::GetMember { compound, id } => {
                vec![compound, id]
            }
            JodinNodeType::TopLevelDeclarations { decs } => decs.iter().collect(),
            JodinNodeType::InNamespace { namespace, inner } => {
                vec![namespace, inner]
            }
            JodinNodeType::ImportIdentifiers { .. } => {
                vec![]
            }
            JodinNodeType::Unimplemented { .. } => {
                vec![]
            }
            JodinNodeType::NodeVector { vec } => vec.iter().collect(),

            JodinNodeType::ReturnValue { expression } => expression.iter().collect(),
            JodinNodeType::Continue => {
                vec![]
            }
            JodinNodeType::Break { .. } => {
                vec![]
            }
            JodinNodeType::Empty => {
                vec![]
            }
            JodinNodeType::Super => {
                vec![]
            }
            JodinNodeType::ConstructorCall {
                name: _,
                generic_parameters: _,
                arguments,
            } => {
                let mut ret = vec![];
                ret.extend(arguments);
                ret
            }
            JodinNodeType::Dereference { node } => {
                vec![node]
            }
            JodinNodeType::GetReference { node } => {
                vec![node]
            }

            JodinNodeType::StructInitializer {
                struct_id,
                fields_and_values,
            } => {
                let mut vector = vec![struct_id];
                vector.extend(
                    fields_and_values
                        .iter()
                        .flat_map(|(first, second)| vec![first, second]),
                );
                vector
            }
            JodinNodeType::IfStatement {
                cond,
                statement,
                else_statement,
            } => {
                let mut ret = vec![cond, statement];
                if let Some(else_s) = else_statement {
                    ret.push(else_s);
                }
                ret
            }
            JodinNodeType::WhileStatement { cond, statement } => {
                vec![cond, statement]
            }
            JodinNodeType::ForStatement {
                init,
                cond,
                delta,
                statement,
            } => {
                let mut ret = vec![];
                let optionals = vec![init, cond, delta]
                    .into_iter()
                    .filter_map(|val| val.as_ref());
                ret.extend(optionals);
                ret.push(statement);
                ret
            }
            JodinNodeType::SwitchStatement {
                to_switch,
                labeled_statements,
            } => {
                let mut ret = vec![to_switch];
                ret.extend(labeled_statements);
                ret
            }
            JodinNodeType::DoStatement { statement, cond } => {
                vec![cond, statement]
            }
            JodinNodeType::ExternDeclaration { declaration } => {
                vec![declaration]
            }
            JodinNodeType::AssignmentExpression {
                maybe_assignment_operator: _,
                lhs,
                rhs,
            } => {
                vec![lhs, rhs]
            }
            JodinNodeType::NewPointer { inner } => {
                vec![inner]
            }
            JodinNodeType::StoreVariable {
                storage_type: _,
                name,
                var_type: _,
                maybe_initial_value,
            } => {
                let mut vec = vec![name];
                vec.extend(maybe_initial_value);
                vec
            }
            JodinNodeType::Case { case, statement } => {
                let mut ret = vec![];
                ret.extend(case);
                ret.push(statement);
                ret
            }
            JodinNodeType::RepeatedArrayInitializer { to_repeat, repeats } => {
                vec![to_repeat, repeats]
            }
            JodinNodeType::ListInitializer { values } => values.into_iter().collect(),
        };
        vector
    }

    /// The mutable child JodinNodes of this variant.
    pub fn children_mut(&mut self) -> impl IntoIterator<Item = &mut JodinNode> {
        let vector: Vec<&mut JodinNode> = match self {
            JodinNodeType::Type(_) => {
                vec![]
            }
            JodinNodeType::Literal(_) => {
                vec![]
            }
            JodinNodeType::Identifier(_) => {
                vec![]
            }
            JodinNodeType::VarDeclarations {
                var_type: _,
                names,
                values,
            } => {
                let mut ret = vec![];
                ret.extend(names);
                ret.extend(values.iter_mut().filter_map(|node| node.as_mut()));
                ret
            }
            JodinNodeType::FunctionDefinition {
                name,
                return_type: _,
                arguments: parameters,
                block,
            } => {
                let mut ret = vec![name];
                ret.extend(parameters);
                ret.push(block);
                ret
            }
            JodinNodeType::Block { expressions } => expressions.into_iter().collect(),
            JodinNodeType::StructureDefinition { name, members } => {
                let mut ret = vec![name];
                ret.extend(members);
                ret
            }
            JodinNodeType::NamedValue { name, var_type: _ } => {
                vec![name]
            }
            JodinNodeType::Uniop { op: _, inner } => {
                vec![inner]
            }
            JodinNodeType::CastExpression { to_type: _, factor } => {
                vec![factor]
            }
            JodinNodeType::Postop { op: _, inner } => {
                vec![inner]
            }
            JodinNodeType::Binop { op: _, lhs, rhs } => {
                vec![lhs, rhs]
            }
            JodinNodeType::Ternary { cond, yes, no } => {
                vec![cond, yes, no]
            }
            JodinNodeType::Index {
                indexed,
                expression,
            } => {
                vec![indexed, expression]
            }
            JodinNodeType::Call {
                called,
                generics_instance,
                arguments: parameters,
            } => {
                let mut ret = vec![called];
                ret.extend(generics_instance);
                ret.extend(parameters);
                ret
            }
            JodinNodeType::GetMember { compound, id } => {
                vec![compound, id]
            }
            JodinNodeType::TopLevelDeclarations { decs } => decs.iter_mut().collect(),
            JodinNodeType::InNamespace { namespace, inner } => {
                vec![namespace, inner]
            }
            JodinNodeType::ImportIdentifiers { .. } => {
                vec![]
            }
            JodinNodeType::Unimplemented { .. } => {
                vec![]
            }
            JodinNodeType::NodeVector { vec } => vec.iter_mut().collect(),
            JodinNodeType::ReturnValue { expression } => expression.iter_mut().collect(),
            JodinNodeType::Continue => {
                vec![]
            }
            JodinNodeType::Break { .. } => {
                vec![]
            }
            JodinNodeType::Empty => {
                vec![]
            }
            JodinNodeType::Super => {
                vec![]
            }
            JodinNodeType::ConstructorCall {
                name: _,
                generic_parameters: _,
                arguments,
            } => {
                let mut ret = vec![];
                ret.extend(arguments);
                ret
            }
            JodinNodeType::Dereference { node } => {
                vec![node]
            }
            JodinNodeType::GetReference { node } => {
                vec![node]
            }
            JodinNodeType::StructInitializer {
                struct_id,
                fields_and_values,
            } => {
                let mut vector = vec![struct_id];
                vector.extend(
                    fields_and_values
                        .iter_mut()
                        .flat_map(|(first, second)| vec![first, second]),
                );
                vector
            }
            JodinNodeType::IfStatement {
                cond,
                statement,
                else_statement,
            } => {
                let mut ret = vec![cond, statement];
                if let Some(else_s) = else_statement {
                    ret.push(else_s);
                }
                ret
            }
            JodinNodeType::WhileStatement { cond, statement } => {
                vec![cond, statement]
            }
            JodinNodeType::ForStatement {
                init,
                cond,
                delta,
                statement,
            } => {
                let mut ret = vec![];
                let optionals = vec![init, cond, delta]
                    .into_iter()
                    .filter_map(|val| val.as_mut());
                ret.extend(optionals);
                ret.push(statement);
                ret
            }
            JodinNodeType::SwitchStatement {
                to_switch,
                labeled_statements,
            } => {
                let mut ret = vec![to_switch];
                ret.extend(labeled_statements);
                ret
            }
            JodinNodeType::DoStatement { statement, cond } => {
                vec![cond, statement]
            }
            JodinNodeType::ExternDeclaration { declaration } => {
                vec![declaration]
            }
            JodinNodeType::AssignmentExpression {
                maybe_assignment_operator: _,
                lhs,
                rhs,
            } => {
                vec![lhs, rhs]
            }
            JodinNodeType::NewPointer { inner } => {
                vec![inner]
            }
            JodinNodeType::StoreVariable {
                storage_type: _,
                name,
                var_type: _,
                maybe_initial_value,
            } => {
                let mut vec = vec![name];
                vec.extend(maybe_initial_value);
                vec
            }
            JodinNodeType::Case { case, statement } => {
                let mut ret = vec![];
                ret.extend(case);
                ret.push(statement);
                ret
            }
            JodinNodeType::RepeatedArrayInitializer { to_repeat, repeats } => {
                vec![to_repeat, repeats]
            }
            JodinNodeType::ListInitializer { values } => values.into_iter().collect(),
        };
        vector
    }
}

impl From<JodinNodeType> for JodinNode {
    fn from(i: JodinNodeType) -> Self {
        JodinNode::new(i)
    }
}
