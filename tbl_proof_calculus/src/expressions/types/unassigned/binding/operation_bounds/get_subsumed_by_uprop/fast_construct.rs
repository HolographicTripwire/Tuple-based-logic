use proof_calculus::{
    propositions::types::unassigned::binding::bounds::GetBoundsForUpropsSubsumedByUprop,
    utils::collections::binding::binders::GetBinder,
};

use crate::{
    expressions::types::{
        assigned::binding::{
            bounds::TblExpressionInsertionBound,
            operation_bounds::get_subsumed_by_uprop::fast_construct::{
                TblFastConstructGetBoundsForExprsSubsumedByUexpr,
                TblFastConstructGetBoundsForPropsSubsumedByUprop,
            },
        },
        unassigned::compound::UnassignedTblExpressionCompound,
    },
    proof_calculus_derived::aliases::propositions::types::unassigned::UnassignedTblProposition,
};

pub type TblFastConstructGetBoundsForUexprsSubsumedByUexpr =
    TblFastConstructGetBoundsForExprsSubsumedByUexpr;
pub type TblFastConstructGetBoundsForUpropsSubsumedByUprop =
    TblFastConstructGetBoundsForPropsSubsumedByUprop;

impl<
    'elem,
    ElemUcompound: 'elem + UnassignedTblExpressionCompound,
    B: GetBinder<TblExpressionInsertionBound>,
> GetBoundsForUpropsSubsumedByUprop<'elem, UnassignedTblProposition<ElemUcompound>, B>
    for TblFastConstructGetBoundsForUexprsSubsumedByUexpr
{
}
