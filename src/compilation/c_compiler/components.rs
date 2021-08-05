//! The different C99 components to get compiled into

use std::fmt::{Display, Formatter};
use std::ops::Deref;

use regex::Regex;

use crate::ast::intermediate_type::{IntermediateType, TypeSpecifier, TypeTail};
use crate::compilation::c_compiler::SeparableCompilable;
use crate::compilation::{Compilable, Context, PaddedWriter, C99};
use crate::core::error::{JodinErrorType, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::types::primitives::Primitive;
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
#[derive(Clone)]
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
#[derive(Clone)]
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
/// use jodin_rs::compilation::c_compiler::{CType, CTypeSpecifier, CTypeDeclarator};
/// use jodin_rs::core::types::primitives::Primitive::*;
///
/// // A primitive type
/// let primtive = CType::new(false, CTypeSpecifier::Primitive(Int), CTypeDeclarator::Identifier);
///
/// assert_eq!(primtive.declarator("hello"), "int hello");
/// ```
#[derive(Clone)]
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
    static ref C_ID_REGEX: Regex = Regex::new(r"[_a-zA-Z]\w").unwrap();
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
            let value = char as u8 as u64;
            hash += value << index;
        }

        let id_string: String = id.iter().collect::<Vec<_>>().join("_");
        let c_id = format!("{}{}", id_string, hash);
        CValidIdentifier(c_id)
    }

    /// Try to construct a c-valid identifier from a Jodin Identifier
    ///
    /// # Error
    /// Errors if the original id isn't valid in C.
    pub fn no_mangle(id: Identifier) -> JodinResult<Self> {
        let id = id.strip_highest_parent().unwrap();
        let to_string = id.to_string();
        if let Some(mat) = C_ID_REGEX.find(to_string.as_str()) {
            if mat.as_str() == to_string {
                Ok(CValidIdentifier(to_string))
            } else {
                Err(JodinErrorType::InvalidAsDirectCDeclaration(id).into())
            }
        } else {
            Err(JodinErrorType::InvalidAsDirectCDeclaration(id).into())
        }
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
        writeln!(
            w,
            "{}({});",
            self.return_type.declarator(self.name.as_str()),
            self.arguments
                .iter()
                .map(|(_, ty)| ty.abstract_declarator())
                .collect::<Vec<_>>()
                .join(",")
        )?;
        Ok(())
    }

    fn definition<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        writeln!(
            w,
            "{}({})",
            self.return_type.declarator(self.name.as_str()),
            self.arguments
                .iter()
                .map(|(_, ty)| ty.abstract_declarator())
                .collect::<Vec<_>>()
                .join(","),
        )?;
        self.compound_statement.compile(context, w)
    }
}

/// Represents a C compound statement.
pub struct CompoundStatement();

impl Compilable<C99> for CompoundStatement {
    fn compile<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
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
