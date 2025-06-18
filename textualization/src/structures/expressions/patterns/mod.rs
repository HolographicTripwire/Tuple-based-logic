use hashable::HashableHashSet;
use parsertools::parsers::{transformers::conjoin, Parser};

use crate::structures::expressions::patterns::components::{pattern_component_parser, ExprPatternAssignment, ExprPatternComponent};

pub mod components;
pub mod parser;
pub mod special_case;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct ExprPattern{
    components: Vec<ExprPatternComponent>
}
impl ExprPattern {
    pub fn new<I: IntoIterator<Item=ExprPatternComponent>>(components: I) -> Self {
        let mut result = Vec::new();
        let mut consts_joined = "".to_string();
        for component in components.into_iter() { match component {
            ExprPatternComponent::Constant(s) => consts_joined += &s, _ => {
                if consts_joined.len() > 0 { 
                    result.push(ExprPatternComponent::Constant(consts_joined));
                    consts_joined = "".to_string();
                } result.push(component);
        }}} if consts_joined.len() > 0 { result.push(ExprPatternComponent::Constant(consts_joined)); }
        Self { components: result }
    }
    pub fn assign(&self, assignments: &ExprPatternAssignments) -> Result<ExprPattern,()> {
        let mut modified = self.components.clone();
        for assignment in assignments.0.into_iter() {
            if assignment != &ExprPatternAssignment::Constant {
                let modify = modified.iter().map(|component| component.assign(&assignment.clone())).collect();
                if modify == modified { return Err(()) }
                modified = modify
            }
        } Ok(Self::new(modified))
    }
}
impl TryInto<String> for ExprPattern {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        let [const_component] = self.components.as_slice() else { return Err(()) };
        if let ExprPatternComponent::Constant(s) = const_component { Ok(s.clone()) }
        else { Err(()) }
    }
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

    #[test]
    fn test_new_with_empty_consts() {
        let pattern = vec![ExprPatternComponent::new_const(""),ExprPatternComponent::new_var("D")];
        let check = vec![ExprPatternComponent::new_var("D")];
        assert_eq!(ExprPattern::new(pattern).components,check)
    }

    #[test]
    fn test_new_with_multiple_consts_at_start() {
        let pattern = vec![ExprPatternComponent::new_const("A"),ExprPatternComponent::new_const("B"),ExprPatternComponent::new_const("C"),ExprPatternComponent::new_var("D")];
        let check = vec![ExprPatternComponent::new_const("ABC"),ExprPatternComponent::new_var("D")];
        assert_eq!(ExprPattern::new(pattern).components,check)
    }

    #[test]
    fn test_new_with_multiple_consts_in_middle() {
        let pattern = vec![ExprPatternComponent::new_var("A"),ExprPatternComponent::new_const("B"),ExprPatternComponent::new_const("C"),ExprPatternComponent::new_const("D"),ExprPatternComponent::new_var("E")];
        let check = vec![ExprPatternComponent::new_var("A"),ExprPatternComponent::new_const("BCD"),ExprPatternComponent::new_var("E")];
        assert_eq!(ExprPattern::new(pattern).components,check)
    }

    #[test]
    fn test_new_with_multiple_consts_at_end() {
        let pattern = vec![ExprPatternComponent::new_var("A"),ExprPatternComponent::new_const("B"),ExprPatternComponent::new_const("C"),ExprPatternComponent::new_const("D")];
        let check = vec![ExprPatternComponent::new_var("A"),ExprPatternComponent::new_const("BCD")];
        assert_eq!(ExprPattern::new(pattern).components,check)
    }

    fn pre_test_assign<C1: IntoIterator<Item=ExprPatternComponent>, A: IntoIterator<Item=ExprPatternAssignment>, C2: IntoIterator<Item=ExprPatternComponent>>
    (components: C1, assignments: A, check: C2) -> (Result<ExprPattern,()>,ExprPattern) {(
        ExprPattern::new(components).assign(&ExprPatternAssignments::new(assignments)),
        ExprPattern::new(check)
    )}

    #[test]
    fn test_assign_with_var_assignment() {
        let (assigned,assigned_check) = pre_test_assign(
            [ExprPatternComponent::new_const("one"),ExprPatternComponent::new_var("two"),ExprPatternComponent::new_vars("three","then","four")],
            [ExprPatternAssignment::new_const(),ExprPatternAssignment::new_var("two", "2")],
            [ExprPatternComponent::new_const("one2"),ExprPatternComponent::new_vars("three","then","four")]
        );
        assert_eq!(assigned,Ok(assigned_check));
    }

    #[test]
    fn test_assign_with_vars_assignment() {
        let (assigned,assigned_check) = pre_test_assign(
            [ExprPatternComponent::new_const("one"),ExprPatternComponent::new_vars("two","then","three"),ExprPatternComponent::new_var("four")],
            [ExprPatternAssignment::new_vars("two", "three",vec!["1","2","3"]),ExprPatternAssignment::new_const()],
            [ExprPatternComponent::new_const("one1then2then3"),ExprPatternComponent::new_var("four")]
        );
        assert_eq!(assigned,Ok(assigned_check));
    }

    #[test]
    fn test_assign_with_missing_assignment() {
        let (assigned,_) = pre_test_assign(
            [ExprPatternComponent::new_const("one"),ExprPatternComponent::new_var("four")],
            [ExprPatternAssignment::new_vars("two", "three",vec!["1","2","3"]),ExprPatternAssignment::new_const()],
            [ExprPatternComponent::new_const("one1then2then3"),ExprPatternComponent::new_var("four")]
        );
        assert_eq!(assigned,Err(()));
    }


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
