use std::collections::HashSet;

use parsertools::parsers::{transformers::alternating, Parser};

use crate::{helpers::{parsers::{controlled::{controlled_word_parser, ControlStrings}, string_parser}, styles::Style}, structures::expressions::patterns::{components::ExprPatternComponent, ExprPattern}};

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct ExprPatternStyle {
    var_indic: String,
    var_enum: String,
}
impl ExprPatternStyle {
    pub fn new(var_indic: String, var_enum: String) -> Self { Self { var_indic, var_enum } }
    pub fn from_strs(var_indic: &str, var_enum: &str) -> Self
        { Self { var_indic: var_indic.to_string(), var_enum: var_enum.to_string() } }
    
    pub fn var_indic(&self) -> &String { &self.var_indic }
    pub fn var_enum(&self) -> &String { &self.var_enum }
    
    pub fn controls(&self) -> HashSet<&str> { HashSet::from_iter(
        [self.var_indic(),
        self.var_enum()]
        .map(|s| s.as_str())
    )}
}
impl Style<ExprPattern> for ExprPatternStyle {
    fn stringify(&self, pattern: &ExprPattern) -> String {
        pattern.components.iter()
            .map(|component| self.stringify(component))
            .collect::<Vec<String>>().join("")
    }
}

fn var_indic_parser<'a>(style: &ExprPatternStyle) -> Parser<'a,char,()> { string_parser(style.var_indic()).unwrap().map(|_| ()) }
fn var_enum_parser<'a>(style: &ExprPatternStyle) -> Parser<'a,char,()> { string_parser(style.var_enum()).unwrap().map(|_| ()) }

pub fn expr_pattern_parser<'a>(style: &ExprPatternStyle, blacklist: &'a ControlStrings) -> Parser<'a, char, ExprPattern> {
    alternating(const_parser(style, blacklist),var_or_vars_parser(style, blacklist)).map(|components| ExprPattern::new(components))
}

fn var_or_vars_parser<'a>(style: &ExprPatternStyle, blacklist: &'a ControlStrings) -> Parser<'a, char, ExprPatternComponent>
    { var_parser(style, blacklist).or(vars_parser(style, blacklist)) }

fn const_parser<'a>(_: &ExprPatternStyle, blacklist: &'a ControlStrings) -> Parser<'a, char, ExprPatternComponent>
    { controlled_word_parser(blacklist).map(|s| ExprPatternComponent::Constant(s)) }
fn var_parser<'a>(style: &ExprPatternStyle, blacklist: &'a ControlStrings) -> Parser<'a, char, ExprPatternComponent> {
    let var_indic_parser = var_indic_parser(style);
    var_indic_parser.clone()
        .then(controlled_word_parser(blacklist))
        .then(var_indic_parser)
        .map(|((_,s),_)| ExprPatternComponent::Variable(s))
}
fn vars_parser<'a>(style: &ExprPatternStyle, blacklist: &'a ControlStrings) -> Parser<'a, char, ExprPatternComponent> {
    let var_indic_parser = var_indic_parser(style);
    let var_enum_parser = var_enum_parser(style);
    let word_parser = controlled_word_parser(blacklist);
    var_indic_parser.clone()
        .then(word_parser.clone()).then(var_enum_parser.clone())
        .then(word_parser.clone()).then(var_enum_parser)
        .then(word_parser.clone()).then(var_indic_parser)
        .map(|((((((_,v1),_),sep),_),v2),_)| ExprPatternComponent::Variables((v1,v2),sep))
}

#[cfg(test)]
pub(crate) use self::tests::TEST_PATTERN_STYLE;
#[cfg(test)]
pub(crate) use self::tests::TEST_BLACKLIST;

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;

    use crate::{structures::expressions::patterns::parser::ExprPatternStyle, test_helpers::parse_str};

    pub const TEST_PATTERN_STYLE: LazyLock<ExprPatternStyle> = LazyLock::new(|| {
        ExprPatternStyle::from_strs("@", "..")
    });
    pub const TEST_BLACKLIST: LazyLock<ControlStrings> = LazyLock::new(|| {
        ControlStrings::from_strs("\\", TEST_PATTERN_STYLE.controls())
    });

    #[test]
    fn test_const_parser() {
        assert_eq!(
            parse_str(
                const_parser(&TEST_PATTERN_STYLE, &TEST_BLACKLIST), 
                "Hello"
            ), Ok(ExprPatternComponent::new_const("Hello"))
        )
    }
    
    #[test]
    fn test_var_parser() {
        assert_eq!(
            parse_str(
                var_parser(&TEST_PATTERN_STYLE, &TEST_BLACKLIST), 
                "@adiw awdio@"
            ), Ok(ExprPatternComponent::new_var("adiw awdio"))
        )
    }

    #[test]
    fn test_vars_parser() {
        assert_eq!(
            parse_str(
                vars_parser(&TEST_PATTERN_STYLE, &TEST_BLACKLIST),
                "@a.. and ..b@"
            ), Ok(ExprPatternComponent::new_vars("a"," and ","b"))
        )
    }

    #[test]
    fn test_pattern_parser_with_const() {
        assert_eq!(
            parse_str(
                expr_pattern_parser(&TEST_PATTERN_STYLE, &TEST_BLACKLIST), 
                "Hello"
            ), Ok(ExprPattern::new([ExprPatternComponent::new_const("Hello")]))
        )
    }
    
    #[test]
    fn test_pattern_parser_with_var() {
        assert_eq!(
            parse_str(
                expr_pattern_parser(&TEST_PATTERN_STYLE, &TEST_BLACKLIST), 
                "@adiw awdio@"
            ), Ok(ExprPattern::new([ExprPatternComponent::new_var("adiw awdio")]))
        )
    }

    #[test]
    fn test_pattern_parser_with_vars() {
        assert_eq!(
            parse_str(
                expr_pattern_parser(&TEST_PATTERN_STYLE, &TEST_BLACKLIST),
                "@a.. and ..b@"
            ), Ok(ExprPattern::new([ExprPatternComponent::new_vars("a"," and ","b")]))
        )
    }

    #[test]
    fn test_pattern_parser_with_escapes() {
        assert_eq!(
            parse_str(
                expr_pattern_parser(&TEST_PATTERN_STYLE, &TEST_BLACKLIST),
                r#"@a.. \@ ..b@"#
            ), Ok(ExprPattern::new([ExprPatternComponent::new_vars("a"," @ ","b")]))
        )
    }

    #[test]
    fn test_pattern_parser_with_complex_string() {
        assert_eq!(
            parse_str(
                expr_pattern_parser(&TEST_PATTERN_STYLE, &TEST_BLACKLIST),
                r#"x(@a.. \@ ..b@,@x@)"#
            ), Ok(ExprPattern::new([
                ExprPatternComponent::new_const("x("),
                ExprPatternComponent::new_vars("a"," @ ","b"),
                ExprPatternComponent::new_const(","),
                ExprPatternComponent::new_var("x"),
                ExprPatternComponent::new_const(")")
            ]))
        )
    }
}
