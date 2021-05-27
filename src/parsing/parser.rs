use pest::Parser;

#[derive(Parser, Debug)]
#[grammar = "parsing/jodin_grammar.pest"]
pub struct JodinParser;

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
        JodinParser::parse(Rule::identifier, "3as").unwrap_err();
    }
}
