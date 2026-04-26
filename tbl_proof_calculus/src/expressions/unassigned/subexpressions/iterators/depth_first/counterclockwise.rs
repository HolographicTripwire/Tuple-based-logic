use itertools::Itertools;
use proof_calculus::utils::collections::optional_iterator::OptionalIterator;

use crate::expressions::unassigned::{UnassignedTblExpression, at_path_enum::UnassignedTblExpressionAtPathEnum, compound::UnassignedCompoundTblExpression, subexpressions::{UnassignedTblSubexpressionInExpression, immediate::LocatedParentOfImmediateUnassignedSubexpressions, iterators::depth_first::{DepthFirstLocatedUnassignedTblSubexpressionIterator, DepthFirstUnassignedTblSubexpressionIterator}}};

pub struct CounterclockwiseDepthFirstUnassignedTblSubexpressionIterator<'a,C: UnassignedCompoundTblExpression,I: IntoIterator<Item=&'a UnassignedTblExpression<C>>>(
    DepthFirstUnassignedTblSubexpressionIterator<'a,C,I,fn(&'a UnassignedTblExpression<C>) -> I>
);
impl <'a, C: UnassignedCompoundTblExpression, I: IntoIterator<Item=&'a UnassignedTblExpression<C>>> CounterclockwiseDepthFirstUnassignedTblSubexpressionIterator<'a,C,I> {
    pub fn new(expr: &'a UnassignedTblExpression<C>) -> CounterclockwiseDepthFirstUnassignedTblSubexpressionIterator<'a, C, impl IntoIterator<Item = &'a UnassignedTblExpression<C>>>
        { CounterclockwiseDepthFirstUnassignedTblSubexpressionIterator(DepthFirstUnassignedTblSubexpressionIterator::new(expr, Self::expansion_helper)) }
    
    fn expansion_helper(expr: &'a UnassignedTblExpression<C>) -> impl IntoIterator<Item = &'a UnassignedTblExpression<C>> {
        if let UnassignedTblExpression::Compound(compound) = expr {
            OptionalIterator::Some(compound.get_immediate_subexpressions().into_iter().collect_vec().into_iter().rev())
        } else { OptionalIterator::None }
    }
}
impl <'a, C: UnassignedCompoundTblExpression,I: IntoIterator<Item=&'a UnassignedTblExpression<C>>> Iterator for CounterclockwiseDepthFirstUnassignedTblSubexpressionIterator<'a,C,I> {
    type Item = &'a UnassignedTblExpression<C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

pub struct CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C: UnassignedCompoundTblExpression, I: IntoIterator<Item=UnassignedTblSubexpressionInExpression<'a,C>>>(
    DepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C,I,fn(UnassignedTblSubexpressionInExpression<'a,C>) -> I>
);
impl <'a, C: UnassignedCompoundTblExpression,I: IntoIterator<Item=UnassignedTblSubexpressionInExpression<'a,C>>> CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C,I> {
    pub fn new(expr: &'a UnassignedTblExpression<C>) -> CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator<'a, C, impl IntoIterator<Item = UnassignedTblSubexpressionInExpression<'a,C>>> {
        CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator(DepthFirstLocatedUnassignedTblSubexpressionIterator::new(expr, Self::expansion_helper))
    }

    fn expansion_helper(expr: UnassignedTblSubexpressionInExpression<'a,C>) -> impl IntoIterator<Item = UnassignedTblSubexpressionInExpression<'a,C>> {
        if let UnassignedTblExpressionAtPathEnum::Compound(compound) = expr.into() {
            OptionalIterator::Some(compound.into_located_immediate_subexpressions().into_iter().collect_vec().into_iter().rev())
        } else { OptionalIterator::None }
    }
}
impl <'a, C: UnassignedCompoundTblExpression,I: IntoIterator<Item=UnassignedTblSubexpressionInExpression<'a,C>>> Iterator for CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C,I> {
    type Item = UnassignedTblSubexpressionInExpression<'a,C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}
