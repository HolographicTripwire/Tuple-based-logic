use crate::propositions::types::unassigned::UnassignedProposition;

pub mod implementations;

pub trait UnassignedDnf: IntoIterator<Item=Self::UnassignedClause> {
    type UnassignedClause: UnassignedDnfClause;
}
pub trait UnassignedDnfClause: IntoIterator<Item=Self::UnassignedProposition> {
    type UnassignedProposition: UnassignedProposition;
}

pub trait UnassignedCnf: IntoIterator<Item=Self::UnassignedClause> {
    type UnassignedClause: UnassignedCnfClause;
}
pub trait UnassignedCnfClause: IntoIterator<Item=Self::UnassignedProposition> {
    type UnassignedProposition: UnassignedProposition;
}
