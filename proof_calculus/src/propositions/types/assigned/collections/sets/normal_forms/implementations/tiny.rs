use crate::structures::{propositions::Proposition, propsets::{implementations::tiny_immutable::{TinyImmutablePropSet1O, TinyImmutablePropSet2O}, normal_forms::{CnfClause, Dnf, DnfClause}}};

type TinyCnf<P:Proposition> = Cnf<P,TinyImmutablePropSet2O<P>>;
type TinyCnfClause<P:Proposition> = CnfClause<P,TinyImmutablePropSet1O<P>>;

type TinyDnf<P:Proposition> = Dnf<P,TinyImmutablePropSet2O<P>>;
type TinyDnfClause<P:Proposition> = DnfClause<P,TinyImmutablePropSet1O<P>>;

// impl <P: Proposition + FastOrd> IntoIterator for TinyCnfClause<P> {
//     type Item = P;
//     type IntoIter = std::vec::IntoIter<P>;

//     #[inline]
//     fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
// }
