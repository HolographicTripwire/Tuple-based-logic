use proof_calculus::propositions::assignments::{
    PartialPropositionalAssignment, PartialPropositionalAssignmentConstructor,
    PropositionalAssignment, PropositionalAssignmentConstructor,
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
    pub trait TblPropositionalAssignment<
        PreAssignmentUcompound: UnassignedTblExpressionCompound,
        PostAssignmentCompound: TblExpressionCompound
    > = PropositionalAssignment<
        UnassignedTblProposition<PreAssignmentUcompound>,
        TblProposition<PostAssignmentCompound>
    >;
    pub trait TblPropositionalAssignmentConstructor<
        PreAssignmentUcompound: UnassignedTblExpressionCompound,
        PostAssignmentCompound: TblExpressionCompound,
        Assignment: TblPropositionalAssignment<PreAssignmentUcompound,PostAssignmentCompound>
    > = PropositionalAssignmentConstructor<
        UnassignedTblProposition<PreAssignmentUcompound>,
        TblProposition<PostAssignmentCompound>,
        Assignment,
    >;

    pub trait TblPartialPropositionalAssignment<
        PreAssignmentUcompound: UnassignedTblExpressionCompound,
        PostAssignmentUcompound: UnassignedTblExpressionCompound
    > = PartialPropositionalAssignment<
        UnassignedTblProposition<PreAssignmentUcompound>,
        UnassignedTblProposition<PostAssignmentUcompound>
    >;
    pub trait TblPartialPropositionalAssignmentConstructor<
        PreAssignmentUcompound: UnassignedTblExpressionCompound,
        PostAssignmentUcompound: UnassignedTblExpressionCompound,
        Assignment: TblPartialPropositionalAssignment<PreAssignmentUcompound,PostAssignmentUcompound>
    > = PartialPropositionalAssignmentConstructor<
        UnassignedTblProposition<PreAssignmentUcompound>,
        UnassignedTblProposition<PostAssignmentUcompound>,
        Assignment,
    >;
}
