
pub mod helpers;
pub mod atom;
pub mod expressions;


#[cfg(test)]
mod test_helpers {
    use std::{fmt::Debug, hash::Hash};

    use parsertools::{Parser, ParsingError};

    pub (crate) fn parse_str<T: PartialEq + Debug + Hash + Clone>(parser: Parser<char,T>, string: &str) -> Result<T,ParsingError<char>> {
        let chars = string.chars().collect::<Vec<_>>();
        parser.parse_all(&chars)
    }
}
