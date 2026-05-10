use std::ops::Deref;

use crate::{
    propositions::types::unassigned::UnassignedProposition, utils::traits::combinable::TryCombine,
};

pub trait PartialPropositionalAssignment<
    PreAssignmentUprop: UnassignedProposition,
    PostAssignmentProp: UnassignedProposition,
>: TryCombine
{
    type AssignmentError;
    type ReverseAssignmentError;
    fn assign(
        &self,
        pre_assignment_uprop: &PreAssignmentUprop,
    ) -> Result<PostAssignmentProp, Self::AssignmentError>;
    fn reverse_assign(
        pre_assignment_uprop: &PreAssignmentUprop,
        post_assignment_prop: &PostAssignmentProp,
    ) -> Result<Self, Self::ReverseAssignmentError>;
}
pub trait PartialPropositionalAssignmentConstructor<
    PreAssignmentUprop: UnassignedProposition,
    PostAssignmentProp: UnassignedProposition,
    Assignment: PartialPropositionalAssignment<PreAssignmentUprop, PostAssignmentProp>,
>: Sized
{
    type Error;
    fn try_construct(
        &self,
        post_assignment_uprop: &PostAssignmentProp,
    ) -> Result<Assignment, Self::Error>;
}
impl<
    PreAssignmentUprop: UnassignedProposition,
    PostAssignmentUprop: UnassignedProposition,
    Assignment: PartialPropositionalAssignment<PreAssignmentUprop, PostAssignmentUprop>,
    AssignmentConstructor: PartialPropositionalAssignmentConstructor<PreAssignmentUprop, PostAssignmentUprop, Assignment>,
    DerefAssignmentConstructor: Deref<Target = AssignmentConstructor>,
> PartialPropositionalAssignmentConstructor<PreAssignmentUprop, PostAssignmentUprop, Assignment>
    for DerefAssignmentConstructor
{
    type Error = AssignmentConstructor::Error;
    fn try_construct(
        &self,
        post_assignment_prop: &PostAssignmentUprop,
    ) -> Result<Assignment, Self::Error> {
        (**self).try_construct(post_assignment_prop)
    }
}
