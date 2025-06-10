use parsertools::{lazy, pred, tok, Parser};

pub fn string_parser<'a>(input: &str) -> Result<Parser<'a, char, ()>,()> {
    let chars: Vec<char> = input.chars().collect();
    string_parser_inner(&chars)
}
fn string_parser_inner<'a>(value: &[char]) -> Result<Parser<'a, char, ()>,()> {
    if value.is_empty() {
        Err(())
    } else { Ok(
        if value.len() == 1 {
            tok(value[0])
        } else {
            tok(value[0])
                .then(string_parser_inner(&value[1..])?)
                .map(|_| ())
        }
    )}
}

fn word_parser<'a>() -> Parser<'a, char, String> {
    single_letter_parser()
        .then(lazy(word_parser))
        .map(|(first_letter, rest)| first_letter + &rest)
        .or(single_letter_parser())
}

fn single_letter_parser<'a>() -> Parser<'a, char, String> {
    pred(|token: &char| { Some(token.to_string()) })
}


#[cfg(test)]
mod tests {
    use crate::parse_str;

    use super::*;
    
    #[test]
    fn test_digit_parser_with_single_char_string() {
        assert_eq!(parse_str(string_parser("H").unwrap(), "H"),Ok(()))
    }
    #[test]
    fn test_digit_parser_with_multi_char_string() {
        assert_eq!(parse_str(string_parser("Hello").unwrap(), "Hello"),Ok(()))
    }
    #[test]
    fn test_digit_parser_with_nonmatching_string() {
        assert!(parse_str(string_parser("Hello").unwrap(), "Hello there").is_err())
    }
}
