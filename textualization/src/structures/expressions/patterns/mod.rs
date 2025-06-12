use hashable::HashableHashSet;
use parsertools::{pred, Parser};

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
        assignments.into_iter().filter(|assignment| assignment == &ExprPatternAssignment::Constant).collect()
    )}
}

fn expr_pattern_matcher<'a>(pattern: &'a ExprPattern) -> Parser<'a, char, Vec<ExprPatternAssignment>> {
    let unary_vec_parser = pattern.components.iter()
        // Convert components to assignment vec parsers
        .map(|component| pattern_component_parser(component)
            .map(|assignment| vec![assignment] )).collect::<Vec<_>>();
    let vec_parser = unary_vec_parser.iter().fold(
        pred(|_: &char| Some(Vec::<ExprPatternAssignment>::new())),
        |acc, next| acc.then(next.clone()).map(|(left,right)| [left,right].concat()));
    vec_parser
}
