use path_lib::iterators::depth_first::{DepthFirstLocatedObjAtPathIterator, DepthFirstObjAtPathIterator};

use crate::expressions::{assigned::subexpressions::TblSubexpressionInExpressionPath, unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression, subexpressions::UnassignedTblSubexpressionInExpression}};

pub mod clockwise;
pub mod counterclockwise;

pub struct DepthFirstUnassignedTblSubexpressionIterator<'a,C: UnassignedCompoundTblExpression,I: IntoIterator<Item=&'a UnassignedTblExpression<C>>,F: Fn(&'a UnassignedTblExpression<C>) -> I>(
    DepthFirstObjAtPathIterator<'a,UnassignedTblExpression<C>,I,F>
);
impl <'a, C: UnassignedCompoundTblExpression, I: IntoIterator<Item=&'a UnassignedTblExpression<C>>,F: Fn(&'a UnassignedTblExpression<C>) -> I> DepthFirstUnassignedTblSubexpressionIterator<'a,C,I,F> {
    pub fn new(expr: &'a UnassignedTblExpression<C>, expander: F) -> Self
        { Self(DepthFirstObjAtPathIterator::new(expr, expander)) }
}
impl <'a, C: UnassignedCompoundTblExpression,I: IntoIterator<Item=&'a UnassignedTblExpression<C>>,F: Fn(&'a UnassignedTblExpression<C>) -> I> Iterator for DepthFirstUnassignedTblSubexpressionIterator<'a,C,I,F> {
    type Item = &'a UnassignedTblExpression<C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

pub struct DepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C: UnassignedCompoundTblExpression, I: IntoIterator<Item=UnassignedTblSubexpressionInExpression<'a,C>>,F: Fn(UnassignedTblSubexpressionInExpression<'a,C>) -> I>(
    DepthFirstLocatedObjAtPathIterator<'a,UnassignedTblExpression<C>,TblSubexpressionInExpressionPath,I,F>
);
impl <'a, C: UnassignedCompoundTblExpression,I: IntoIterator<Item=UnassignedTblSubexpressionInExpression<'a,C>>, F: Fn(UnassignedTblSubexpressionInExpression<'a,C>) -> I> DepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C,I,F> {
    pub fn new(expr: &'a UnassignedTblExpression<C>, expander: F) -> Self {
        Self(DepthFirstLocatedObjAtPathIterator::new(UnassignedTblSubexpressionInExpression { 
            obj: expr,
            path: TblSubexpressionInExpressionPath::default()
        }, expander))
    }
}
impl <'a, C: UnassignedCompoundTblExpression,I: IntoIterator<Item=UnassignedTblSubexpressionInExpression<'a,C>>,F: Fn(UnassignedTblSubexpressionInExpression<'a,C>) -> I> Iterator for DepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C,I,F> {
    type Item = UnassignedTblSubexpressionInExpression<'a,C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}
