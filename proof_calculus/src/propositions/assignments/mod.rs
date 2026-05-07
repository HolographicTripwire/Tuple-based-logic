use crate::{propositions::types::{assigned::Proposition,unassigned::UnassignedProposition}, utils::traits::combinable::TryCombine};

pub trait PropositionalAssignment<PreAssignmentUprop: UnassignedProposition, PostAssignmentProp: Proposition>: TryCombine {
    type AssignmentError;
    type ReverseAssignmentError;
    fn assign(&self, pre_assignment_uprop: &PreAssignmentUprop) -> Result<PostAssignmentProp,Self::AssignmentError>;
    fn reverse_assign(pre_assignment_uprop: &PreAssignmentUprop, post_assignment_prop: &PostAssignmentProp) -> Result<Self,Self::ReverseAssignmentError>;
}

pub trait PropositionalAssignmentConstructor<PreAssignmentUprop: UnassignedProposition, PostAssignmentProp: Proposition, Assignment: PropositionalAssignment<PreAssignmentUprop,PostAssignmentProp>>: Sized {
    type Error;
    fn try_construct(&self, post_assignment_prop: &PostAssignmentProp) -> Result<Assignment,Self::Error>;
}

pub trait PartialPropositionalAssignment<PreAssignmentUprop: UnassignedProposition,PostAssignmentProp: UnassignedProposition>: TryCombine {
    type AssignmentError;
    type ReverseAssignmentError;
    fn assign(&self, pre_assignment_uprop: &PreAssignmentUprop) -> Result<PostAssignmentProp,Self::AssignmentError>;
    fn reverse_assign(pre_assignment_uprop: &PreAssignmentUprop, post_assignment_prop: &PostAssignmentProp) -> Result<Self,Self::ReverseAssignmentError>;
}
pub trait PartialPropositionalAssignmentConstructor<PreAssignmentUprop: UnassignedProposition,PostAssignmentProp: UnassignedProposition, Assignment: PartialPropositionalAssignment<PreAssignmentUprop,PostAssignmentProp>>: Sized {
    type Error;
    fn try_construct(&self, post_assignment_uprop: &PostAssignmentProp) -> Result<Assignment,Self::Error>;
}
