use proof_calculus::propositions::types::assigned::binding::binders::{
    GetBinderForPropIdenticalToProp, GetBinderForPropsSubsumedByUprop, InsertBinderForProp,
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
    // GetPropIdenticalToProp
    pub trait GetBinderForTblPropIdenticalToProp<
        ElemCompound: TblExpressionCompound
    > = GetBinderForPropIdenticalToProp<
        TblProposition<ElemCompound>
    >;

    // GetPropsSubsumedByUprop
    pub trait GetBinderForTblPropsSubsumedByUprop<
        SubsumerElemUprop: UnassignedTblExpressionCompound
    > = GetBinderForPropsSubsumedByUprop<
        UnassignedTblProposition<SubsumerElemUprop>
    >;

    // InsertProp
    pub trait InsertBinderForTblProp<'elem,
        ElemCompound: 'elem + TblExpressionCompound
    > = InsertBinderForProp<'elem,
        TblProposition<ElemCompound>
    >;
}
