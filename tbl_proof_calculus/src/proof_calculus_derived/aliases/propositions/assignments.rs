use proof_calculus::propositions::assignments::{
    PartialPropositionalAssignment, PropositionalAssignment,
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
    pub trait TblPropositionalAssignment
    <PreAssignmentUcompound: UnassignedTblExpressionCompound, PostAssignmentCompound: TblExpressionCompound>
    = PropositionalAssignment<UnassignedTblProposition<PreAssignmentUcompound>,TblProposition<PostAssignmentCompound>>;
    pub trait PartialTblPropositionalAssignment
    <PreAssignmentUcompound: UnassignedTblExpressionCompound, PostAssignmentUcompound: UnassignedTblExpressionCompound>
    = PartialPropositionalAssignment<UnassignedTblProposition<PreAssignmentUcompound>,UnassignedTblProposition<PostAssignmentUcompound>>;
}
