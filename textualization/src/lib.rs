
pub mod helpers;
pub mod structures;
pub mod generation;

#[cfg(test)]
mod test_helpers {
    use std::{fmt::Debug, hash::Hash};

    use parsertools::{Parser, ParseError};

    pub (crate) fn parse_str<T: PartialEq + Debug + Hash + Clone>(parser: Parser<char,T>, string: &str) -> Result<T,ParseError<char>> {
        let chars = string.chars().collect::<Vec<_>>();
        parser.parse(&chars)
    }
    pub (crate) fn parse_all_str<T: PartialEq + Debug + Hash + Clone>(parser: Parser<char,T>, string: &str) -> Result<T,ParseError<char>> {
        let chars = string.chars().collect::<Vec<_>>();
        parser.parse_all(&chars)
    }
}
