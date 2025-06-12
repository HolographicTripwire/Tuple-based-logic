
pub mod helpers;
pub mod structures;
pub mod generation;

#[cfg(test)]
mod test_helpers {
    use std::collections::HashSet;

    use parsertools::{AstBounds, ParseError, Parser};

    pub (crate) fn parse_str<T: AstBounds>(parser: Parser<char,T>, string: &str) -> Result<T,ParseError<char>> {
        let chars = string.chars().collect::<Vec<_>>();
        parser.parse(&chars)
    }
    pub (crate) fn parse_all_str<T: AstBounds>(parser: Parser<char,T>, string: &str) -> HashSet<T> {
        let chars = string.chars().collect::<Vec<_>>();
        parser.parse_all(&chars)
    }
}
