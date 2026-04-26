use path_lib::iterators::depth_first::{DepthFirstLocatedObjAtPathIterator, DepthFirstObjAtPathIterator};

use crate::expressions::assigned::{TblExpression, compound::CompoundTblExpression, subexpressions::{TblSubexpressionInExpression, TblSubexpressionInExpressionPath}};

pub mod clockwise;
pub mod counterclockwise;

pub struct DepthFirstTblSubexpressionIterator<'a,C: CompoundTblExpression,I: IntoIterator<Item=&'a TblExpression<C>>,F: Fn(&'a TblExpression<C>) -> I>(
    DepthFirstObjAtPathIterator<'a,TblExpression<C>,I,F>
);
impl <'a, C: CompoundTblExpression, I: IntoIterator<Item=&'a TblExpression<C>>,F: Fn(&'a TblExpression<C>) -> I> DepthFirstTblSubexpressionIterator<'a,C,I,F> {
    pub fn new(expr: &'a TblExpression<C>, expander: F) -> Self
        { Self(DepthFirstObjAtPathIterator::new(expr, expander)) }
}
impl <'a, C: CompoundTblExpression,I: IntoIterator<Item=&'a TblExpression<C>>,F: Fn(&'a TblExpression<C>) -> I> Iterator for DepthFirstTblSubexpressionIterator<'a,C,I,F> {
    type Item = &'a TblExpression<C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

pub struct DepthFirstLocatedTblSubexpressionIterator<'a,C: CompoundTblExpression, I: IntoIterator<Item=TblSubexpressionInExpression<'a,C>>,F: Fn(TblSubexpressionInExpression<'a,C>) -> I>(
    DepthFirstLocatedObjAtPathIterator<'a,TblExpression<C>,TblSubexpressionInExpressionPath,I,F>
);
impl <'a, C: CompoundTblExpression,I: IntoIterator<Item=TblSubexpressionInExpression<'a,C>>, F: Fn(TblSubexpressionInExpression<'a,C>) -> I> DepthFirstLocatedTblSubexpressionIterator<'a,C,I,F> {
    pub fn new(expr: &'a TblExpression<C>, expander: F) -> Self {
        Self(DepthFirstLocatedObjAtPathIterator::new(TblSubexpressionInExpression { 
            obj: expr,
            path: TblSubexpressionInExpressionPath::default()
        }, expander))
    }
}
impl <'a, C: CompoundTblExpression,I: IntoIterator<Item=TblSubexpressionInExpression<'a,C>>,F: Fn(TblSubexpressionInExpression<'a,C>) -> I> Iterator for DepthFirstLocatedTblSubexpressionIterator<'a,C,I,F> {
    type Item = TblSubexpressionInExpression<'a,C>;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}
