use proof_calculus::utils::collections::dense_usize_map::DenseUsizeMap;

use crate::{generation::expressions::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression}, structures::expressions::{TblExpression, compound::CompoundTblExpression}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct TblExpressionAssignment<C: CompoundTblExpression>(pub DenseUsizeMap<TblExpression<C>>);
impl <C: CompoundTblExpression> Default for TblExpressionAssignment<C> {
    fn default() -> Self { Self(Default::default()) }
}
impl <const N: usize, C:CompoundTblExpression> From<[(usize,TblExpression<C>); N]> for TblExpressionAssignment<C> {
    fn from(pairs: [(usize,TblExpression<C>); N]) -> Self { Self(pairs.into()) }
}
impl <C: CompoundTblExpression> From<Vec<(usize,TblExpression<C>)>> for TblExpressionAssignment<C> {
    fn from(pairs: Vec<(usize,TblExpression<C>)>) -> Self { Self(pairs.into()) }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct PartialTblExpressionAssignment<C: UnassignedCompoundTblExpression>(pub DenseUsizeMap<UnassignedTblExpression<C>>);
impl <C: UnassignedCompoundTblExpression> Default for PartialTblExpressionAssignment<C> {
    fn default() -> Self { Self(Default::default()) }
}
impl <const N: usize, C: UnassignedCompoundTblExpression> From<[(usize,UnassignedTblExpression<C>); N]> for PartialTblExpressionAssignment<C> {
    fn from(pairs: [(usize,UnassignedTblExpression<C>); N]) -> Self { Self(pairs.into()) }
}
impl <C: UnassignedCompoundTblExpression> From<Vec<(usize,UnassignedTblExpression<C>)>> for PartialTblExpressionAssignment<C> {
    fn from(pairs: Vec<(usize,UnassignedTblExpression<C>)>) -> Self { Self(pairs.into()) }
}
