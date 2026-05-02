use std::{collections::HashSet};

use proof_calculus::{propositions::types::assigned::binding::bounds::GetBoundsForPropIdenticalToProp, utils::collections::binding::{binders::{Binder, GetBinder}, bounds::{GetBounds, UniqueGetBounds}}};

use crate::{expressions::types::assigned::{TblExpression, at_path_enum::{TblExpressionAtPathEnum}, binding::bounds::{TblExpressionIdentityBound, TblPropositionIdentityBound, TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength}, compound::CompoundTblExpression, subexpressions::iterators::depth_first::counterclockwise::CounterclockwiseDepthFirstLocatedTblSubexpressionIterator}, proof_calculus_derived::aliases::propositions::types::TblProposition};

/// [PropositionIdentityBounds] for [TblProposition] which is fast to construct
/// To see [PropositionIdentityBounds] for [TblProposition] which fast to perform lookups with, see [TblExpressionFastLookupIdentityBounds]
#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct TblFastConstructGetBoundsForExprIdenticalToExpr(Box<[TblExpressionIdentityBound]>);
pub type TblFastConstructGetBoundsForPropIdenticalToProp = TblFastConstructGetBoundsForExprIdenticalToExpr;

impl <B: GetBinder<TblExpressionIdentityBound>> GetBounds<B> for TblFastConstructGetBoundsForExprIdenticalToExpr {
    fn get_from<'binder>(&self, binder: &'binder B) -> HashSet<&'binder <B as Binder>::Value>
        { binder.get_intersection(self.0.iter()) }
}
impl <B: GetBinder<TblPropositionIdentityBound>> UniqueGetBounds<B> for TblFastConstructGetBoundsForExprIdenticalToExpr {}
impl <'prop,C: 'prop + CompoundTblExpression, B: GetBinder<TblPropositionIdentityBound>> GetBoundsForPropIdenticalToProp<'prop,TblProposition<C>,B> for TblFastConstructGetBoundsForExprIdenticalToExpr {}
impl <'a, C: CompoundTblExpression> From<&'a TblExpression<C>> for TblFastConstructGetBoundsForPropIdenticalToProp {
    fn from(expr: &'a TblExpression<C>) -> Self {
        let bounds = CounterclockwiseDepthFirstLocatedTblSubexpressionIterator::new(expr)
            .map(|v| 
                match v.into() {
                    TblExpressionAtPathEnum::Atomic(atom) =>
                        TblExpressionBoundAtomExactValue::new(atom.path, *atom.obj).into(),
                    TblExpressionAtPathEnum::Compound(compound) =>
                        TblExpressionBoundCompoundExactLength::new(compound.path, compound.obj.len()).into()
                })
            .collect();
        Self(bounds)
    }
}

impl TblFastConstructGetBoundsForExprIdenticalToExpr {
    pub fn bounds(&self) -> &Box<[TblExpressionIdentityBound]> { &self.0 }
}

