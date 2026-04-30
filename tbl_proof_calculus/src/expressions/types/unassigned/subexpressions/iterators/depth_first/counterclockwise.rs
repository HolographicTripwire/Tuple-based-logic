use itertools::Itertools;

use crate::expressions::types::unassigned::{UnassignedTblExpression, at_path_enum::UnassignedTblExpressionAtPathEnum, compound::UnassignedCompoundTblExpression, subexpressions::{UnassignedTblSubexpressionInExpression, immediate::LocatedParentOfImmediateUnassignedSubexpressions, iterators::depth_first::{DepthFirstLocatedUnassignedTblSubexpressionIterator, DepthFirstUnassignedTblSubexpressionIterator}}};

pub struct CounterclockwiseDepthFirstUnassignedTblSubexpressionIterator<'a,C: UnassignedCompoundTblExpression>(
    DepthFirstUnassignedTblSubexpressionIterator<'a,C,Vec<&'a UnassignedTblExpression<C>>,fn(&'a UnassignedTblExpression<C>) -> Vec<&'a UnassignedTblExpression<C>>>
);
impl <'a, C: UnassignedCompoundTblExpression> CounterclockwiseDepthFirstUnassignedTblSubexpressionIterator<'a,C> {
    pub fn new(expr: &'a UnassignedTblExpression<C>) -> CounterclockwiseDepthFirstUnassignedTblSubexpressionIterator<'a, C>
        { CounterclockwiseDepthFirstUnassignedTblSubexpressionIterator(DepthFirstUnassignedTblSubexpressionIterator::new(expr, Self::expansion_helper)) }
    
    fn expansion_helper(expr: &'a UnassignedTblExpression<C>) -> Vec<&'a UnassignedTblExpression<C>> {
        if let UnassignedTblExpression::Compound(compound) = expr {
            let mut result = compound.get_immediate_subexpressions().into_iter().collect_vec();
            result.reverse();
            result
        } else { vec![] }
    }
}
impl <'a, C: UnassignedCompoundTblExpression> Iterator for CounterclockwiseDepthFirstUnassignedTblSubexpressionIterator<'a,C> {
    type Item = &'a UnassignedTblExpression<C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

pub struct CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C: UnassignedCompoundTblExpression>(
    DepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C,Vec<UnassignedTblSubexpressionInExpression<'a,C>>,fn(UnassignedTblSubexpressionInExpression<'a,C>) -> Vec<UnassignedTblSubexpressionInExpression<'a,C>>>
);
impl <'a, C: UnassignedCompoundTblExpression> CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C> {
    pub fn new(expr: &'a UnassignedTblExpression<C>) -> CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator<'a, C> {
        CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator(DepthFirstLocatedUnassignedTblSubexpressionIterator::new(expr, Self::expansion_helper))
    }

    fn expansion_helper(expr: UnassignedTblSubexpressionInExpression<'a,C>) -> Vec<UnassignedTblSubexpressionInExpression<'a,C>> {
        if let UnassignedTblExpressionAtPathEnum::Compound(compound) = expr.into() {
            let mut result = compound.into_located_immediate_subexpressions().into_iter().collect_vec();
            result.reverse();
            result
        } else { vec![] }
    }
}
impl <'a, C: UnassignedCompoundTblExpression> Iterator for CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C> {
    type Item = UnassignedTblSubexpressionInExpression<'a,C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}
