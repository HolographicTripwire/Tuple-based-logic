use std::collections::{HashMap, HashSet};

use proof_calculus::{propositions::assigned::binding::bounds::GetBoundsForPropsSubsumedByUprop, utils::collections::binding::{binders::{Binder, GetBinder}, bounds::{GetBounds, UniqueGetBounds}}};

use crate::{expressions::{assigned::{binding::bounds::{TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength, TblExpressionBoundValueDuplicated, TblExpressionInsertionBound}, subexpressions::TblSubexpressionInExpressionPath}, unassigned::{UnassignedTblExpression, at_path_enum::UnassignedTblExpressionAtPathEnum, compound::UnassignedCompoundTblExpression, subexpressions::iterators::depth_first::counterclockwise::CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator, variable::TblExpressionVariable}}, proof_calculus_derived::aliases::propositions::UnassignedTblProposition};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct TblFastConstructGetBoundsForExprsSubsumedByUexpr(Box<[TblExpressionInsertionBound]>);
pub type TblFastConstructGetBoundsForPropsSubsumedByUprop = TblFastConstructGetBoundsForExprsSubsumedByUexpr;

impl <B: GetBinder<TblExpressionInsertionBound>> GetBounds<B> for TblFastConstructGetBoundsForExprsSubsumedByUexpr {
    fn get_from<'binder>(&self, binder: &'binder B) -> HashSet<&'binder <B as Binder>::Value>
        { binder.get_intersection(self.0.iter()) }
}
impl <B: GetBinder<TblExpressionInsertionBound>> UniqueGetBounds<B> for TblFastConstructGetBoundsForExprsSubsumedByUexpr {}
impl <'prop,C: 'prop + UnassignedCompoundTblExpression, B: GetBinder<TblExpressionInsertionBound>> GetBoundsForPropsSubsumedByUprop<'prop,UnassignedTblProposition<C>,B> for TblFastConstructGetBoundsForExprsSubsumedByUexpr {}
impl <'a, C: UnassignedCompoundTblExpression> From<&'a UnassignedTblExpression<C>> for TblFastConstructGetBoundsForPropsSubsumedByUprop {
    fn from(expr: &'a UnassignedTblExpression<C>) -> Self {
        let mut first_var_instances: HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath> = HashMap::new();
        let bounds = CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator::new(expr)
            .filter_map(|v| 
                match v.into() {
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

impl TblFastConstructGetBoundsForExprsSubsumedByUexpr {
    pub fn bounds(&self) -> &Box<[TblExpressionInsertionBound]> { &self.0 }
}
