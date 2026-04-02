use crate::structures::propositions::Proposition;

pub mod implementations;

pub trait Dnf<P: Proposition>: IntoIterator<Item=Self::C> {
    type C: DnfClause<P>;
}
pub trait DnfClause<P: Proposition>: IntoIterator<Item=P> {}

pub trait Cnf<P: Proposition>: IntoIterator<Item=Self::C> {
    type C: CnfClause<P>;
}
pub trait CnfClause<P: Proposition>: IntoIterator<Item=P> {}
