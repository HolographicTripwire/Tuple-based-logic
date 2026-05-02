use proof_calculus::{propositions::types::unassigned::binding::bounds::GetBoundsForUpropsSubsumedByUprop, utils::collections::binding::binders::GetBinder};

use crate::{expressions::types::{assigned::binding::{bounds::TblExpressionInsertionBound, operation_bounds::get_subsumed_by_uprop::fast_construct::{TblFastConstructGetBoundsForExprsSubsumedByUexpr, TblFastConstructGetBoundsForPropsSubsumedByUprop}}, unassigned::compound::UnassignedCompoundTblExpression}, proof_calculus_derived::aliases::propositions::types::UnassignedTblProposition};

pub type TblFastConstructGetBoundsForUexprsSubsumedByUexpr = TblFastConstructGetBoundsForExprsSubsumedByUexpr;
pub type TblFastConstructGetBoundsForUpropsSubsumedByUprop = TblFastConstructGetBoundsForPropsSubsumedByUprop;

impl <'prop,C: 'prop + UnassignedCompoundTblExpression, B: GetBinder<TblExpressionInsertionBound>> GetBoundsForUpropsSubsumedByUprop<'prop,UnassignedTblProposition<C>,B> for TblFastConstructGetBoundsForUexprsSubsumedByUexpr {}
