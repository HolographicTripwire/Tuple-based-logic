
pub mod helpers;
pub mod structures;
pub mod generation;

#[cfg(test)]
mod test_helpers {
    use std::collections::HashSet;

    use parsertools::parsers::{AstBounds, results::ParseError, Parser};

    pub (crate) fn parse_str<T: AstBounds>(parser: Parser<char,T>, string: &str) -> Result<T,ParseError<char>>
        { parser.parse(string.chars()) }
    pub (crate) fn parse_all_str<T: AstBounds>(parser: Parser<char,T>, string: &str) -> HashSet<T>
        { parser.parse_all(string.chars()) }
}
