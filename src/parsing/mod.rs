//! Different modules that make up the parsing mechanisms for jodin. The parsing system is based
//! off of the pest crate.

#![allow(missing_docs)]

use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;

use crate::core::error::{JodinErrorType, JodinResult};

/// The JodinParser. Used the pest engine to perform parsing.
#[derive(Parser, Debug)]
#[grammar = "parsing/jodin_grammar.pest"]
pub struct JodinParser;

/// Type alias to allow for easier refactoring
pub type JodinRule = Rule;

/// Attempts to parse an input string using a JodinRule.
///
/// Returns an error if it fails.
pub fn complete_parse(rule: Rule, input: &str) -> JodinResult<Pairs<Rule>> {
    let result: Result<_, pest::error::Error<_>> = JodinParser::parse(rule, input);
    result.map_err(|err| JodinErrorType::ParserError(err, None).into())
    //Ok(result)
}

#[cfg(test)]
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
