use std::collections::HashSet;

use parsertools::{results::ParseError, transformers::{alternating, disjunction, series}, Parser};

use crate::helpers::parsers::{string_parser, word_parser};

#[derive(Clone)]
pub struct ControlStrings {
    escape: String, 
    others: HashSet<String>
}
impl ControlStrings {
    pub fn new(escape: String, others: HashSet<String>) -> Self { Self{escape, others} }
    pub fn from_strs<'a, I: IntoIterator<Item = &'a str>>(escape: &str, others: I) -> Self { Self{escape: escape.to_string(), others: others.into_iter().map(|s| s.to_string()).collect()} }

    pub fn controls(&self) -> HashSet<String> { self.others.iter().chain(vec![self.escape_control()]).cloned().collect() }
    pub fn escape_control(&self) -> &String { &self.escape }
    pub fn non_escape_controls(&self) -> &HashSet<String> { &self.others }
}

pub fn control_parser<'a>(controls: &ControlStrings) -> Parser<'a,char,String>
    { escape_control_parser(controls).or(other_control_parser(controls)) }
pub fn escape_control_parser<'a>(controls: &ControlStrings) -> Parser<'a, char,String>
    { string_parser(controls.escape_control()).unwrap() }
pub fn other_control_parser<'a>(controls: &ControlStrings) -> Parser<'a,char,String>
    { disjunction(controls.non_escape_controls().iter().map(|s| string_parser(s).unwrap())) }

pub fn controlled_word_parser<'a>(controls: ControlStrings) -> Parser<'a,char,String> {
    let word_not_containing_control = word_not_containing_parser(controls.controls());
    let series_of_escaped_controls = series(escaped_control_parser(controls)).map(|vec| vec.concat());
    alternating(word_not_containing_control, series_of_escaped_controls).map(|strings| strings.concat())
}

fn escaped_control_parser<'a>(controls: ControlStrings) -> Parser<'a,char,String> {
    escape_control_parser(&controls)
        .then(escape_control_parser(&controls).or(other_control_parser(&controls)))
        .map(|(_,s)| s)
}

fn word_not_containing_parser<'a>(blacklist: HashSet<String>) -> Parser<'a, char, String> {
    word_parser().filter(
        move |word| !blacklist.iter().any(|should_not_contain| -> bool { word.contains(should_not_contain) }),
        ParseError::UnexpectedTokenProperUnknown
    )
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::test_helpers::parse_str;

    use super::*;

    const TEST_CONTROL_STRINGS: LazyLock<ControlStrings> = LazyLock::new(||
        ControlStrings::from_strs("\\", vec!["#",".."])
    );

    #[test]
    pub fn test_word_not_containing_parser_with_no_blacklist() {
        assert_eq!(parse_str(word_not_containing_parser(HashSet::new()), "vsnofgojgrn"),Ok("vsnofgojgrn".to_string()))
    }

    #[test]
    pub fn test_word_not_containing_parser_with_no_blacklisted_items() {
        assert_eq!(parse_str(word_not_containing_parser(TEST_CONTROL_STRINGS.non_escape_controls().clone()), "vsnofgojgrn"),Ok("vsnofgojgrn".to_string()))
    }

    #[test]
    pub fn test_word_not_containing_parser_with_blacklisted_items() {
        let parsed = parse_str(word_not_containing_parser(TEST_CONTROL_STRINGS.non_escape_controls().clone()), "vsnofg..ojgrn");
        assert!(parsed.is_err())
    }

    #[test]
    pub fn test_controlled_parser_with_no_controls() {
        assert_eq!(parse_str(controlled_word_parser(TEST_CONTROL_STRINGS.clone()), "Hello"),Ok("Hello".to_string()))
    }

    #[test]
    pub fn test_controlled_parser_with_single_char_control() {
        let parsed = parse_str(controlled_word_parser(TEST_CONTROL_STRINGS.clone()), "Hel#lo");
        assert!(parsed.is_err())
    }

    #[test]
    pub fn test_controlled_parser_with_multi_char_control() {
        let parsed = parse_str(controlled_word_parser(TEST_CONTROL_STRINGS.clone()), "Hel..lo");
        assert!(parsed.is_err())
    }

    #[test]
    pub fn test_controlled_parser_with_escaped_single_char_control() {
        assert_eq!(parse_str(controlled_word_parser(TEST_CONTROL_STRINGS.clone()), "Hel\\#lo"),Ok("Hel#lo".to_string()))
    }

    #[test]
    pub fn test_controlled_parser_with_escaped_multi_char_control() {
        assert_eq!(parse_str(controlled_word_parser(TEST_CONTROL_STRINGS.clone()), "Hel\\..lo"),Ok("Hel..lo".to_string()))
    }

    #[test]
    pub fn test_controlled_parser_with_single_escape() {
        let parsed = parse_str(controlled_word_parser(TEST_CONTROL_STRINGS.clone()), r#"Hel\lo"#);
        assert!(parsed.is_err())
    }

    #[test]
    pub fn test_controlled_parser_with_double_escape() {
        assert_eq!(parse_str(controlled_word_parser(TEST_CONTROL_STRINGS.clone()), r#"Hel\\lo"#),Ok(r#"Hel\lo"#.to_string()))
    }

    #[test]
    pub fn test_controlled_parser_with_triple_escape() {
        let parsed = parse_str(controlled_word_parser(TEST_CONTROL_STRINGS.clone()), r#"Hel\\\lo"#);
        assert!(parsed.is_err())
    }
}