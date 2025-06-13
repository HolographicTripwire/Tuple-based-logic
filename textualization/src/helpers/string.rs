use parsertools::{lazy, pred, tok, Parser};

use crate::helpers::vec_concat_parser_transformer;

pub fn string_parser<'a>(input: &str) -> Result<Parser<'a, char, String>,()> {
    if input.len() == 0 { return Err(()) }
    let chars: Vec<char> = input.chars().collect();
    let char_parsers = chars.into_iter().map(|char| tok(char).map(move |_| vec![char]));
    Ok(vec_concat_parser_transformer(char_parsers).map(|vec| vec.into_iter().collect::<String>()))
}

pub fn word_parser<'a>() -> Parser<'a, char, String> {
    single_letter_parser()
        .then(lazy(word_parser))
        .map(|(first_letter, rest)| first_letter + &rest)
        .or(single_letter_parser())
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
