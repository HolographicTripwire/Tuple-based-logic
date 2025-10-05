use std::collections::HashSet;

use parsertools::{transformers::alternating, Parser};

use crate::{helpers::{parsers::controlled::ControlStrings, styles::Style}, structures::expressions::patterns::{components::ExprPatternComponent, ExprPattern}};

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
    type ParseParams = ControlStrings;

    fn stringify(&self, pattern: &ExprPattern) -> String {
        pattern.components.iter()
            .map(|component| self.stringify(component))
            .collect::<Vec<String>>().join("")
    }
    
    fn parser<'a>(&self, blacklist: Self::ParseParams) -> Parser<'a,char,ExprPattern> {
        alternating(self.const_parser(blacklist.clone()),var_or_vars_parser(self, blacklist))
            .map(|components| ExprPattern::new(components))
    }
}

fn var_or_vars_parser<'a>(style: &ExprPatternStyle, blacklist: ControlStrings) -> Parser<'a, char, ExprPatternComponent>
    { style.var_parser(blacklist.clone()).or(style.vars_parser(blacklist)) }

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
                TEST_PATTERN_STYLE.const_parser(TEST_BLACKLIST.clone()), 
                "Hello"
            ), Ok(ExprPatternComponent::new_const("Hello"))
        )
    }
    
    #[test]
    fn test_var_parser() {
        assert_eq!(
            parse_str(
                TEST_PATTERN_STYLE.var_parser(TEST_BLACKLIST.clone()), 
                "@adiw awdio@"
            ), Ok(ExprPatternComponent::new_var("adiw awdio"))
        )
    }

    #[test]
    fn test_vars_parser() {
        assert_eq!(
            parse_str(
                TEST_PATTERN_STYLE.vars_parser(TEST_BLACKLIST.clone()),
                "@a.. and ..b@"
            ), Ok(ExprPatternComponent::new_vars("a"," and ","b"))
        )
    }

    #[test]
    fn test_pattern_parser_with_const() {
        assert_eq!(
            parse_str(
                TEST_PATTERN_STYLE.parser(TEST_BLACKLIST.clone()), 
                "Hello"
            ), Ok(ExprPattern::new([ExprPatternComponent::new_const("Hello")]))
        )
    }
    
    #[test]
    fn test_pattern_parser_with_var() {
        assert_eq!(
            parse_str(
                TEST_PATTERN_STYLE.parser(TEST_BLACKLIST.clone()), 
                "@adiw awdio@"
            ), Ok(ExprPattern::new([ExprPatternComponent::new_var("adiw awdio")]))
        )
    }

    #[test]
    fn test_pattern_parser_with_vars() {
        assert_eq!(
            parse_str(
                TEST_PATTERN_STYLE.parser(TEST_BLACKLIST.clone()),
                "@a.. and ..b@"
            ), Ok(ExprPattern::new([ExprPatternComponent::new_vars("a"," and ","b")]))
        )
    }

    #[test]
    fn test_pattern_parser_with_escapes() {
        assert_eq!(
            parse_str(
                TEST_PATTERN_STYLE.parser(TEST_BLACKLIST.clone()),
                r#"@a.. \@ ..b@"#
            ), Ok(ExprPattern::new([ExprPatternComponent::new_vars("a"," @ ","b")]))
        )
    }

    #[test]
    fn test_pattern_parser_with_complex_string() {
        assert_eq!(
            parse_str(
                TEST_PATTERN_STYLE.parser(TEST_BLACKLIST.clone()),
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
