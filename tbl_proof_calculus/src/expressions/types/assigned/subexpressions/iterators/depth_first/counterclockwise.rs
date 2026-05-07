use itertools::Itertools;

use crate::expressions::types::assigned::{TblExpression, at_path_enum::TblExpressionAtPathEnum, compound::TblExpressionCompound, subexpressions::{LocatedParentOfImmediateSubexpressions, TblSubexpressionInExpression, iterators::depth_first::{DepthFirstLocatedTblSubexpressionIterator, DepthFirstTblSubexpressionIterator}}};

pub struct CounterclockwiseDepthFirstTblSubexpressionIterator<'a,C: TblExpressionCompound>(
    DepthFirstTblSubexpressionIterator<'a,C,Vec<&'a TblExpression<C>>,fn(&'a TblExpression<C>) -> Vec<&'a TblExpression<C>>>
);
impl <'a, C: TblExpressionCompound> CounterclockwiseDepthFirstTblSubexpressionIterator<'a,C> {
    pub fn new(expr: &'a TblExpression<C>) -> CounterclockwiseDepthFirstTblSubexpressionIterator<'a, C>
        { CounterclockwiseDepthFirstTblSubexpressionIterator(DepthFirstTblSubexpressionIterator::new(expr, Self::expansion_helper)) }
    
    fn expansion_helper(expr: &'a TblExpression<C>) -> Vec<&'a TblExpression<C>> {
        if let TblExpression::Compound(compound) = expr {
            let mut result = compound.get_immediate_subexpressions().into_iter().collect_vec();
            result.reverse();
            result
        } else { vec![] }
    }
}
impl <'a, C: TblExpressionCompound> Iterator for CounterclockwiseDepthFirstTblSubexpressionIterator<'a,C> {
    type Item = &'a TblExpression<C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

pub struct CounterclockwiseDepthFirstLocatedTblSubexpressionIterator<'a,C: TblExpressionCompound>(
    DepthFirstLocatedTblSubexpressionIterator<'a,C,Vec<TblSubexpressionInExpression<'a,C>>,fn(TblSubexpressionInExpression<'a,C>) -> Vec<TblSubexpressionInExpression<'a,C>>>
);
impl <'a, C: TblExpressionCompound> CounterclockwiseDepthFirstLocatedTblSubexpressionIterator<'a,C> {
    pub fn new(expr: &'a TblExpression<C>) -> CounterclockwiseDepthFirstLocatedTblSubexpressionIterator<'a, C> {
        CounterclockwiseDepthFirstLocatedTblSubexpressionIterator(DepthFirstLocatedTblSubexpressionIterator::new(expr, Self::expansion_helper))
    }

    fn expansion_helper(expr: TblSubexpressionInExpression<'a,C>) -> Vec<TblSubexpressionInExpression<'a,C>> {
        if let TblExpressionAtPathEnum::Compound(compound) = expr.into() {
            let mut result = compound.into_located_immediate_subexpressions().into_iter().collect_vec();
            result.reverse();
            result
        } else { vec![] }
    }
}
impl <'a, C: TblExpressionCompound> Iterator for CounterclockwiseDepthFirstLocatedTblSubexpressionIterator<'a,C> {
    type Item = TblSubexpressionInExpression<'a,C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}
