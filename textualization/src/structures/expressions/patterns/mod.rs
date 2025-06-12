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
