//! The different C99 components to get compiled into

use std::fmt::{Display, Formatter};
use std::ops::Deref;

use regex::Regex;

use crate::ast::intermediate_type::{IntermediateType, TypeSpecifier, TypeTail};
use crate::compilation::c_compiler::SeparableCompilable;
use crate::compilation::{Compilable, Context, PaddedWriter, C99};
use crate::core::error::{JodinErrorType, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::literal::Literal;
use crate::core::operator::Operator;
use crate::core::types::primitives::Primitive;
use itertools::Itertools;
use std::fmt::Write;

/// Represents a C translation Unit
pub enum TranslationUnit {
    /// Represents a C declaration
    Declaration {
        /// The C type of this declaration
        c_type: CType,
        /// It's identifier
        identifier: CValidIdentifier,
    },
    /// Represents a function definition
    FunctionDefinition {
        /// The function info for this definition
        function_info: FunctionInfo,
    },
    /// A C typedef
    Typedef {
        /// The original c type
        c_type: CType,
        /// identifier
        identifier: CValidIdentifier,
    },
    /// A structure declaration
    StructureDeclaration {
        /// the name of the structure
        name: CValidIdentifier,
        /// the fields of the structure
        fields: Vec<(CType, CValidIdentifier)>,
    },
}

impl SeparableCompilable for TranslationUnit {
    fn declaration<W: Write>(&self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        match self {
            TranslationUnit::Declaration { c_type, identifier } => {
                let declaration = c_type.declarator(identifier.as_str());
                writeln!(w, "{};", declaration)?;
                Ok(())
            }
            TranslationUnit::FunctionDefinition { function_info } => {
                function_info.declaration(context, w)
            }
            TranslationUnit::Typedef { c_type, identifier } => {
                let declaration = c_type.declarator(identifier.as_str());
                writeln!(w, "typedef {};", declaration)?;
                Ok(())
            }
            TranslationUnit::StructureDeclaration { name, .. } => {
                writeln!(w, "struct {};", name)?;
                Ok(())
            }
        }
    }

    fn definition<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        match self {
            TranslationUnit::FunctionDefinition { function_info } => {
                function_info.definition(context, w)
            }
            TranslationUnit::StructureDeclaration { name, fields } => {
                writeln!(w, "struct {} {{", name)?;
                w.increase_pad();
                for (c_type, id) in fields {
                    let declaration = c_type.declarator(id.as_str());
                    writeln!(w, "{};", declaration)?;
                }
                w.decrease_pad();
                writeln!(w, "}};")?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

/// The abstract type represented in C at its use
#[derive(Clone, Debug)]
pub enum CTypeDeclarator {
    /// A pointer type
    Pointer(Box<CTypeDeclarator>),
    /// An array type
    Array {
        /// The inner declarator
        inner: Box<CTypeDeclarator>,
        /// The maybe size of the array
        maybe_size: Option<usize>,
    },
    /// Arguments passed to the type
    Arguments {
        /// The inner declarator
        inner: Box<CTypeDeclarator>,
        /// The argument types
        argument_types: Vec<CType>,
    },
    /// An identifier
    Identifier,
    /// Empty
    Empty,
}

impl CTypeDeclarator {
    fn declarator(&self, id: &str) -> String {
        match self {
            CTypeDeclarator::Pointer(inner) => {
                format!("(*{})", inner.declarator(id))
            }
            CTypeDeclarator::Array { inner, maybe_size } => {
                let tail = match maybe_size {
                    None => "[]".to_string(),
                    Some(size) => {
                        format!("[{}]", size)
                    }
                };

                format!("{}{}", inner.declarator(id), tail)
            }
            CTypeDeclarator::Arguments {
                inner,
                argument_types,
            } => {
                let collected: Vec<_> = argument_types
                    .iter()
                    .map(|arg| arg.abstract_declarator())
                    .collect();
                let tail = format!("({})", collected.join(","));

                format!("{}{}", inner.declarator(id), tail)
            }
            CTypeDeclarator::Identifier => id.to_string(),
            CTypeDeclarator::Empty => "".to_string(),
        }
    }
}

/// A type specifier
#[derive(Clone, Debug)]
pub enum CTypeSpecifier {
    /// A named structure (such as struct obj)
    NamedStruct {
        /// The name of the structure
        name: CValidIdentifier,
    },
    /// An alias for a type
    TypeDefinition {
        /// The name of the type accoring to a type definition
        name: CValidIdentifier,
    },
    ///
    Primitive(Primitive),
}

/// Represents a c type
///
/// # Examples
/// ```
/// use jodinc::ompilation::c_compiler::{CType, CTypeSpecifier, CTypeDeclarator};
/// use jodinc::ore::types::primitives::Primitive::*;
///
/// // A primitive type
/// let primtive = CType::new(false, CTypeSpecifier::Primitive(Int), CTypeDeclarator::Identifier);
///
/// assert_eq!(primtive.declarator("hello"), "int hello");
/// ```
#[derive(Clone, Debug)]
pub struct CType {
    is_const: bool,
    specifier: CTypeSpecifier,
    declarator: CTypeDeclarator,
}

impl CType {
    /// Create a new CType
    pub fn new(is_const: bool, specifier: CTypeSpecifier, declarator: CTypeDeclarator) -> Self {
        CType {
            is_const,
            specifier,
            declarator,
        }
    }
}

impl CType {
    /// Create an abstract declarator
    pub fn abstract_declarator(&self) -> String {
        self.declarator("")
    }

    /// Create a declarator with an id
    pub fn declarator(&self, id: &str) -> String {
        let specifier = match &self.specifier {
            CTypeSpecifier::NamedStruct { name } => format!("struct {}", name),
            CTypeSpecifier::TypeDefinition { name } => name.as_str().to_string(),
            CTypeSpecifier::Primitive(p) => p.to_string(),
        };

        let inner_declaration = self.declarator.declarator(id);

        format!(
            "{}{} {}",
            if self.is_const { "const " } else { "" },
            specifier,
            inner_declaration
        )
    }
}

impl From<Primitive> for CType {
    fn from(p: Primitive) -> Self {
        CType::new(
            false,
            CTypeSpecifier::Primitive(p),
            CTypeDeclarator::Identifier,
        )
    }
}

impl From<&IntermediateType> for CType {
    fn from(im: &IntermediateType) -> Self {
        let is_const = im.is_const;
        let c_specifier = match &im.type_specifier {
            TypeSpecifier::Id(id) => CTypeSpecifier::TypeDefinition {
                name: CValidIdentifier::new(id.clone()),
            },
            TypeSpecifier::Primitive(p) => CTypeSpecifier::Primitive(p.clone()),
        };
        let mut ret = CType::new(is_const, c_specifier, CTypeDeclarator::Identifier);

        for tail in &im.tails {
            let CType {
                is_const,
                specifier,
                declarator,
            } = ret;
            match tail {
                TypeTail::Pointer => {
                    ret = CType::new(
                        is_const,
                        specifier,
                        CTypeDeclarator::Pointer(Box::new(declarator)),
                    );
                }
                TypeTail::Array(Some(size)) => {
                    todo!("Need to have expression compiler created first");
                    ret = CType {
                        is_const,
                        specifier,
                        declarator,
                    };
                }
                TypeTail::Array(None) => {
                    ret = CType::new(
                        is_const,
                        specifier,
                        CTypeDeclarator::Array {
                            inner: Box::new(declarator),
                            maybe_size: None,
                        },
                    );
                }
                TypeTail::Function(func) => {
                    ret = CType::new(
                        is_const,
                        specifier,
                        CTypeDeclarator::Arguments {
                            inner: Box::new(declarator),
                            argument_types: func.into_iter().map(|im| CType::from(im)).collect(),
                        },
                    );
                }
            }
        }

        ret
    }
}

impl From<IntermediateType> for CType {
    fn from(it: IntermediateType) -> Self {
        Self::from(&it)
    }
}

impl Compilable<C99> for CType {
    fn compile<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        write!(w, "{}", self.abstract_declarator())?;
        Ok(())
    }
}

/// A identifier that is valid in C
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CValidIdentifier(String);

lazy_static! {
    static ref C_ID_REGEX: Regex = Regex::new(r"[_a-zA-Z]\w*").unwrap();
}

impl CValidIdentifier {
    /// Construct a c-valid identifier from a JodinIdentifier
    pub fn new(id: Identifier) -> Self {
        let mut hash: u64 = 0;

        let id = if id.highest_parent() == "{base}" {
            id.strip_highest_parent().unwrap()
        } else {
            id
        };

        //let id = id.strip_highest_parent().unwrap();
        for (index, char) in id.to_string().char_indices() {
            if char.is_ascii_alphanumeric() || char == '_' {
                let value = char as u8 as u64;
                hash += value << index;
            }
        }

        let id_string: String = id
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .intersperse(vec!['_'])
            .flatten()
            .filter(|char| char.is_ascii_alphanumeric() || *char == '_')
            .collect::<String>();
        let c_id = format!("{}{}", id_string, hash);
        CValidIdentifier(c_id)
    }

    /// Try to construct a c-valid identifier from a Jodin Identifier
    ///
    /// # Error
    /// Errors if the original id isn't valid in C.
    pub fn no_mangle(id: Identifier) -> JodinResult<Self> {
        let id_this = id.this();
        if let Some(mat) = C_ID_REGEX.find(id_this) {
            if mat.as_str() == id_this {
                Ok(CValidIdentifier(id_this.to_string()))
            } else {
                Err(JodinErrorType::InvalidAsDirectCDeclaration(Identifier::from(id_this)).into())
            }
        } else {
            Err(JodinErrorType::InvalidAsDirectCDeclaration(Identifier::from(id_this)).into())
        }
    }
}

impl From<&Identifier> for CValidIdentifier {
    fn from(id: &Identifier) -> Self {
        CValidIdentifier::new(id.clone())
    }
}

impl Deref for CValidIdentifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for CValidIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for CValidIdentifier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Represent a C Struct Declaration
pub struct StructInfo {
    name: CValidIdentifier,
    fields: Vec<(CValidIdentifier, CType)>,
}

impl SeparableCompilable for StructInfo {
    fn declaration<W: Write>(&self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        writeln!(w, "typedef struct {0} {0};", self.name)?;
        Ok(())
    }

    fn definition<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        writeln!(w, "struct {} {{", self.name)?;
        w.increase_pad();
        for (field, field_type) in self.fields {
            let str = field_type.declarator(field.as_str());
            writeln!(w, "{}", str)?;
        }
        w.decrease_pad();
        writeln!(w, "}};")?;
        Ok(())
    }
}

/// Represents A function definition in C
pub struct FunctionInfo {
    name: CValidIdentifier,
    return_type: CType,
    arguments: Vec<(CValidIdentifier, CType)>,
    compound_statement: CompoundStatement,
}

impl FunctionInfo {
    /// Creates a new function info instance.
    pub fn new(
        name: CValidIdentifier,
        return_type: CType,
        arguments: Vec<(CValidIdentifier, CType)>,
        compound_statement: CompoundStatement,
    ) -> Self {
        FunctionInfo {
            name,
            return_type,
            arguments,
            compound_statement,
        }
    }
}

impl SeparableCompilable for FunctionInfo {
    fn declaration<W: Write>(&self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        let return_type_name = format!("{}_return_type", self.name);
        writeln!(
            w,
            "typedef {};",
            self.return_type.declarator(&return_type_name)
        )?;
        writeln!(
            w,
            "{} {}({});",
            return_type_name,
            self.name,
            self.arguments
                .iter()
                .map(|(_, ty)| ty.abstract_declarator())
                .collect::<Vec<_>>()
                .join(",")
        )?;
        Ok(())
    }

    fn definition<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        let return_type_name = format!("{}_return_type", self.name);
        write!(
            w,
            "{} {}({}) ",
            return_type_name,
            self.name,
            self.arguments
                .iter()
                .map(|(id, ty)| ty.declarator(&id))
                .collect::<Vec<_>>()
                .join(","),
        )?;
        self.compound_statement.compile(context, w)
    }
}

/// Represents a C compound statement.
pub struct CompoundStatement {
    block_statement: Vec<Statement>,
}

impl CompoundStatement {
    /// Create an empty compound statement
    pub fn empty() -> Self {
        Self {
            block_statement: vec![],
        }
    }

    /// Create a compound statement using these statements
    pub fn new(block_statement: Vec<Statement>) -> Self {
        CompoundStatement { block_statement }
    }
}

impl Compilable<C99> for CompoundStatement {
    fn compile<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        write!(w, "{{ ")?;
        if !self.block_statement.is_empty() {
            writeln!(w)?;
            w.increase_pad();
            for statement in self.block_statement {
                statement.compile(context, w)?;
            }
            w.decrease_pad();
        }
        writeln!(w, "}}")?;
        Ok(())
    }
}

/// A statement type within C
#[derive(Debug)]
pub enum Statement {
    /// A block statement
    Block(Vec<Statement>),
    /// Declares a variable
    VariableDeclaration {
        /// The declaration type
        c_type: CType,
        /// The identifier
        identifier: CValidIdentifier,
        /// An optional initialization statement
        maybe_init: Option<Expression>,
    },
    /// Switch statement
    SwitchStatement,
    /// If statement
    IfStatement,
    /// While statement
    WhileStatement,
    /// For statement
    ForStatement,
    /// Do while statement
    DoWhileStatement,
    /// Just an expression statement
    ExpressionStatement,
    /// An assignment statement
    AssignmentStatement,
}

impl Compilable<C99> for Statement {
    fn compile<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        match self {
            Statement::Block(b) => {
                write!(w, "{{")?;
                if !b.is_empty() {
                    writeln!(w)?;
                    for statement in b {
                        statement.compile(context, w)?;
                    }
                }
                writeln!(w, "}}")?;
            }
            Statement::VariableDeclaration {
                c_type,
                identifier,
                maybe_init,
            } => {
                writeln!(w, "{};", c_type.declarator(&identifier))?;
                if let Some(expr) = maybe_init {
                    write!(w, "{} = ", identifier)?;
                    expr.compile(context, w)?;
                    writeln!(w, ";")?;
                }
            }
            Statement::SwitchStatement => {}
            Statement::IfStatement => {}
            Statement::WhileStatement => {}
            Statement::ForStatement => {}
            Statement::DoWhileStatement => {}
            Statement::ExpressionStatement => {}
            Statement::AssignmentStatement => {}
        }
        Ok(())
    }
}

/// A c expression
#[derive(Debug)]
pub enum Expression {
    /// A literal
    Literal(Literal),
    /// A variable
    Variable(CValidIdentifier),
    /// A bi operation
    Binop {
        /// Left hand side of the expression
        lhs: Box<Expression>,
        /// The operator
        op: Operator,
        /// Right hand side of the expression
        rhs: Box<Expression>,
    },
    /// A uni op
    Uniop {
        /// The relevant expression
        inner: Box<Expression>,
        /// The operator
        op: Operator,
        /// Whether its a pre-op (++i) or post-op (i++)
        is_post_op: bool,
    },
    /// Call a function
    Call {
        /// Calling expression
        to_call: Box<Expression>,
        /// Arguments to call
        arguments: Vec<Expression>,
    },
    /// Get a member from a struct
    GetMember {
        /// The parent object
        parent: Box<Expression>,
        /// The field name
        field: CValidIdentifier,
    },
    /// Apply the index function
    Index {
        /// The array
        parent: Box<Expression>,
        /// The index of the array
        index: Box<Expression>,
    },
}

impl Compilable<C99> for Expression {
    fn compile<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        match self {
            Expression::Literal(l) => {
                let as_string = match l {
                    Literal::String(s) => {
                        format!("{:?}", s)
                    }
                    Literal::Char(c) => {
                        format!("'{}'", c)
                    }
                    Literal::Boolean(b) => match b {
                        true => "1".to_string(),
                        false => "0".to_string(),
                    },
                    Literal::Float(f) => {
                        format!("{}F", f)
                    }
                    Literal::Double(d) => {
                        format!("{}L", d)
                    }
                    Literal::Byte(b) => {
                        format!("((char) {})", b)
                    }
                    Literal::Short(s) => {
                        format!("((short) {})", s)
                    }
                    Literal::Int(i) => i.to_string(),
                    Literal::Long(l) => {
                        format!("{}L", l)
                    }
                    Literal::UnsignedByte(b) => {
                        format!("((unsigned char) {})", b)
                    }
                    Literal::UnsignedShort(s) => {
                        format!("((unsigned short) {})", s)
                    }
                    Literal::UnsignedInt(i) => {
                        format!("{}U", i)
                    }
                    Literal::UnsignedLong(l) => {
                        format!("{}UL", l)
                    }
                };
                write!(w, "{}", as_string)?;
            }
            Expression::Variable(v) => {
                write!(w, "{}", v)?;
            }
            Expression::Binop { lhs, op, rhs } => {
                write!(w, "(")?;
                lhs.compile(context, w)?;
                write!(w, ") {} (", op)?;
                rhs.compile(context, w)?;
                write!(w, ")")?;
            }
            Expression::Uniop { .. } => {}
            Expression::Call { .. } => {}
            Expression::GetMember { .. } => {}
            Expression::Index { .. } => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use super::*;

    #[test]
    fn function_ptrs() {
        let func_type = CType::new(
            false,
            CTypeSpecifier::Primitive(Primitive::Int),
            CTypeDeclarator::Arguments {
                inner: Box::new(CTypeDeclarator::Pointer(Box::new(
                    CTypeDeclarator::Identifier,
                ))),
                argument_types: vec![
                    CType::new(
                        false,
                        CTypeSpecifier::Primitive(Primitive::Int),
                        CTypeDeclarator::Pointer(Box::new(CTypeDeclarator::Identifier)),
                    ),
                    CType::new(
                        false,
                        CTypeSpecifier::Primitive(Primitive::Int),
                        CTypeDeclarator::Pointer(Box::new(CTypeDeclarator::Identifier)),
                    ),
                ],
            },
        );

        let as_declarator = func_type.declarator("id");
        let as_abstract_declarator = func_type.abstract_declarator();

        assert_eq!(as_declarator, "int (*id)(int (*),int (*))");
        assert_eq!(as_abstract_declarator, "int (*)(int (*),int (*))");
    }

    #[test]
    fn identifiers_to_c_ids() {
        let complex_identifier = Identifier::from_iter(["stdio", "println"]);
        let c_id = CValidIdentifier::new(complex_identifier.clone());
        assert_ne!(c_id.to_string(), complex_identifier.to_string());
        println!("{}", c_id);
    }

    #[test]
    fn structs_compilable() {
        let structure = StructInfo {
            name: CValidIdentifier("record".to_string()),
            fields: vec![
                (
                    CValidIdentifier("field1".to_string()),
                    CType::from(Primitive::Int),
                ),
                (
                    CValidIdentifier("field2".to_string()),
                    CType::from(Primitive::Boolean),
                ),
            ],
        };
        let context = Context::new();

        let string = String::new();
        let mut padded_string = PaddedWriter::new(string).with_initial_pad(1);
        structure.declaration(&context, &mut padded_string).unwrap();
        structure.definition(&context, &mut padded_string).unwrap();
        println!("{}", padded_string.finish());
    }
}
