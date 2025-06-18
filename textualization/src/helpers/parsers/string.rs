use parsertools::parsers::{tokens::{pred, tok}, transformers::{conjoin, series}, Parser};

pub fn string_parser<'a>(input: &str) -> Result<Parser<'a, char, String>,()> {
    if input.len() == 0 { return Err(()) }
    let char_parsers = input.chars().map(|char| tok(char));
    Ok(conjoin(char_parsers).map(|vec| vec.into_iter().collect::<String>()))
}

pub fn word_parser<'a>() -> Parser<'a, char, String> {
    series(single_letter_parser()).map(|strings| strings.concat())
}

pub fn single_letter_parser<'a>() -> Parser<'a, char, String> {
    pred(|token: &char| { Some(token.to_string()) })
}


#[cfg(test)]
mod tests {
    use crate::test_helpers::parse_str;

    use super::*;
    
    #[test]
    fn test_string_parser_with_single_char_string() {
        assert_eq!(parse_str(string_parser("H").unwrap(), "H"),Ok("H".to_string()))
    }
    #[test]
    fn test_string_parser_with_multi_char_string() {
        assert_eq!(parse_str(string_parser("Hello").unwrap(), "Hello"),Ok("Hello".to_string()))
    }
    #[test]
    fn test_string_parser_with_nonmatching_string() {
        assert!(parse_str(string_parser("Hello").unwrap(), "Hello there").is_err())
    }
}
