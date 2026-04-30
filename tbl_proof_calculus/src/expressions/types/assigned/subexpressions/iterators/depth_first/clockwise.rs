use itertools::Itertools;

use crate::expressions::types::assigned::{TblExpression, at_path_enum::TblExpressionAtPathEnum, compound::CompoundTblExpression, subexpressions::{LocatedParentOfImmediateSubexpressions, TblSubexpressionInExpression, iterators::depth_first::{DepthFirstLocatedTblSubexpressionIterator, DepthFirstTblSubexpressionIterator}}};

pub struct ClockwiseDepthFirstTblSubexpressionIterator<'a,C: CompoundTblExpression>(
    DepthFirstTblSubexpressionIterator<'a,C,Vec<&'a TblExpression<C>>,fn(&'a TblExpression<C>) -> Vec<&'a TblExpression<C>>>
);
impl <'a, C: CompoundTblExpression> ClockwiseDepthFirstTblSubexpressionIterator<'a,C> {
    pub fn new(expr: &'a TblExpression<C>) -> ClockwiseDepthFirstTblSubexpressionIterator<'a, C>
        { ClockwiseDepthFirstTblSubexpressionIterator(DepthFirstTblSubexpressionIterator::new(expr, Self::expansion_helper)) }
    
    fn expansion_helper(expr: &'a TblExpression<C>) -> Vec<&'a TblExpression<C>> {
        if let TblExpression::Compound(compound) = expr {
            compound.get_immediate_subexpressions().into_iter().collect_vec()
        } else { vec![] }
    }
}
impl <'a, C: CompoundTblExpression> Iterator for ClockwiseDepthFirstTblSubexpressionIterator<'a,C> {
    type Item = &'a TblExpression<C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

pub struct ClockwiseDepthFirstLocatedTblSubexpressionIterator<'a,C: CompoundTblExpression>(
    DepthFirstLocatedTblSubexpressionIterator<'a,C,Vec<TblSubexpressionInExpression<'a,C>>,fn(TblSubexpressionInExpression<'a,C>) -> Vec<TblSubexpressionInExpression<'a,C>>>
);
impl <'a, C: CompoundTblExpression> ClockwiseDepthFirstLocatedTblSubexpressionIterator<'a,C> {
    pub fn new(expr: &'a TblExpression<C>) -> ClockwiseDepthFirstLocatedTblSubexpressionIterator<'a, C> {
        ClockwiseDepthFirstLocatedTblSubexpressionIterator(DepthFirstLocatedTblSubexpressionIterator::new(expr, Self::expansion_helper))
    }

    fn expansion_helper(expr: TblSubexpressionInExpression<'a,C>) -> Vec<TblSubexpressionInExpression<'a,C>> {
        if let TblExpressionAtPathEnum::Compound(compound) = expr.into() {
            compound.into_located_immediate_subexpressions().into_iter().collect_vec()
        } else { vec![] }
    }
}
impl <'a, C: CompoundTblExpression> Iterator for ClockwiseDepthFirstLocatedTblSubexpressionIterator<'a,C> {
    type Item = TblSubexpressionInExpression<'a,C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}
