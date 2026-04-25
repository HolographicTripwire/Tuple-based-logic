use proof_calculus::{propositions::bounds::unassigned::GetBoundsForUpropsSubsumedByUprop, utils::collections::binders::GetBinder};

use crate::{expressions::{assigned::binding::{bounds::TblExpressionInsertionBound, operation_bounds::get_subsumed_by_uprop::fast_construct::{TblFastConstructGetBoundsForExprsSubsumedByUexpr, TblFastConstructGetBoundsForPropsSubsumedByUprop}}, unassigned::compound::UnassignedCompoundTblExpression}, proof_calculus_derived::aliases::propositions::UnassignedTblProposition};

pub type TblFastConstructGetBoundsForUexprsSubsumedByUexpr = TblFastConstructGetBoundsForExprsSubsumedByUexpr;
pub type TblFastConstructGetBoundsForUpropsSubsumedByUprop = TblFastConstructGetBoundsForPropsSubsumedByUprop;

impl <'prop,C: 'prop + UnassignedCompoundTblExpression, B: GetBinder<TblExpressionInsertionBound>> GetBoundsForUpropsSubsumedByUprop<'prop,UnassignedTblProposition<C>,B> for TblFastConstructGetBoundsForUexprsSubsumedByUexpr {}
