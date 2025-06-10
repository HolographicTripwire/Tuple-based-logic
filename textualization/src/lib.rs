use std::{fmt::Debug, hash::Hash};

use parsertools::{Parser, ParsingError};

pub mod helpers;

pub (self) fn parse_str<T: PartialEq + Debug + Hash + Clone>(parser: Parser<char,T>, string: &str) -> Result<T,ParsingError<char>> {
    let chars = string.chars().collect::<Vec<_>>();
    parser.parse_all(&chars)
}
