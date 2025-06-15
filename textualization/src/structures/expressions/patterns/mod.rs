use hashable::HashableHashSet;
use parsertools::parsers::{transformers::conjoin, Parser};

use crate::structures::expressions::patterns::components::{pattern_component_parser, ExprPatternAssignment, ExprPatternComponent};

pub mod components;
pub mod parser;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct ExprPattern{
    components: Vec<ExprPatternComponent>
}
impl ExprPattern {
    fn new<I: IntoIterator<Item=ExprPatternComponent>>(components: I) -> Self { Self { components: components.into_iter().collect() }}
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct ExprPatternAssignments(HashableHashSet<ExprPatternAssignment>);
impl ExprPatternAssignments {
    fn new<I: IntoIterator<Item=ExprPatternAssignment>>(assignments: I) -> Self { Self(
        assignments.into_iter().filter(|assignment| assignment != &ExprPatternAssignment::Constant).collect()
    )}
}

fn expr_pattern_matcher<'a>(pattern: &'a ExprPattern) -> Parser<'a, char, ExprPatternAssignments> {
    let components = pattern.components.iter()
        .map(|component| pattern_component_parser(component));
    conjoin(components).map(|assignments| ExprPatternAssignments::new(assignments))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{structures::expressions::patterns::parser::{expr_pattern_parser}, test_helpers::{parse_all_str, parse_str}};

    use super::*;

    fn pre_test_matcher(pattern_str: &str, match_str: &str, assignments_vec: Vec<(Vec<(&str,&str)>,Vec<(&str,&str,Vec<&str>)>)>) -> (HashSet<ExprPatternAssignments>,HashSet<ExprPatternAssignments>) {
        let controls = parser::TEST_PATTERN_CONTROLS;
        let blacklist = parser::TEST_BLACKLIST;
        let parser = expr_pattern_parser(&controls, &blacklist);
        let pattern = parse_str(parser,pattern_str).unwrap();
        let assignments = parse_all_str(expr_pattern_matcher(&pattern), match_str);
        
        let assignments_check = assignments_vec.into_iter().map(|(var_vec,vars_vec)| -> ExprPatternAssignments {
            let var = var_vec.into_iter().map(|(var,val)| ExprPatternAssignment::new_var(var, val));
            let vars = vars_vec.into_iter().map(|(var1,var2,vals)| ExprPatternAssignment::new_vars(var1,var2,vals));
            ExprPatternAssignments::new(var.chain(vars))
        }).collect();
        (assignments, assignments_check)
    }
    
    #[test]
    fn test_match_with_const() {
        let (assignments, check) = pre_test_matcher(
            "r32u89",
            "r32u89",
            vec![
                (vec![],vec![]),
        ]); assert_eq!(assignments, check);
    }

    #[test]
    fn test_match_with_var() {
        let (assignments, check) = pre_test_matcher(
            "@x1@",
            "fgt43y4", 
            vec![
                (vec![("x1","fgt43y4")],vec![]),
        ]); assert_eq!(assignments, check);
    }

    #[test]
    fn test_match_with_vars() {
        let (assignments, check) = pre_test_matcher(
            "@x1..,..x2@", 
            "a,b,c",
            vec![
                (vec![],vec![("x1","x2",vec!["a","b","c"])]),
                (vec![],vec![("x1","x2",vec!["a,b","c"])]),
                (vec![],vec![("x1","x2",vec!["a","b,c"])]),
                (vec![],vec![("x1","x2",vec!["a,b,c"])]),
        ]); assert_eq!(assignments, check);
    }

    #[test]
    fn test_match_with_complex_string() {
        let (assignments, check) = pre_test_matcher(
            "(@G@,(f,@A.. & ..B@))",
            "(g_value,(f,a1 & a2 & a3))",
            vec![
                (vec![("G","g_value")], vec![("A","B",vec!["a1","a2","a3"])]),
                (vec![("G","g_value")], vec![("A","B",vec!["a1 & a2","a3"])]),
                (vec![("G","g_value")], vec![("A","B",vec!["a1","a2 & a3"])]),
                (vec![("G","g_value")], vec![("A","B",vec!["a1 & a2 & a3"])]),
        ]); assert_eq!(assignments, check);
    }
}
