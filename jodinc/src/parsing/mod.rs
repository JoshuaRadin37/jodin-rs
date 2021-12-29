//! Different modules that make up the parsing mechanisms for jodin. The parsing system is based
//! off of the pest crate.

use jodin_common::ast::{JodinNode, JodinNodeType};
use jodin_common::error::{JodinError, JodinErrorType, JodinResult};
use std::fmt::{Display, Formatter};

use jodin_common::core::operator::Operator;

use anyhow::anyhow;
use jodin_common::types::intermediate_type::IntermediateType;
use jodin_common::utility::Flatten;
use logos::{Lexer, Logos, Skip, SpannedIter};
use regex::Regex;
use std::str::FromStr;

// pub mod jodin_grammar;
lalrpop_mod!(#[allow(missing_docs)] pub jodin_grammar, "/parsing/jodin_grammar.rs");

/// Shorthand for the result given by the Lexer
pub type Spanned<Tok, Loc> = Result<(Loc, Tok, Loc), JodinError>;

/// The jodin lexer
pub struct JodinLexer<'input> {
    original: &'input str,
    lexer: SpannedIter<'input, Tok<'input>>,
}

impl<'input> JodinLexer<'input> {
    /// Create a new lexer from an input
    pub fn new(input: &'input str) -> Self {
        JodinLexer {
            original: input,
            lexer: Tok::lexer(input).spanned(),
        }
    }
}

impl<'input> Iterator for JodinLexer<'input> {
    type Item = Spanned<Tok<'input>, usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((tok, span)) = self.lexer.next() {
            match tok {
                Tok::Error => {
                    return Some(Err(JodinErrorType::LexerError(
                        self.original[span].to_string(),
                    )
                    .into()))
                }
                tok => Some(Ok((span.start, tok, span.end))),
            }
        } else {
            None
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Logos)]
pub enum Tok<'input> {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r"0[xX][a-fA-F0-9]+(u|U|l|L)*")]
    #[regex(r"0[0-9]+(u|U|l|L)*")]
    #[regex(r"[0-9]+(u|U|l|L)*")]
    #[regex(r"[a-zA-Z_]?'(.|[^'])+'")]
    #[regex(r"[0-9]+[Ee][+-]?[0-9]+(f|F|l|L)?")]
    #[regex(r"\.[0-9]+([Ee][+-]?[0-9]+)?(f|F|l|L)?")]
    #[regex(r"(\d+\.\d+)([lL]?)")]
    Constant(&'input str),
    #[regex(r#""(\\.|[^\\"])*""#)]
    #[regex(r#"\(\*""#, string)]
    StringLiteral(&'input str),
    #[token("break")]
    Break,
    #[token("case")]
    Case,
    #[token("char")]
    Char,
    #[token("const")]
    Const,
    #[token("continue")]
    Continue,
    #[token("default")]
    Default,
    #[token("double")]
    Double,
    #[token("do")]
    Do,
    #[token("else")]
    Else,
    #[token("float")]
    Float,
    #[token("for")]
    For,
    #[token("if")]
    If,
    #[token("int")]
    Int,
    #[token("long")]
    Long,
    #[token("return")]
    Return,
    #[token("short")]
    Short,
    #[token("static")]
    Static,
    #[token("typedef")]
    Typedef,
    #[token("union")]
    Union,
    #[token("unsigned")]
    Unsigned,
    #[token("struct")]
    Struct,
    #[token("void")]
    Void,
    #[token("while")]
    While,
    #[token("class")]
    Class,
    #[token("public")]
    Public,
    #[token("private")]
    Private,
    #[token("new")]
    New,
    #[token("super")]
    Super,
    #[token("virtual")]
    Virtual,
    #[token("sizeof")]
    Sizeof,
    #[token("boolean")]
    Boolean,
    #[token("in")]
    In,
    #[token("implement")]
    Implement,
    #[token("internal")]
    Internal,
    #[token("using")]
    Using,
    #[token("typeof")]
    Typeof,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("abstract")]
    Abstract,
    #[token("is")]
    Is,
    #[token("trait")]
    Trait,
    #[token("enum")]
    Enum,
    #[token("switch")]
    Switch,
    #[token("as")]
    As,
    #[token("...")]
    Varargs,
    #[token("::")]
    Namespaced,
    #[token(",")]
    Comma,
    #[token("[")]
    LBrac,
    #[token("]")]
    RBrac,
    #[token("{")]
    LCurl,
    #[token("}")]
    RCurl,
    #[token("(")]
    LPar,
    #[token(")")]
    RPar,
    #[token(";")]
    Semic,
    #[token(":")]
    Colon,
    #[token("?")]
    Qmark,
    #[regex(r"__\w+", priority = 100)]
    SpecialKeyword(&'input str),
    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,
    #[token("==", |_| Operator::Equal)]
    #[token("!=", |_| Operator::Nequal)]
    #[token("<=", |_| Operator::Lte)]
    #[token(">=", |_| Operator::Gte)]
    #[regex(r"[+\-*/&%<>!^|][\*/&%<>!^|]?", |lex| Operator::from_str(lex.slice()))]
    Op(Operator),
    #[token(".")]
    Dot,
    #[token("=")]
    Assign,
    #[regex(r"[+\-*/&%^|]=", maybe_assign_operator, priority = 10)]
    OpAssign(Operator),
    #[token("->")]
    Point,
    #[token("fn")]
    FunctionPointer,
    #[token("let")]
    Let,
    #[token("foreach")]
    Foreach,
    #[token("extern")]
    Extern,
    #[regex(r"[a-zA-Z_]\w*")]
    #[regex(r"@[a-zA-Z_]\w*", |lex| &lex.source()[1..])]
    Identifier(&'input str),
    #[regex(r"//.*", logos::skip)]
    #[token("/*", comment)]
    Comment,
    #[error]
    Error,
}

impl Display for Tok<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A part of an expression
pub enum ExpressionMember {
    /// An indivisible part of the expression
    Factor(JodinNode),
    /// An operator in the expression
    Op(Operator),
}

/// Turns a list of expression members into an AST tree
pub fn into_order_of_operations(segments: Vec<ExpressionMember>) -> JodinNode {
    let mut expression_stack = vec![];
    let mut op_stack: Vec<Operator> = vec![];

    let consume_op = |expression_stack: &mut Vec<JodinNode>, op_stack: &mut Vec<Operator>| {
        if let (Some(lhs), Some(rhs)) = {
            let mut collected = expression_stack.drain(expression_stack.len() - 2..);
            (collected.next(), collected.next())
        } {
            let op = op_stack.pop().unwrap();
            let node: JodinNode = JodinNodeType::Binop { op, lhs, rhs }.into();
            expression_stack.push(node);
        } else {
            panic!()
        }
    };

    for segment in segments {
        match segment {
            ExpressionMember::Factor(factor) => {
                expression_stack.push(factor);
            }
            ExpressionMember::Op(op) => {
                loop {
                    if let Some(peek) = op_stack.last() {
                        if peek.as_precedence() <= op.as_precedence() {
                            consume_op(&mut expression_stack, &mut op_stack);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                op_stack.push(op);
            }
        }
    }

    while !op_stack.is_empty() {
        consume_op(&mut expression_stack, &mut op_stack);
    }

    assert_eq!(expression_stack.len(), 1);
    expression_stack.pop().unwrap()
}

fn maybe_assign_operator<'input>(lex: &mut Lexer<'input, Tok<'input>>) -> JodinResult<Operator> {
    let full_operator: &str = lex.slice();
    let regex = Regex::new(r"(?P<operator>.[^=]?)=").unwrap();
    if let Some(captures) = regex.captures(full_operator) {
        let op = captures
            .name("operator")
            .expect("operator group should exist");
        let operator = op.as_str();
        Operator::from_str(operator)
    } else {
        Err(JodinErrorType::LexerError(full_operator.to_string()).into())
    }
}

fn comment<'input>(lex: &mut Lexer<'input, Tok<'input>>) -> Skip {
    loop {
        let remaining: &str = lex.remainder();
        if remaining.len() < 2 {
            panic!("Unterminated block comment")
        } else {
            let string = &remaining[..2];
            if string == "*/" {
                lex.bump(string.bytes().len());
                break;
            } else {
                let one_char = &remaining[..1];
                lex.bump(one_char.bytes().len());
            }
        }
    }
    Skip
}

fn string<'input>(lex: &mut Lexer<'input, Tok<'input>>) -> &'input str {
    let mut count = 0;
    let mut remainder = &lex.source()[count..(count + 3)];
    while remainder != r#""*)"# {
        count += 1;
        remainder = &lex.source()[count..(count + 3)];
    }
    let found: &'input str = &lex.remainder()[..count];
    lex.bump(found.bytes().len());
    let ret = &lex.source()[..(count + 3)];
    ret
}

trait UnwrapVector<T, E> {
    fn unwrap_vec(self) -> Result<Vec<T>, E>;
}

impl<T, E> UnwrapVector<T, E> for Vec<Result<T, E>> {
    fn unwrap_vec(self) -> Result<Vec<T>, E> {
        let mut ret = vec![];
        for result in self {
            ret.push(result?);
        }
        Ok(ret)
    }
}

#[macro_use]
#[allow(unused)]
macro_rules! parse {
    ($parser:ty, $ex:expr) => {{
        let string: &str = $ex;
        let lexer = $crate::parsing::JodinLexer::new(string);
        let parser = <$parser>::new();
        parser.parse(string, lexer)
    }};
}

type ParseResult = JodinResult<JodinNode>;

/// Parse an expression into a parse result
pub fn parse_expression<S: AsRef<str>>(expr: S) -> ParseResult {
    Flatten::flatten(
        parse!(jodin_grammar::ExpressionParser, expr.as_ref())
            .map_err(|e| JodinError::from(anyhow!("{}", e))),
    )
}

pub fn parse_type<S: AsRef<str>>(expr: S) -> JodinResult<IntermediateType> {
    parse!(jodin_grammar::CanonicalTypeParser, expr.as_ref())
        .map_err(|e| JodinError::from(anyhow!("{}", e)))
}

pub fn parse_program<S: AsRef<str>>(expr: S) -> ParseResult {
    Flatten::flatten(
        parse!(jodin_grammar::JodinFileParser, expr.as_ref())
            .map_err(|e| JodinError::from(anyhow!("{}", e))),
    )
}

#[allow(unused_results)]
mod tests {
    use super::jodin_grammar;
    use crate::parsing::{JodinLexer, Tok};
    use jodin_common::ast::{JodinNode, JodinNodeType};
    use jodin_common::core::literal::Literal;
    use jodin_common::core::operator::Operator;
    use jodin_common::identifier::Identifier;
    use jodin_common::types::primitives::Primitive;
    use jodin_common::types::Type;
    use std::iter::FromIterator;
    use std::str::FromStr;

    #[test]
    fn lex_identifiers() {
        let string = "+=";
        let mut lexer = JodinLexer::new(string);
        match lexer.next() {
            Some(Ok((_, Tok::OpAssign(Operator::Plus), _))) => {}
            v => {
                panic!("Didn't lex correctly: {:?}", v)
            }
        }
    }

    #[test]
    fn parse_identifiers() {
        let string = "std::mod::hello";
        let lexer = JodinLexer::new(string);
        let identifier_parser = jodin_grammar::IdentifierParser::new();
        assert_eq!(
            identifier_parser.parse(string, lexer).unwrap(),
            Identifier::from_iter(["std", "mod", "hello"])
        );
        assert!(parse!(jodin_grammar::IdentifierParser, "int").is_err());
        assert!(parse!(jodin_grammar::IdentifierParser, "@int").is_ok());
    }

    #[test]
    fn parse_constant() {
        assert_eq!(
            parse!(jodin_grammar::LiteralParser, "10").unwrap(),
            Literal::Int(10)
        );
        // assert!(parse!(jodin_grammar::LiteralParser, "10.1").is_ok());
        // assert!(parse!(jodin_grammar::LiteralParser, ".1F").is_ok());
        assert!(parse!(jodin_grammar::LiteralParser, "0xAFD").is_ok());
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            parse!(jodin_grammar::StringParser, r#""hello, world!""#).unwrap(),
            Literal::String("hello, world!".to_string())
        );
        assert_eq!(
            parse!(jodin_grammar::StringParser, r#"(*"hello, world!"*)"#).unwrap(),
            Literal::String("hello, world!".to_string())
        );
    }

    #[test]
    fn parse_type() {
        let x = parse!(jodin_grammar::CanonicalTypeParser, "[int: 5]").unwrap();
        assert_eq!(
            x,
            Primitive::Int
                .as_intermediate()
                .with_array(JodinNode::from(JodinNodeType::Literal(Literal::Int(5))))
                .unwrap()
        );
    }

    #[test]
    fn parse_id_list() {
        assert!(parse!(jodin_grammar::IdentifierListParser, "").is_ok());
        assert!(parse!(jodin_grammar::IdentifierListParser, "hello").is_ok());
        assert!(parse!(jodin_grammar::IdentifierListParser, "hello,").is_err());
        assert!(parse!(jodin_grammar::IdentifierListParser, "hello, my_name").is_ok());
        assert!(
            parse!(jodin_grammar::IdentifierListParser, "hello, __my_name").is_err(),
            "@my_name is a special keyword, not an identifier"
        );
        assert!(parse!(jodin_grammar::IdentifierListParser, "hello, my_name,").is_err());
        let ids = parse!(jodin_grammar::IdentifierListParser, "hello, my_name, bleh").unwrap();
        assert_eq!(ids, vec!["hello", "my_name", "bleh"]);
    }

    #[test]
    fn parse_atom_modifier() {
        assert!(parse!(jodin_grammar::AtomModifierParser, "value").is_ok());
        assert!(parse!(jodin_grammar::AtomModifierParser, "value()").is_ok());
        assert!(parse!(jodin_grammar::AtomModifierParser, "value(1,2,3)").is_ok());
        assert!(parse!(jodin_grammar::AtomModifierParser, "(value)(1,2)").is_ok());
        assert!(parse!(jodin_grammar::AtomModifierParser, "(value)(1,2,)").is_err());
    }

    #[test]
    fn parse_expression() {
        parse!(jodin_grammar::ExpressionParser, "1+(2-3)/5==8<9").unwrap();
        super::parse_expression("3 || 2").unwrap();
    }

    #[test]
    fn parse_statement() {
        parse!(jodin_grammar::StatementParser, "a = 3;").unwrap();
        parse!(jodin_grammar::StatementParser, "a[0] = 3;").unwrap();
        parse!(jodin_grammar::StatementParser, "a.hello[3].beep = 3;").unwrap();
        parse!(jodin_grammar::StatementParser, "if (true) { }").unwrap();
        parse!(jodin_grammar::StatementParser, "if (true) { } else { }").unwrap();
        parse!(
            jodin_grammar::StatementParser,
            "if (false) { } else if (true) { }"
        )
        .unwrap();
        parse!(jodin_grammar::StatementParser, "while (false) { }").unwrap();
        parse!(jodin_grammar::StatementParser, "return true;").unwrap();
        parse!(jodin_grammar::StatementParser, "return;").unwrap();
        parse!(jodin_grammar::StatementParser, "int a = 4;")
            .expect_err("c-style declarations no longer supported");
        parse!(jodin_grammar::StatementParser, "let a: int = 3*2;").unwrap();
        parse!(jodin_grammar::StatementParser, "let a: fn() -> int;").unwrap();
        parse!(jodin_grammar::StatementParser, "for(;;) { }").unwrap();
        parse!(
            jodin_grammar::StatementParser,
            "for(let i: int = 0; i < 2; ++i) { }"
        )
        .unwrap();
    }

    #[test]
    fn parse_types() {
        parse!(jodin_grammar::CanonicalTypeParser, "int").unwrap();
        parse!(jodin_grammar::CanonicalTypeParser, "*int").unwrap();
        parse!(jodin_grammar::CanonicalTypeParser, "fn(*int) -> void").unwrap();
        parse!(jodin_grammar::CanonicalTypeParser, "Array<int>").unwrap();
        parse!(jodin_grammar::CanonicalTypeParser, "[int: 5]").unwrap();
        // is c giberish easier to understand?
        parse!(
            jodin_grammar::CanonicalTypeParser,
            "fn() -> [fn() -> char: 5]"
        )
        .unwrap();
    }

    #[test]
    fn parse_function_definition() {
        parse!(
            jodin_grammar::FunctionDefinitionParser,
            r"
        fn main() {

        }
       "
        )
        .unwrap();
        parse!(
            jodin_grammar::FunctionDefinitionParser,
            r"
        fn main(argc: int, argv: [argv]) {

        }
       "
        )
        .unwrap();
        parse!(
            jodin_grammar::FunctionDefinitionParser,
            r"
        fn main(argc: int, argv: [argv]) -> int {

        }
       "
        )
        .unwrap();
        parse!(
            jodin_grammar::FunctionDefinitionParser,
            r"
        fn fibonacci(n: unsigned int) -> unsigned int {
            switch(n) {
                case 0:
                    return 0;
                case 1:
                    return 1;
                default:
                    return fibonacci(n-1) + fibonacci(n-2);
            }
        }
       "
        )
        .unwrap();

        //  parse!(
        //      jodin_grammar::FunctionDefinitionParser,
        //      r"
        //  fn fibonacci(n: unsigned int) -> unsigned int {
        //      static table: *[unsigned int] = new [0u: n+1];
        //      if (n < 2) {
        //          return n;
        //      }
        //      (*table)[1] = 1;
        //      for (i: int in range(2, n+1)) {
        //          (*table)[i] = (*table)[i-1] + (*table)[i-2];
        //      }
        //      return (*table)[n];
        //  }
        // "
        //  )
        //  .unwrap();
    }

    #[test]
    fn parse_structure_definition() {
        parse!(
            jodin_grammar::StructureDefinitionParser,
            r"
            struct Hello {
            
            }
        "
        )
        .unwrap();
        parse!(
            jodin_grammar::StructureDefinitionParser,
            r"
            struct Hello {
                value: int
            }
        "
        )
        .unwrap();
        parse!(
            jodin_grammar::StructureDefinitionParser,
            r"
            struct Hello {
                value: int,
                value2: short
            }
        "
        )
        .unwrap();
    }
}
