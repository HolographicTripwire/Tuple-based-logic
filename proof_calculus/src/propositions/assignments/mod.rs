use crate::{propositions::types::{assigned::Proposition,unassigned::UnassignedProposition}, utils::traits::combinable::TryCombine};

pub trait PropositionalAssignment<FromUprop: UnassignedProposition,ToProp: Proposition>: TryCombine {
    fn assign_to(&self, uprop: &FromUprop) -> Result<ToProp,()>;
}

pub trait PropositionalAssignmentConstructor<FromUprop: UnassignedProposition, ToProp: Proposition, Assignment: PropositionalAssignment<FromUprop,ToProp>>: Sized {
    type Error;
    fn try_construct(&self, prop: &ToProp) -> Result<Assignment,Self::Error>;
}


pub trait PartialPropositionalAssignment<'assignment,'from,FromUprop: UnassignedProposition, ToUprop: UnassignedProposition>: TryCombine {
    fn assign_to(&'assignment self, uprop: &'from FromUprop) -> ToUprop;
}
pub trait PartialPropositionalAssignmentConstructor<'assignment, 'from, FromUprop: UnassignedProposition, ToUprop: UnassignedProposition, Assignment: PartialPropositionalAssignment<'assignment,'from,FromUprop,ToUprop>>: Sized {
    type Error;
    fn try_construct(&self, prop: &FromUprop) -> Result<Assignment,Self::Error>;
}
