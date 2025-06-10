use parsertools::{lazy, pred, Parser};

pub fn num_parser<'a>() -> Parser<'a, char, usize> {
    digit_parser().then(lazy(num_parser))
        .map(|(first_digit, rest)| first_digit * 10 + rest)
        .or(digit_parser())
}

pub fn digit_parser<'a>() -> Parser<'a, char, usize> {
    pred(|token: &char| {
        vec!['0','1','2','3','4','5','6','7','8','9'].iter().position(|digit| token == digit) 
    })
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
        assert_eq!(parse_str(num_parser(), "012"),Ok(12))
    }
    #[test]
    fn test_digit_parser_with_nonnum() {
        assert!(parse_str(num_parser(), "10a").is_err());
    }
}
