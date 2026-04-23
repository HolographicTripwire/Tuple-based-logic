use std::collections::{HashSet};

use proof_calculus::{propositions::bounds::GetBoundsForPropIdenticalToProp, utils::collections::binders::{Binder, GetBinder, GetBounds, UniqueGetBounds}};

use crate::{expressions::assigned::{TblExpression, at_path_enum::TblSubexpressionInExpressionEnum, binding::bounds::{TblExpressionIdentityBound, TblPropositionIdentityBound, atom_value::TblExpressionBoundAtomExactValue, compound_length::TblExpressionBoundCompoundExactLength}, compound::CompoundTblExpression, subexpressions::{TblSubexpressionInExpression, TblSubexpressionInExpressionPath, immediate::LocatedParentOfImmediateSubexpressions}}, proof_calculus_derived::aliases::propositions::TblProposition};

/// [PropositionIdentityBounds] for [TblProposition] which is fast to construct
/// To see [PropositionIdentityBounds] for [TblProposition] which fast to perform lookups with, see [TblExpressionFastLookupIdentityBounds]
#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct TblFastConstructGetBoundsForExprIdenticalToExpr(Box<[TblExpressionIdentityBound]>);
pub type TblFastConstructGetBoundsForPropIdenticalToProp = TblFastConstructGetBoundsForExprIdenticalToExpr;

impl <'a, B: GetBinder<&'a TblExpressionIdentityBound>> GetBounds<B> for TblFastConstructGetBoundsForExprIdenticalToExpr {
    fn get_from<'b>(&self, binder: &'b B) -> HashSet<&'b <B as Binder>::Value>
        { binder.get_intersection(self.0.iter()) }
}
impl <'a, B: GetBinder<&'a TblPropositionIdentityBound>> UniqueGetBounds<B> for TblFastConstructGetBoundsForExprIdenticalToExpr {}
impl <'a, C: 'a + CompoundTblExpression, B: GetBinder<&'a TblPropositionIdentityBound> + GetBinder<TblPropositionIdentityBound> + GetBinder<TblPropositionIdentityBound>> GetBoundsForPropIdenticalToProp<'a,TblProposition<C>,B> for TblFastConstructGetBoundsForExprIdenticalToExpr {}
impl <'a, C: CompoundTblExpression> From<&'a TblExpression<C>> for TblFastConstructGetBoundsForPropIdenticalToProp {
    fn from(expr: &'a TblExpression<C>) -> Self
        { Self(TblFastConstructGetBoundsIteratorForExprIdenticalToExpr::new(expr).collect()) }
}

impl <'a> TblFastConstructGetBoundsForExprIdenticalToExpr {
    pub fn bounds(&self) -> &Box<[TblExpressionIdentityBound]> { &self.0 }
}
// impl <'a, C: CompoundTblExpression> IntoIterator for TblFastConstructGetBoundsForExprIdenticalToExpr<'a,C> {
//     type Item = TblExpressionIdentityBound;
//     type IntoIter = TblExpressionFastConstructIdentityBoundsIterator<'a,C>;

//     fn into_iter(self) -> Self::IntoIter { TblExpressionFastConstructIdentityBoundsIterator::new(self.0) }
// }

pub (super) struct TblFastConstructGetBoundsIteratorForExprIdenticalToExpr<'a,C: CompoundTblExpression> {
    exprs: Vec<TblSubexpressionInExpression<'a, C>>
}
impl <'a, C: CompoundTblExpression> TblFastConstructGetBoundsIteratorForExprIdenticalToExpr<'a,C> {
    pub fn new(expr: &'a TblExpression<C>) -> Self {
        Self { exprs: vec![TblSubexpressionInExpression { obj: expr, path: TblSubexpressionInExpressionPath::default() }] }
    }

    fn process(&mut self, expr: TblSubexpressionInExpression<'a,C>) -> TblExpressionIdentityBound {
        match expr.into() {
            TblSubexpressionInExpressionEnum::Atomic(atom) =>
                { TblExpressionBoundAtomExactValue::new(atom.path, *atom.obj).into() },
            TblSubexpressionInExpressionEnum::Compound(compound) => {
                let (path, length) = (compound.path.clone(), compound.obj.len());
                self.exprs.extend(compound.into_located_immediate_subexpressions());
                TblExpressionBoundCompoundExactLength::new(path, length).into()
            }
        }
    }

}
impl <'a, C: CompoundTblExpression> Iterator for TblFastConstructGetBoundsIteratorForExprIdenticalToExpr<'a,C> {
    type Item = TblExpressionIdentityBound;
    fn next(&mut self) -> Option<Self::Item>
        { self.exprs.pop().map(|expr| self.process(expr)) }
}
