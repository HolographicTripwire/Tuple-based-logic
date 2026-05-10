use proof_calculus::propositions::types::unassigned::binding::binders::{
    GetBinderForUpropEquivalentToUprop, GetBinderForUpropIdenticalToUprop,
    GetBinderForUpropsSubsumedByUprop, GetBinderForUpropsSubsumingProp,
    GetBinderForUpropsSubsumingUprop, InsertBinderForUprop,
};
use trait_aliases::trait_aliases;

use crate::{
    expressions::types::{
        assigned::compound::TblExpressionCompound,
        unassigned::compound::UnassignedTblExpressionCompound,
    },
    proof_calculus_derived::aliases::propositions::types::{
        assigned::TblProposition, unassigned::UnassignedTblProposition,
    },
};

trait_aliases! {
    // GetUpropIdenticalToUprop
    pub trait GetBinderForTblUpropIdenticalToUprop<
        ElemUcompound: UnassignedTblExpressionCompound
    > = GetBinderForUpropIdenticalToUprop<
        UnassignedTblProposition<ElemUcompound>
    >;
    // GetUpropEquivalentToUprop
    pub trait GetBinderForTblUpropEquivalentToUprop<
        ElemUcompound: UnassignedTblExpressionCompound
    > = GetBinderForUpropEquivalentToUprop<
        UnassignedTblProposition<ElemUcompound>
    >;

    // GetUpropsSubsumedByUprop
    pub trait GetBinderForTblUpropsSubsumedByUprop<
        SubsumerElemUcompound: UnassignedTblExpressionCompound
    > = GetBinderForUpropsSubsumedByUprop<
        UnassignedTblProposition<SubsumerElemUcompound>
    >;
    // GetUpropsSubsumingProp
    pub trait GetBinderForTblUpropsSubsumingProp<
        SubsumedElemCompound: TblExpressionCompound
    > = GetBinderForUpropsSubsumingProp<
        TblProposition<SubsumedElemCompound>
    >;
    // GetUpropsSubsumingUprop
    pub trait GetBinderForTblUpropsSubsumingUprop<
        SubsumedElemUcompound: UnassignedTblExpressionCompound
    > = GetBinderForUpropsSubsumingUprop<
        UnassignedTblProposition<SubsumedElemUcompound>
    >;

    // InsertUprop
    pub trait InsertBinderForTblUprop<
        'elem,
        ElemUcompound: 'elem + UnassignedTblExpressionCompound>
    = InsertBinderForUprop<
        'elem,
        UnassignedTblProposition<ElemUcompound>
    >;
}
