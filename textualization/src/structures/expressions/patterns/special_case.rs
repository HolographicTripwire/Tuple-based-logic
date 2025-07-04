use parsertools::Parser;
use tbl_structures::propositions::Expression;

use crate::structures::expressions::{patterns::{expr_pattern_matcher, ExprPattern}, SpecialCase};

#[derive(Clone)]
pub struct ExprPatternPair {
    left: ExprPattern,
    right: ExprPattern,
}
impl ExprPatternPair {
    pub fn new(left: ExprPattern, right: ExprPattern) -> Self
        { Self { left, right } }

    fn left_to_right<'a>(&self) -> Parser<'a,char,String> {
        expr_pattern_translator(self.left.clone(), self.right.clone())
    }
    fn right_to_left<'a>(&self) -> Parser<'a,char,String> {
        expr_pattern_translator(self.right.clone(), self.left.clone())
    }
}
impl <'a> SpecialCase<'a> for ExprPatternPair {
    fn parser(&self, expr_parser: Parser<'a,char,Expression>) -> Parser<'a,char,Expression> {
        self.right_to_left().clone()
            .split_map(move |s| expr_parser.parse(s.chars()))
    }
}

fn expr_pattern_translator<'a>(before: ExprPattern, after: ExprPattern) -> Parser<'a,char,String> {
    expr_pattern_matcher(before)
        .map(move |assignments| after.assign(&assignments).unwrap().try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{collections::HashSet, sync::LazyLock};

    use parsertools::{results::ParseError, Parser};
    use tbl_structures::propositions::Expression;

    use crate::{helpers::styles::Style, structures::expressions::{patterns::{parser::{TEST_BLACKLIST, TEST_PATTERN_STYLE}, ExprPattern}, raw::tests::TEST_RAW_EXPRESSION_STYLE}, test_helpers::{parse_all_str, parse_str}};

    fn parse_pattern(s: &str) -> ExprPattern
        { parse_str(TEST_PATTERN_STYLE.parser(TEST_BLACKLIST.clone()), s).unwrap() }

    fn pre_expr_pattern_translator_test(before_pattern_str: &str, after_pattern_str: &str, before_str: &str, after_strs: Vec<&str>) -> (HashSet<String>, HashSet<String>) {
        let before_pattern = parse_pattern(before_pattern_str);
        let after_pattern = parse_pattern(after_pattern_str);
        let after = parse_all_str(expr_pattern_translator(before_pattern, after_pattern),before_str);
        let after_check = after_strs.into_iter().map(|s| s.to_string()).collect();
        (after, after_check)
    }

    #[test]
    fn test_expr_pattern_translator_with_vars() {
        let (after, after_check) = pre_expr_pattern_translator_test(
            "(&,@A..,..B@)", "(@A.. & ..B@)", "(&,A,B,C)",
            vec!["(A & B & C)","(A & B,C)","(A,B & C)","(A,B,C)"]
        ); assert_eq!(after, after_check);
    }

    const RAW_EXPRESSION_PARSER: LazyLock<Parser<char,Expression>> = LazyLock::new(||
        TEST_RAW_EXPRESSION_STYLE.parser(())
    );
    fn parse_pattern_pair<'a>(l: &str, r: &str) -> ExprPatternPair
        { ExprPatternPair::new(parse_pattern(l),parse_pattern(r)) }

    fn pre_test_special_case(before_pattern_str: &str, after_pattern_str: &str, before_str: &str, after_expression: &str) -> (Result<Expression,ParseError<char>>,Expression) {
        let pattern_pair = parse_pattern_pair(before_pattern_str,after_pattern_str);
        let after = parse_str(pattern_pair.parser(RAW_EXPRESSION_PARSER.clone()),before_str);
        let after_check = parse_str(RAW_EXPRESSION_PARSER.clone(),after_expression).unwrap();
        (after, after_check)
    }

    #[test]
    fn test_special_case_with_no_change() {
        let (after,after_check) = pre_test_special_case("@A@", "@A@", "#1", "#1");
        assert_eq!(after,Ok(after_check))
    }

    #[test]
    fn test_special_case_with_var() {
        let (after,after_check) = pre_test_special_case("@A@", "@A@+", "#1+", "#1");
        assert_eq!(after,Ok(after_check))
    }

    #[test]
    fn test_special_case_with_vars() {
        let (after,after_check) = pre_test_special_case("(@A..,..B@)","(@A.. & ..B@)","(#1 & #2 & #3)","(#1,#2,#3)");
        assert_eq!(after,Ok(after_check))
    }
}
