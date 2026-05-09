use proof_calculus::propositions::types::assigned::binding::binders::{
    GetBinderForPropIdenticalToProp, GetBinderForPropsSubsumedByUprop,
};
use trait_aliases::trait_aliases;

use crate::expressions::types::{
    assigned::{TblExpression, compound::TblExpressionCompound},
    unassigned::{UnassignedTblExpression, compound::UnassignedTblExpressionCompound},
};

trait_aliases! {
    pub trait GetBinderForTblPropIdenticalToProp<'a, ElemCompound: TblExpressionCompound, B>
    = GetBinderForPropIdenticalToProp<TblExpression<ElemCompound>>;
    pub trait GetBinderForTblPropsSubsumedByUprop<'a, MapCompound: TblExpressionCompound, ElemUcompound: UnassignedTblExpressionCompound>
    = GetBinderForPropsSubsumedByUprop<'a, TblExpression<MapCompound>, UnassignedTblExpression<ElemUcompound>>;
}
