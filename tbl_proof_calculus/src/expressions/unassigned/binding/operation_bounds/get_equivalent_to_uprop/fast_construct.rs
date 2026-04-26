use std::collections::{HashMap, HashSet};

use proof_calculus::{propositions::bounds::unassigned::GetBoundsForUpropsEquivalentToUprop, utils::collections::binders::{Binder, GetBinder, GetBounds, UniqueGetBounds}};
use crate::{expressions::{assigned::{binding::bounds::{TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength, TblExpressionBoundValueDuplicated}, subexpressions::TblSubexpressionInExpressionPath}, unassigned::{UnassignedTblExpression, at_path_enum::UnassignedTblExpressionAtPathEnum, binding::bounds::{TblExpressionBoundVariableExistsAtLocation, UnassignedTblExpressionEquivalenceBound}, compound::UnassignedCompoundTblExpression, subexpressions::iterators::{back_depth_first::BackDepthFirstUnassignedTblSubexpressionIterator, depth_first::counterclockwise::CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator}, variable::TblExpressionVariable}}, proof_calculus_derived::aliases::propositions::UnassignedTblProposition};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct TblFastConstructGetBoundsForUexprsEquivalentToUexpr(Box<[UnassignedTblExpressionEquivalenceBound]>);
pub type TblFastConstructGetBoundsForUpropsEquivalentToUprop = TblFastConstructGetBoundsForUexprsEquivalentToUexpr;

impl <B: GetBinder<UnassignedTblExpressionEquivalenceBound>> GetBounds<B> for TblFastConstructGetBoundsForUexprsEquivalentToUexpr {
    fn get_from<'binder>(&self, binder: &'binder B) -> HashSet<&'binder <B as Binder>::Value>
        { binder.get_intersection(self.0.iter()) }
}
impl <B: GetBinder<UnassignedTblExpressionEquivalenceBound>> UniqueGetBounds<B> for TblFastConstructGetBoundsForUexprsEquivalentToUexpr {}
impl <'prop,C: 'prop + UnassignedCompoundTblExpression, B: GetBinder<UnassignedTblExpressionEquivalenceBound>> GetBoundsForUpropsEquivalentToUprop<'prop,UnassignedTblProposition<C>,B> for TblFastConstructGetBoundsForUexprsEquivalentToUexpr {}
impl <'a, C: UnassignedCompoundTblExpression> From<&'a UnassignedTblExpression<C>> for TblFastConstructGetBoundsForUpropsEquivalentToUprop {
    fn from(expr: &'a UnassignedTblExpression<C>) -> Self {
        let mut first_var_instances: HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath> = HashMap::new();
        let mut bounds = Vec::new();
        for expr in CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator::new(expr) { match expr.into() {
            UnassignedTblExpressionAtPathEnum::Atomic(atom) =>
                bounds.push(TblExpressionBoundAtomExactValue::new(atom.path, *atom.obj).into()),
            UnassignedTblExpressionAtPathEnum::Variable(variable) => {
                match first_var_instances.get(variable.obj) {
                    Some(path) =>
                        bounds.push(TblExpressionBoundValueDuplicated::new(path.clone(), variable.path.clone()).0.into()),
                    None => { first_var_instances.insert(*variable.obj, variable.path.clone()); }
                };
                bounds.push(TblExpressionBoundVariableExistsAtLocation::new(variable.path).into());
            }, UnassignedTblExpressionAtPathEnum::Compound(compound) =>
                bounds.push(TblExpressionBoundCompoundExactLength::new(compound.path, compound.obj.len()).into())
        }}
        Self(bounds.into())
    }
}

impl TblFastConstructGetBoundsForUexprsEquivalentToUexpr {
    pub fn bounds(&self) -> &Box<[UnassignedTblExpressionEquivalenceBound]> { &self.0 }
}
