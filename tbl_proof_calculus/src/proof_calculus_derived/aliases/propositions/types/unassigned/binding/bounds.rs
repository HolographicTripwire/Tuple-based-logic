use proof_calculus::{
    propositions::{
        assignments::{PartialPropositionalAssignment, PropositionalAssignment},
        types::unassigned::binding::bounds::{
            GetBoundsForConstructibleUpropsEquivalentToUprop,
            GetBoundsForConstructibleUpropsSubsumedByUprop,
            GetBoundsForConstructibleUpropsSubsumingProp,
            GetBoundsForConstructibleUpropsSubsumingUprop, GetBoundsForUpropIdenticalToUprop,
            GetBoundsForUpropsEquivalentToUprop, GetBoundsForUpropsSubsumedByUprop,
            GetBoundsForUpropsSubsumingProp, GetBoundsForUpropsSubsumingUprop,
            InsertBoundsForUprop,
        },
    },
    utils::collections::binding::binders::{Binder, InsertBinder},
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
    // GetUpropsIdenticalToUprop
    pub trait GetBoundsForTblUpropIdenticalToUprop<'a,
        ElemUcompound:'a + UnassignedTblExpressionCompound,
        B:Binder
    > = GetBoundsForUpropIdenticalToUprop<'a,
        UnassignedTblProposition<ElemUcompound>,
        B
    >;

    // GetUpropsEquivalentToUprop
    pub trait GetBoundsForTblUpropsEquivalentToUprop<'a,
        ElemUcompound: 'a + UnassignedTblExpressionCompound,
        B: Binder
    > = GetBoundsForUpropsEquivalentToUprop<'a,
        UnassignedTblProposition<ElemUcompound>,B
    >;
    pub trait GetBoundsForConstructibleTblUpropsEquivalentToUprop<'a,
        MapUcompound: 'a + UnassignedTblExpressionCompound,
        ElemUcompound: 'a + UnassignedTblExpressionCompound,
        B: Binder,
        ElemToMapAssignment: PartialPropositionalAssignment<UnassignedTblProposition<ElemUcompound>, UnassignedTblProposition<MapUcompound>>,
        MapToElemAssignment: PartialPropositionalAssignment<UnassignedTblProposition<MapUcompound>, UnassignedTblProposition<ElemUcompound>>,
    > = GetBoundsForConstructibleUpropsEquivalentToUprop<'a,
        UnassignedTblProposition<MapUcompound>,
        UnassignedTblProposition<ElemUcompound>,
        ElemToMapAssignment,
        MapToElemAssignment,
        B,
    >;

    // GetUpropsSubsumingProp
    pub trait GetBoundsForTblUpropsSubsumingProp<'a,
        ElemCompound: 'a + TblExpressionCompound,
        B: Binder
    > = GetBoundsForUpropsSubsumingProp<'a,
        TblProposition<ElemCompound>,B
    >;
    pub trait GetBoundsForConstructibleTblUpropsSubsumingProp<'a,
        MapUcompound: 'a + UnassignedTblExpressionCompound,
        ElemCompound: 'a + TblExpressionCompound,
        MapToElemAssignment: PropositionalAssignment<UnassignedTblProposition<MapUcompound>, TblProposition<ElemCompound>>,
        B: Binder,
    > = GetBoundsForConstructibleUpropsSubsumingProp<'a,
        UnassignedTblProposition<MapUcompound>,
        TblProposition<ElemCompound>,
        MapToElemAssignment,
        B,
    >;

    // GetUpropsSubsumingUprop
    pub trait GetBoundsForTblUpropsSubsumingUprop<'a,
        ElemUcompound: 'a + UnassignedTblExpressionCompound,
        B: Binder
    > = GetBoundsForUpropsSubsumingUprop<'a,
        UnassignedTblProposition<ElemUcompound>,B
    >;
    pub trait GetBoundsForConstructibleTblUpropsSubsumingUprop<'a,
        MapUcompound: 'a + UnassignedTblExpressionCompound,
        ElemUcompound: 'a + UnassignedTblExpressionCompound,
        MapToElemAssignment: PartialPropositionalAssignment<UnassignedTblProposition<MapUcompound>, UnassignedTblProposition<ElemUcompound>>,
        B: Binder,
    > = GetBoundsForConstructibleUpropsSubsumingUprop<'a,
        UnassignedTblProposition<MapUcompound>,
        UnassignedTblProposition<ElemUcompound>,
        MapToElemAssignment,
        B,
    >;

    // GetUpropsSubsumedByUprop
    pub trait GetBoundsForTblUpropsSubsumedByUprop<'a,
        ElemUcompound: 'a + UnassignedTblExpressionCompound,
        B: Binder,
    > = GetBoundsForUpropsSubsumedByUprop<'a,
        UnassignedTblProposition<ElemUcompound>,B
    >;
    pub trait GetBoundsForConstructibleTblUpropsSubsumedByUprop<'a,
        ElemUcompound: 'a + UnassignedTblExpressionCompound,
        MapUcompound: 'a + UnassignedTblExpressionCompound,
        ElemToMapAssignment: PartialPropositionalAssignment<UnassignedTblProposition<ElemUcompound>, UnassignedTblProposition<MapUcompound>>,
        B: Binder,
    > = GetBoundsForConstructibleUpropsSubsumedByUprop<'a,
        UnassignedTblProposition<MapUcompound>,
        UnassignedTblProposition<ElemUcompound>,
        B,
        ElemToMapAssignment,
    >;

    // InsertUprop
    pub trait InsertBoundsForTblUprop<'elem,
        ElemUcompound: 'elem + UnassignedTblExpressionCompound,
        B: InsertBinder<Self>,
    > = InsertBoundsForUprop<'elem,
        UnassignedTblProposition<ElemUcompound>,
        B
    >;
}
