use proof_calculus::utils::collections::optional_iterator::OptionalIterator;

use crate::expressions::assigned::{TblExpression, at_path_enum::TblExpressionAtPathEnum, compound::CompoundTblExpression, subexpressions::{TblSubexpressionInExpression, immediate::LocatedParentOfImmediateSubexpressions, iterators::depth_first::{DepthFirstLocatedTblSubexpressionIterator, DepthFirstTblSubexpressionIterator}}};

pub struct ClockwiseDepthFirstTblSubexpressionIterator<'a,C: CompoundTblExpression,I: IntoIterator<Item=&'a TblExpression<C>>>(
    DepthFirstTblSubexpressionIterator<'a,C,I,fn(&'a TblExpression<C>) -> I>
);
impl <'a, C: CompoundTblExpression, I: IntoIterator<Item=&'a TblExpression<C>>> ClockwiseDepthFirstTblSubexpressionIterator<'a,C,I> {
    pub fn new(expr: &'a TblExpression<C>) -> ClockwiseDepthFirstTblSubexpressionIterator<'a, C, impl IntoIterator<Item = &'a TblExpression<C>>>
        { ClockwiseDepthFirstTblSubexpressionIterator(DepthFirstTblSubexpressionIterator::new(expr, Self::expansion_helper)) }
    
    fn expansion_helper(expr: &'a TblExpression<C>) -> impl IntoIterator<Item = &'a TblExpression<C>> {
        if let TblExpression::Compound(compound) = expr {
            OptionalIterator::Some(compound.get_immediate_subexpressions().into_iter())
        } else { OptionalIterator::None }
    }
}
impl <'a, C: CompoundTblExpression,I: IntoIterator<Item=&'a TblExpression<C>>> Iterator for ClockwiseDepthFirstTblSubexpressionIterator<'a,C,I> {
    type Item = &'a TblExpression<C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

pub struct ClockwiseDepthFirstLocatedTblSubexpressionIterator<'a,C: CompoundTblExpression, I: IntoIterator<Item=TblSubexpressionInExpression<'a,C>>>(
    DepthFirstLocatedTblSubexpressionIterator<'a,C,I,fn(TblSubexpressionInExpression<'a,C>) -> I>
);
impl <'a, C: CompoundTblExpression,I: IntoIterator<Item=TblSubexpressionInExpression<'a,C>>> ClockwiseDepthFirstLocatedTblSubexpressionIterator<'a,C,I> {
    pub fn new(expr: &'a TblExpression<C>) -> ClockwiseDepthFirstLocatedTblSubexpressionIterator<'a, C, impl IntoIterator<Item = TblSubexpressionInExpression<'a,C>>> {
        ClockwiseDepthFirstLocatedTblSubexpressionIterator(DepthFirstLocatedTblSubexpressionIterator::new(expr, Self::expansion_helper))
    }

    fn expansion_helper(expr: TblSubexpressionInExpression<'a,C>) -> impl IntoIterator<Item = TblSubexpressionInExpression<'a,C>> {
        if let TblExpressionAtPathEnum::Compound(compound) = expr.into() {
            OptionalIterator::Some(compound.into_located_immediate_subexpressions().into_iter())
        } else { OptionalIterator::None }
    }
}
impl <'a, C: CompoundTblExpression,I: IntoIterator<Item=TblSubexpressionInExpression<'a,C>>> Iterator for ClockwiseDepthFirstLocatedTblSubexpressionIterator<'a,C,I> {
    type Item = TblSubexpressionInExpression<'a,C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}
