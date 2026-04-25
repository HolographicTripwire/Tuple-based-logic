use std::collections::{HashMap, HashSet};

use proof_calculus::{propositions::bounds::unassigned::GetBoundsForPropsSubsumedByUprop, utils::collections::binders::{Binder, GetBinder, GetBounds, UniqueGetBounds}};

use crate::{expressions::{assigned::{binding::bounds::{TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength, TblExpressionBoundValueDuplicated, TblExpressionSubsumptionBound}, subexpressions::TblSubexpressionInExpressionPath}, unassigned::{UnassignedTblExpression, at_path_enum::UnassignedTblExpressionAtPathEnum, compound::UnassignedCompoundTblExpression, subexpressions::iterators::back_depth_first::BackDepthFirstUnassignedTblExpressionIterator, variable::TblExpressionVariable}}, proof_calculus_derived::aliases::propositions::UnassignedTblProposition};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct TblFastConstructGetBoundsForExprSubsumedByUexpr(Box<[TblExpressionSubsumptionBound]>);
pub type TblFastConstructGetBoundsForPropIdenticalToProp = TblFastConstructGetBoundsForExprSubsumedByUexpr;

impl <B: GetBinder<TblExpressionSubsumptionBound>> GetBounds<B> for TblFastConstructGetBoundsForExprSubsumedByUexpr {
    fn get_from<'binder>(&self, binder: &'binder B) -> HashSet<&'binder <B as Binder>::Value>
        { binder.get_intersection(self.0.iter()) }
}
impl <B: GetBinder<TblExpressionSubsumptionBound>> UniqueGetBounds<B> for TblFastConstructGetBoundsForExprSubsumedByUexpr {}
impl <'prop,C: 'prop + UnassignedCompoundTblExpression, B: GetBinder<TblExpressionSubsumptionBound>> GetBoundsForPropsSubsumedByUprop<'prop,UnassignedTblProposition<C>,B> for TblFastConstructGetBoundsForExprSubsumedByUexpr {}
impl <'a, C: UnassignedCompoundTblExpression> From<&'a UnassignedTblExpression<C>> for TblFastConstructGetBoundsForPropIdenticalToProp {
    fn from(expr: &'a UnassignedTblExpression<C>) -> Self {
        let mut first_var_instances: HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath> = HashMap::new();
        let bounds = BackDepthFirstUnassignedTblExpressionIterator::new(expr)
            .filter_map(|v| 
                match v {
                    UnassignedTblExpressionAtPathEnum::Atomic(atom) =>
                        Some(TblExpressionBoundAtomExactValue::new(atom.path, *atom.obj).into()),
                    UnassignedTblExpressionAtPathEnum::Variable(variable) => { match first_var_instances.get(variable.obj) {
                        Some(path) => 
                            Some(TblExpressionBoundValueDuplicated::new(path.clone(), variable.path).0.into()),
                        None => { first_var_instances.insert(*variable.obj, variable.path); None }
                    }},
                    UnassignedTblExpressionAtPathEnum::Compound(compound) =>
                        Some(TblExpressionBoundCompoundExactLength::new(compound.path, compound.obj.len()).into()),
                })
            .collect();
        Self(bounds)
    }
}

impl <'a> TblFastConstructGetBoundsForExprSubsumedByUexpr {
    pub fn bounds(&self) -> &Box<[TblExpressionSubsumptionBound]> { &self.0 }
}
