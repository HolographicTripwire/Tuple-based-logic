use std::{collections::HashSet};

use proof_calculus::{propositions::unassigned::binding::bounds::GetBoundsForUpropIdenticalToUprop, utils::collections::binding::{binders::{Binder, GetBinder}, bounds::{GetBounds, UniqueGetBounds}}};
use crate::{expressions::{assigned::binding::bounds::{TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength}, unassigned::{UnassignedTblExpression, at_path_enum::UnassignedTblExpressionAtPathEnum, binding::bounds::{UnassignedTblExpressionBoundVariableExactValue, UnassignedTblExpressionIdentityBound}, compound::UnassignedCompoundTblExpression, subexpressions::iterators::depth_first::counterclockwise::CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator}}, proof_calculus_derived::aliases::propositions::UnassignedTblProposition};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct TblFastConstructGetBoundsForUexprIdenticalToUexpr(Box<[UnassignedTblExpressionIdentityBound]>);
pub type TblFastConstructGetBoundsForUpropIdenticalToUprop = TblFastConstructGetBoundsForUexprIdenticalToUexpr;

impl <B: GetBinder<UnassignedTblExpressionIdentityBound>> GetBounds<B> for TblFastConstructGetBoundsForUexprIdenticalToUexpr {
    fn get_from<'binder>(&self, binder: &'binder B) -> HashSet<&'binder <B as Binder>::Value>
        { binder.get_intersection(self.0.iter()) }
}
impl <B: GetBinder<UnassignedTblExpressionIdentityBound>> UniqueGetBounds<B> for TblFastConstructGetBoundsForUexprIdenticalToUexpr {}
impl <'prop,C: 'prop + UnassignedCompoundTblExpression, B: GetBinder<UnassignedTblExpressionIdentityBound>> GetBoundsForUpropIdenticalToUprop<'prop,UnassignedTblProposition<C>,B> for TblFastConstructGetBoundsForUexprIdenticalToUexpr {}
impl <'a, C: UnassignedCompoundTblExpression> From<&'a UnassignedTblExpression<C>> for TblFastConstructGetBoundsForUpropIdenticalToUprop {
    fn from(expr: &'a UnassignedTblExpression<C>) -> Self {
        let bounds = CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator::new(expr)
            .map(|v| 
                match v.into() {
                    UnassignedTblExpressionAtPathEnum::Atomic(atom) =>
                        TblExpressionBoundAtomExactValue::new(atom.path, *atom.obj).into(),
                    UnassignedTblExpressionAtPathEnum::Variable(variable) =>
                        UnassignedTblExpressionBoundVariableExactValue::new(variable.path, *variable.obj).into(),
                    UnassignedTblExpressionAtPathEnum::Compound(compound) =>
                        TblExpressionBoundCompoundExactLength::new(compound.path, compound.obj.len()).into()
                })
            .collect();
        Self(bounds)
    }
}

impl TblFastConstructGetBoundsForUexprIdenticalToUexpr {
    pub fn bounds(&self) -> &Box<[UnassignedTblExpressionIdentityBound]> { &self.0 }
}
