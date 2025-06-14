use parsertools::parsers::{tokens::pred, transformers::series, Parser};

pub fn num_parser<'a>() -> Parser<'a, char, usize> {
    series(digit_parser_inner())
        .map(|vec| vec.join("").parse::<usize>().unwrap()) 
}

pub fn digit_parser<'a>() -> Parser<'a, char, usize> {
    digit_parser_inner().map(|s| s.parse::<usize>().unwrap())
}
pub fn digit_parser_inner<'a>() -> Parser<'a, char, String> {
    pred(|token: &char| 
        { if vec!['0','1','2','3','4','5','6','7','8','9'].contains(token) { Some(token.to_string()) } else { None } }
    )
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::parse_str;

    use super::*;
    
    #[test]
    fn test_digit_parser_with_digit() {
        assert_eq!(parse_str(digit_parser(), "0"),Ok(0))
    }
    #[test]
    fn test_digit_parser_with_nonnumber() {
        assert!(parse_str(digit_parser(), "a").is_err());
    }
    #[test]
    fn test_digit_parser_with_nondigit() {
        assert!(parse_str(digit_parser(), "01").is_err());
    }

    #[test]
    fn test_num_parser_with_digit() {
        assert_eq!(parse_str(num_parser(), "0"),Ok(0))
    }
    #[test]
    fn test_num_parser_with_num() {
        assert_eq!(parse_str(num_parser(), "012003"),Ok(12003))
    }
    #[test]
    fn test_digit_parser_with_nonnum() {
        assert!(parse_str(num_parser(), "10a").is_err());
    }
}
