use itertools::Itertools;

use crate::expressions::unassigned::{UnassignedTblExpression, at_path_enum::UnassignedTblExpressionAtPathEnum, compound::UnassignedCompoundTblExpression, subexpressions::{UnassignedTblSubexpressionInExpression, immediate::LocatedParentOfImmediateUnassignedSubexpressions, iterators::depth_first::{DepthFirstLocatedUnassignedTblSubexpressionIterator, DepthFirstUnassignedTblSubexpressionIterator}}};

pub struct ClockwiseDepthFirstUnassignedTblSubexpressionIterator<'a,C: UnassignedCompoundTblExpression>(
    DepthFirstUnassignedTblSubexpressionIterator<'a,C,Vec<&'a UnassignedTblExpression<C>>,fn(&'a UnassignedTblExpression<C>) -> Vec<&'a UnassignedTblExpression<C>>>
);
impl <'a, C: UnassignedCompoundTblExpression> ClockwiseDepthFirstUnassignedTblSubexpressionIterator<'a,C> {
    pub fn new(expr: &'a UnassignedTblExpression<C>) -> ClockwiseDepthFirstUnassignedTblSubexpressionIterator<'a, C>
        { ClockwiseDepthFirstUnassignedTblSubexpressionIterator(DepthFirstUnassignedTblSubexpressionIterator::new(expr, Self::expansion_helper)) }
    
    fn expansion_helper(expr: &'a UnassignedTblExpression<C>) -> Vec<&'a UnassignedTblExpression<C>> {
        if let UnassignedTblExpression::Compound(compound) = expr {
            compound.get_immediate_subexpressions().into_iter().collect_vec()
        } else { vec![] }
    }
}
impl <'a, C: UnassignedCompoundTblExpression> Iterator for ClockwiseDepthFirstUnassignedTblSubexpressionIterator<'a,C> {
    type Item = &'a UnassignedTblExpression<C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

pub struct ClockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C: UnassignedCompoundTblExpression>(
    DepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C,Vec<UnassignedTblSubexpressionInExpression<'a,C>>,fn(UnassignedTblSubexpressionInExpression<'a,C>) -> Vec<UnassignedTblSubexpressionInExpression<'a,C>>>
);
impl <'a, C: UnassignedCompoundTblExpression> ClockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C> {
    pub fn new(expr: &'a UnassignedTblExpression<C>) -> ClockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator<'a, C> {
        ClockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator(DepthFirstLocatedUnassignedTblSubexpressionIterator::new(expr, Self::expansion_helper))
    }

    fn expansion_helper(expr: UnassignedTblSubexpressionInExpression<'a,C>) -> Vec<UnassignedTblSubexpressionInExpression<'a,C>> {
        if let UnassignedTblExpressionAtPathEnum::Compound(compound) = expr.into() {
            compound.into_located_immediate_subexpressions().into_iter().collect_vec()
        } else { vec![] }
    }
}
impl <'a, C: UnassignedCompoundTblExpression> Iterator for ClockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C> {
    type Item = UnassignedTblSubexpressionInExpression<'a,C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}
