use crate::core::error::{JodinError, JodinResult};
use pest::iterators::{Pair, Pairs};
use pest::{ParseResult, Parser, RuleType};

#[derive(Parser, Debug)]
#[grammar = "parsing/jodin_grammar.pest"]
pub struct JodinParser;

pub type JodinRule = Rule;

pub fn complete_parse(rule: Rule, input: &str) -> JodinResult<Pairs<Rule>> {
    let pairs: Pairs<_> = JodinParser::parse(rule, input)?;
    if pairs.as_str() == input {
        Ok(pairs)
    } else {
        Err(JodinError::IncompleteParse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::parser::JodinParser;
    use pest::iterators::{Pair, Pairs};
    use pest::Parser;

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
        match p {
            Ok(o) => println!("{:#?}", o),
            Err(JodinError::ParserError(e, None)) => {
                println!("{}", e);
                panic!()
            }
            _ => {
                panic!()
            }
        }
        let p = complete_parse(Rule::declaration, "unsigned int red(int n);");
        match p {
            Ok(o) => println!("{:#?}", o),
            Err(JodinError::ParserError(e, None)) => {
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
        match p {
            Ok(o) => println!("{:#?}", o),
            Err(JodinError::ParserError(e, None)) => {
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
