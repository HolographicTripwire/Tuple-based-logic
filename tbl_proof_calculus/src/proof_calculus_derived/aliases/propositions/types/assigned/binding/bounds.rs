use proof_calculus::{
    propositions::{
        assignments::PropositionalAssignment,
        types::assigned::binding::bounds::{
            GetBoundsForConstructiblePropsSubsumedByUprop, GetBoundsForPropIdenticalToProp,
            GetBoundsForPropsSubsumedByUprop, InsertBoundsForProp,
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
    // GetPropsIdenticalToProp
    pub trait GetBoundsForTblPropIdenticalToProp<'a,
        ElemCompound:'a + TblExpressionCompound,
        B:Binder
    > = GetBoundsForPropIdenticalToProp<'a,
        TblProposition<ElemCompound>,
        B
    >;

    // GetPropsSubsumedByUprop
    pub trait GetBoundsForTblPropsSubsumedByUprop<'a,
        ElemUcompound:'a + UnassignedTblExpressionCompound,
        B:Binder
    > = GetBoundsForPropsSubsumedByUprop<'a,
        UnassignedTblProposition<ElemUcompound>,
        B
    >;
    pub trait GetBoundsForConstructibleTblPropsSubsumedByUprop<'a,
        MapCompound: 'a + TblExpressionCompound,
        ElemUcompound: 'a + UnassignedTblExpressionCompound,
        ElemToMapAssignment: PropositionalAssignment<UnassignedTblProposition<ElemUcompound>, TblProposition<MapCompound>>,
        B: Binder,
    > = GetBoundsForConstructiblePropsSubsumedByUprop<'a,
        TblProposition<MapCompound>,
        UnassignedTblProposition<ElemUcompound>,
        ElemToMapAssignment,
        B,
    >;

    // InsertProp
    pub trait InsertBoundsForTblProp<'elem,
        ElemCompound: 'elem + TblExpressionCompound,
        B: InsertBinder<Self>,
    > = InsertBoundsForProp<'elem,
        TblProposition<ElemCompound>,
        B
    >;
}
