//! Different modules that make up the parsing mechanisms for jodin. The parsing system is based
//! off of the pest crate.

#![allow(missing_docs)]

#[cfg(feature = "pest_parser")]
use pest::iterators::Pairs;
#[cfg(feature = "pest_parser")]
use pest::Parser;
#[cfg(feature = "pest_parser")]
use pest_derive::Parser;

use crate::ast::{JodinNode, JodinNodeType};
use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use crate::core::literal::Literal;
use crate::core::operator::{Operator, Precedence};
use itertools::Itertools;
use logos::internal::CallbackResult;
use logos::{Lexer, Logos, Skip, SpannedIter};
use regex::Regex;
use std::str::{CharIndices, FromStr};

/// The JodinParser. Used the pest engine to perform parsing.
#[cfg(all(feature = "pest_parser", not(feature = "larlpop_parser")))]
#[derive(Parser, Debug)]
#[grammar = "parsing/jodin_grammar.pest"]
pub struct JodinParser;

/// Type alias to allow for easier refactoring
#[cfg(feature = "pest_parser")]
pub type JodinRule = Rule;

/// Attempts to parse an input string using a JodinRule.
///
/// Returns an error if it fails.
#[cfg(feature = "pest_parser")]
pub fn complete_parse(rule: Rule, input: &str) -> JodinResult<Pairs<Rule>> {
    let result: Result<_, pest::error::Error<_>> = JodinParser::parse(rule, input);
    result.map_err(|err| JodinErrorType::ParserError(err, None).into())
    //Ok(result)
}

#[cfg(test)]
#[cfg(feature = "pest_parser")]
mod tests {
    use pest::Parser;

    use crate::parsing::{complete_parse, JodinParser};

    use super::*;

    #[test]
    fn parse_identifier() {
        JodinParser::parse(Rule::identifier, "hello::what").unwrap();
        JodinParser::parse(Rule::identifier, "h1::what").unwrap();
        complete_parse(Rule::identifier, "h1::if").unwrap_err();
        JodinParser::parse(Rule::identifier, "3as").unwrap_err();
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            JodinParser::parse(Rule::string, "\"hello \\\"world\"")
                .unwrap()
                .as_str(),
            "\"hello \\\"world\"",
            "Couldn't parse escapable strings"
        );
        assert_eq!(
            JodinParser::parse(Rule::string, "(*\"hello \"world\"*)")
                .unwrap()
                .as_str(),
            "(*\"hello \"world\"*)",
            "Couldn't parse exact strings"
        );
    }

    #[test]
    fn parse_math_expression() {
        JodinParser::parse(Rule::identifier, "true").unwrap_err();
        JodinParser::parse(Rule::expression, "3").unwrap();
        JodinParser::parse(Rule::expression, "3+4").unwrap();
        JodinParser::parse(Rule::expression, "3+-4").unwrap();
        println!(
            "{:#?}",
            JodinParser::parse(Rule::expression, "true || false").unwrap()
        );
        assert_eq!(
            JodinParser::parse(Rule::expression, "-3+-4*5")
                .unwrap()
                .as_str(),
            "-3+-4*5"
        );
        assert_eq!(
            JodinParser::parse(Rule::expression, "variable.field->method()")
                .unwrap()
                .as_str(),
            "variable.field->method()",
            "Couldn't parse members"
        );
        assert_eq!(
            JodinParser::parse(
                Rule::expression,
                "variable.method(var1->field1, var2->field() + 2)*4"
            )
            .unwrap()
            .as_str(),
            "variable.method(var1->field1, var2->field() + 2)*4",
            "Couldn't parse members and args list"
        );
        complete_parse(Rule::expression, "3ul >> 2").unwrap();
        complete_parse(Rule::expression, "true || false").unwrap();
    }

    #[test]
    fn parse_statements() {
        complete_parse(
            Rule::selection_statement,
            "\
        if (true || false) {\
            \
        } else if (false) { } else;\
        ",
        )
        .unwrap();
    }

    #[test]
    fn parse_declaration() {
        let p = complete_parse(Rule::declaration, "unsigned int red;");
        match p.map_err(|e| e.into_err_and_bt().0) {
            Ok(o) => println!("{:#?}", o),
            Err(JodinErrorType::ParserError(e, None)) => {
                println!("{}", e);
                panic!()
            }
            _ => {
                panic!()
            }
        }
        let p = complete_parse(Rule::declaration, "unsigned int red(int n);");
        match p.map_err(|e| e.into_err_and_bt().0) {
            Ok(o) => println!("{:#?}", o),
            Err(JodinErrorType::ParserError(e, None)) => {
                println!("{}", e);
                panic!()
            }
            _ => {
                panic!()
            }
        }
    }

    #[test]
    fn parse_function_definition() {
        let string = "unsigned int factorial(unsigned int n) {\
        if (n == 0) return 0;\
        return factorial(n- 1) * n; }";
        let p = complete_parse(Rule::function_definition, string);
        match p.map_err(|e| e.into_err_and_bt().0) {
            Ok(o) => println!("{:#?}", o),
            Err(JodinErrorType::ParserError(e, None)) => {
                println!("{:#?}", e);
                println!("{}", e);
                panic!()
            }
            _ => {
                panic!()
            }
        }
    }

    #[test]
    fn parse_structure_definition() {
        let string = "struct basic { int* hello; };";
        complete_parse(Rule::struct_definition, string).unwrap();
        let string =
            "struct basic<T <- Object> { T* hello; }; struct basic<T <- Object> { T* hello; };";
        complete_parse(Rule::top_level_declarations, string).unwrap();
    }
}

#[cfg(all(not(feature = "pest_parser"), feature = "larlpop_parser"))]
pub mod jodin_grammar;

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
            println!("Parsed {:?} from {:?}", tok, &self.original[span.clone()]);
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
    #[regex(r"[0-9]+\.[0-9]*([Ee][+-]?[0-9]+)?(f|F|l|L)?")]
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
    #[token("=", |_| Option::<Operator>::None)]
    #[regex(r"[+\-*/&%^|]=", maybe_assign_operator, priority = 10)]
    Assign(Option<Operator>),
    #[regex(r"[a-zA-Z_]\w*")]
    #[regex(r"@[a-zA-Z_]\w*", |lex| &lex.source()[1..])]
    Identifier(&'input str),
    #[regex(r"//.*", logos::skip)]
    #[token("/*", comment)]
    Comment,
    #[error]
    Error,
}

pub enum ExpressionMember {
    Factor(JodinNode),
    Op(Operator),
}

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

fn maybe_assign_operator<'input>(
    lex: &mut Lexer<'input, Tok<'input>>,
) -> JodinResult<Option<Operator>> {
    let full_operator: &str = lex.slice();
    let regex = Regex::new(r"(?P<operator>.[^=]?)=").unwrap();
    if let Some(captures) = regex.captures(full_operator) {
        let op = captures
            .name("operator")
            .expect("operator group should exist");
        let operator = op.as_str();
        Operator::from_str(operator).map(|op| Some(op))
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
macro_rules! parse {
    ($parser:ty, $ex:expr) => {{
        let string: &str = $ex;
        let lexer = $crate::parsing::JodinLexer::new(string);
        let parser = <$parser>::new();
        parser.parse(string, lexer)
    }};
}

type ParseResult = JodinResult<JodinNode>;

#[cfg(all(not(feature = "pest_parser"), feature = "larlpop_parser"))]
mod tests {
    use super::jodin_grammar;
    use crate::core::identifier::Identifier;
    use crate::core::literal::Literal;
    use crate::core::operator::Operator;
    use crate::parsing::{JodinLexer, Tok};
    use std::iter::FromIterator;
    use std::str::FromStr;

    #[test]
    fn lex_identifiers() {
        let string = "+=";
        let mut lexer = JodinLexer::new(string);
        match lexer.next() {
            Some(Ok((_, Tok::Assign(Some(Operator::Plus)), _))) => {}
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
        let result = parse!(jodin_grammar::ExpressionParser, "1+(2-3)*4/5==8");
        println!("{:#?}", result.unwrap());
    }

    #[test]
    fn parse_statement() {
        parse!(jodin_grammar::StatementParser, "a[0] = 3;").unwrap();
        parse!(jodin_grammar::StatementParser, "a.hello[3].beep = 3;").unwrap();
    }
}
