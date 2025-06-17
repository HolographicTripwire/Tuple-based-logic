
use parsertools::parsers::{transformers::alternating, Parser};

use crate::{helpers::{controlled::{controlled_word_parser, ControlStrings}, string_parser}, structures::expressions::patterns::{components::ExprPatternComponent, ExprPattern}};

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct ExprPatternControls {
    var_indic: String,
    var_enum: String,
}
impl ExprPatternControls {
    pub fn new(var_indic: String, var_enum: String) -> Self { Self { var_indic, var_enum } }
    pub fn from_strs(var_indic: &str, var_enum: &str) -> Self
        { Self { var_indic: var_indic.to_string(), var_enum: var_enum.to_string() } }
    
    pub fn var_indic(&self) -> &String { &self.var_indic }
    pub fn var_enum(&self) -> &String { &self.var_enum }
    pub fn controls(&self) -> impl Iterator<Item=&str> { [self.var_indic.as_str(), self.var_enum.as_str()].into_iter() }
}

fn var_indic_parser<'a>(controls: &ExprPatternControls) -> Parser<'a,char,()> { string_parser(controls.var_indic()).unwrap().map(|_| ()) }
fn var_enum_parser<'a>(controls: &ExprPatternControls) -> Parser<'a,char,()> { string_parser(controls.var_enum()).unwrap().map(|_| ()) }

pub fn expr_pattern_parser<'a>(controls: &ExprPatternControls, blacklist: &'a ControlStrings) -> Parser<'a, char, ExprPattern> {
    alternating(const_parser(controls, blacklist),var_or_vars_parser(controls, blacklist)).map(|components| ExprPattern::new(components))
}

fn var_or_vars_parser<'a>(controls: &ExprPatternControls, blacklist: &'a ControlStrings) -> Parser<'a, char, ExprPatternComponent>
    { var_parser(controls, blacklist).or(vars_parser(controls, blacklist)) }

fn const_parser<'a>(_: &ExprPatternControls, blacklist: &'a ControlStrings) -> Parser<'a, char, ExprPatternComponent>
    { controlled_word_parser(blacklist).map(|s| ExprPatternComponent::Constant(s)) }
fn var_parser<'a>(controls: &ExprPatternControls, blacklist: &'a ControlStrings) -> Parser<'a, char, ExprPatternComponent> {
    let var_indic_parser = var_indic_parser(controls);
    var_indic_parser.clone()
        .then(controlled_word_parser(blacklist))
        .then(var_indic_parser)
        .map(|((_,s),_)| ExprPatternComponent::Variable(s))
}
fn vars_parser<'a>(controls: &ExprPatternControls, blacklist: &'a ControlStrings) -> Parser<'a, char, ExprPatternComponent> {
    let var_indic_parser = var_indic_parser(controls);
    let var_enum_parser = var_enum_parser(controls);
    let word_parser = controlled_word_parser(blacklist);
    var_indic_parser.clone()
        .then(word_parser.clone()).then(var_enum_parser.clone())
        .then(word_parser.clone()).then(var_enum_parser)
        .then(word_parser.clone()).then(var_indic_parser)
        .map(|((((((_,v1),_),sep),_),v2),_)| ExprPatternComponent::Variables((v1,v2),sep))
}

#[cfg(test)]
pub(crate) use self::tests::TEST_PATTERN_CONTROLS;
#[cfg(test)]
pub(crate) use self::tests::TEST_BLACKLIST;

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;

    use crate::{structures::expressions::patterns::parser::ExprPatternControls, test_helpers::parse_str};

    pub const TEST_PATTERN_CONTROLS: LazyLock<ExprPatternControls> = LazyLock::new(|| {
        ExprPatternControls::from_strs("@", "..")
    });
    pub const TEST_BLACKLIST: LazyLock<ControlStrings> = LazyLock::new(|| {
        ControlStrings::from_strs("\\", TEST_PATTERN_CONTROLS.controls())
    });

    #[test]
    fn test_const_parser() {
        assert_eq!(
            parse_str(
                const_parser(&TEST_PATTERN_CONTROLS, &TEST_BLACKLIST), 
                "Hello"
            ), Ok(ExprPatternComponent::new_const("Hello"))
        )
    }
    
    #[test]
    fn test_var_parser() {
        assert_eq!(
            parse_str(
                var_parser(&TEST_PATTERN_CONTROLS, &TEST_BLACKLIST), 
                "@adiw awdio@"
            ), Ok(ExprPatternComponent::new_var("adiw awdio"))
        )
    }

    #[test]
    fn test_vars_parser() {
        assert_eq!(
            parse_str(
                vars_parser(&TEST_PATTERN_CONTROLS, &TEST_BLACKLIST),
                "@a.. and ..b@"
            ), Ok(ExprPatternComponent::new_vars("a"," and ","b"))
        )
    }

    #[test]
    fn test_pattern_parser_with_const() {
        assert_eq!(
            parse_str(
                expr_pattern_parser(&TEST_PATTERN_CONTROLS, &TEST_BLACKLIST), 
                "Hello"
            ), Ok(ExprPattern::new([ExprPatternComponent::new_const("Hello")]))
        )
    }
    
    #[test]
    fn test_pattern_parser_with_var() {
        assert_eq!(
            parse_str(
                expr_pattern_parser(&TEST_PATTERN_CONTROLS, &TEST_BLACKLIST), 
                "@adiw awdio@"
            ), Ok(ExprPattern::new([ExprPatternComponent::new_var("adiw awdio")]))
        )
    }

    #[test]
    fn test_pattern_parser_with_vars() {
        assert_eq!(
            parse_str(
                expr_pattern_parser(&TEST_PATTERN_CONTROLS, &TEST_BLACKLIST),
                "@a.. and ..b@"
            ), Ok(ExprPattern::new([ExprPatternComponent::new_vars("a"," and ","b")]))
        )
    }

    #[test]
    fn test_pattern_parser_with_escapes() {
        assert_eq!(
            parse_str(
                expr_pattern_parser(&TEST_PATTERN_CONTROLS, &TEST_BLACKLIST),
                r#"@a.. \@ ..b@"#
            ), Ok(ExprPattern::new([ExprPatternComponent::new_vars("a"," @ ","b")]))
        )
    }

    #[test]
    fn test_pattern_parser_with_complex_string() {
        assert_eq!(
            parse_str(
                expr_pattern_parser(&TEST_PATTERN_CONTROLS, &TEST_BLACKLIST),
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
