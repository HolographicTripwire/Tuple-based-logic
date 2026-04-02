use crate::structures::propositions::Proposition;

pub mod implementations;

trait Dnf<P: Proposition>: IntoIterator<Item=Self::C> {
    type C: DnfClause<P>;
}
trait DnfClause<P: Proposition>: IntoIterator<Item=P> {}

trait Cnf<P: Proposition>: IntoIterator<Item=Self::C> {
    type C: CnfClause<P>;
}
trait CnfClause<P: Proposition>: IntoIterator<Item=P> {}
