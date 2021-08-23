//! Different modules that make up the parsing mechanisms for jodin. The parsing system is based
//! off of the pest crate.

#![allow(missing_docs)]

#[cfg(feature = "pest_parser")]
use pest::iterators::Pairs;
#[cfg(feature = "pest_parser")]
use pest::Parser;
#[cfg(feature = "pest_parser")]
use pest_derive::Parser;

use crate::core::error::{JodinErrorType, JodinResult, JodinError};
use std::str::CharIndices;
use crate::core::literal::Literal;
use crate::core::operator::Operator;
use logos::{Logos, Lexer, SpannedIter, Skip};

/// The JodinParser. Used the pest engine to perform parsing.
#[cfg(all(feature="pest_parser", not(feature = "larlpop_parser")))]
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

#[cfg(all(not(feature="pest_parser"), feature = "larlpop_parser"))]
pub mod jodin_grammar;

pub type Spanned<Tok, Loc> = Result<(Loc, Tok, Loc), JodinError>;

/// The jodin lexer
pub struct JodinLexer<'input> {
    lexer: SpannedIter<'input, Tok<'input>>
}

impl<'input> JodinLexer<'input> {
    /// Create a new lexer from an input
    pub fn new(input: &'input str) -> Self {
        JodinLexer {
            lexer: Tok::lexer(input).spanned()
        }
    }
}

/*

D			[0-9]
L			[a-zA-Z_]
H			[a-fA-F0-9]
E			[Ee][+-]?{D}+
FS			(f|F|l|L)
IS			(u|U|l|L)*

 */

#[derive(Debug, Copy, Clone, Logos)]
pub enum Tok<'input> {
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
    Case,
    Char,
    Const,
    Continue,
    Default,
    Double,
    Do,
    Else,
    Float,
    For,
    If,
    Int,
    Long,
    Return,
    Short,
    Static,
    Typedef,
    Union,
    Unsigned,
    Struct,
    Void,
    While,
    Class,
    Public,
    Private,
    New,
    Super,
    Virtual,
    Sizeof,
    Boolean,
    In,
    Implement,
    Internal,
    Using,
    Typeof,
    True,
    False,
    Abstract,
    Is,
    Trait,
    Enum,
    Switch,
    As,
    Varargs,
    #[token("::")]
    Namespaced,
    SpecialKeyword(&'input str),
    Operator(Operator),
    Assign(Option<Operator>),
    #[regex(r"[a-zA-Z_]\w+")]
    Identifier(&'input str),
    #[error]
    Error,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r"//.*", logos::skip)]
    #[token("/*", comment)]
    Comment,
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
    let ret = &lex.source()[..(count+3)];
    ret
}



impl<'input> Iterator for JodinLexer<'input> {
    type Item = Spanned<Tok<'input>, usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((tok, span)) = self.lexer.next() {
            match tok {
                Tok::Error => return Some(Err(JodinErrorType::LexerError.into())),
                tok => Some(Ok((span.start, tok, span.end)))
            }
        } else {
            None
        }
    }
}

#[macro_use]
macro_rules! parse {
    ($parser:ty, $ex:expr) => {
        {
            let string: &str = $ex;
            let lexer = $crate::parsing::JodinLexer::new(string);
            let parser = <$parser>::new();
            parser.parse(string, lexer)
        }
    };
}

#[cfg(all(not(feature="pest_parser"), feature = "larlpop_parser"))]
mod tests {
    use super::jodin_grammar;
    use crate::core::identifier::Identifier;
    use std::iter::FromIterator;
    use crate::core::literal::Literal;
    use std::str::FromStr;
    use crate::parsing::JodinLexer;

    #[test]
    fn parse_identifiers() {

        let string = "std::mod::hello";
        let lexer = JodinLexer::new(string);
        let identifier_parser = jodin_grammar::IdentifierParser::new();
        assert_eq!(identifier_parser.parse(string, lexer).unwrap(), Identifier::from_iter(["std", "mod", "hello"]));
    }

    #[test]
    fn parse_constant() {
        assert_eq!(parse!(jodin_grammar::LiteralParser, "10").unwrap(), Literal::Int(10));
        // assert!(parse!(jodin_grammar::LiteralParser, "10.1").is_ok());
        // assert!(parse!(jodin_grammar::LiteralParser, ".1F").is_ok());
        assert!(parse!(jodin_grammar::LiteralParser, "0xAFD").is_ok());
    }

    #[test]
    fn parse_string() {
        assert_eq!(parse!(jodin_grammar::StringParser, r#""hello, world!""#).unwrap(), Literal::String("hello, world!".to_string()));
        assert_eq!(parse!(jodin_grammar::StringParser, r#"(*"hello, world!"*)"#).unwrap(), Literal::String("hello, world!".to_string()));
    }

}
