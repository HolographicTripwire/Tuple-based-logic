use std::{cmp::Ordering, ops::Deref};

use crate::{structures::{normal_forms::{Cnf, CnfClause}, propositions::Proposition}, utils::{collections::small_immutable_ordered_set::SmallImmutableOrderedSet, traits::fast_ord::FastOrd}};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct TinyCnf<P: Proposition + FastOrd>(SmallImmutableOrderedSet<TinyCnfClause<P>>);
impl <P: Proposition + FastOrd> Cnf<P> for TinyCnf<P> {
    type C = TinyCnfClause<P>;
}
impl <P: Proposition + FastOrd> TinyCnf<P> {
    pub fn new(clauses: Box<[TinyCnfClause<P>]>) -> Self {
        Self ( SmallImmutableOrderedSet::new(clauses) )
    }

    pub fn get_clauses(&self) -> &impl IntoIterator<Item=TinyCnfClause<P>> { &self.0 }
}
impl <P: Proposition + FastOrd> IntoIterator for TinyCnf<P> {
    type Item = TinyCnfClause<P>;
    type IntoIter = std::vec::IntoIter<TinyCnfClause<P>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
impl <P: Proposition + FastOrd> Deref for TinyCnf<P> {
    type Target = SmallImmutableOrderedSet<TinyCnfClause<P>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl <P: Proposition + FastOrd> FastOrd for TinyCnf<P> {
    #[inline]
    fn fast_cmp(&self, other: &Self) -> Ordering { self.0.fast_cmp(&other.0) }
}

#[derive(Clone,Hash,Debug,PartialEq,Eq)]
pub struct TinyCnfClause<P: Proposition + FastOrd>(SmallImmutableOrderedSet<P>);
impl <P: Proposition + FastOrd> CnfClause<P> for TinyCnfClause<P> {}
impl <P: Proposition + FastOrd> TinyCnfClause<P> {
    pub fn new(propositions: Box<[P]>) -> Self { 
        Self(SmallImmutableOrderedSet::new(propositions))
    }
    pub fn get_propositions(&self) -> &impl IntoIterator<Item=P> { &self.0 }
    pub fn into_propositions(self) -> impl IntoIterator<Item=P> { self.0 }
    
    pub fn contains(&self, proposition: &P) -> bool { self.0.contains(proposition) }
    pub fn len(&self) -> usize { self.0.len() }
}
impl <P: Proposition + FastOrd> IntoIterator for TinyCnfClause<P> {
    type Item = P;
    type IntoIter = std::vec::IntoIter<P>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
impl <P: Proposition + FastOrd> Deref for TinyCnfClause<P> {
    type Target = SmallImmutableOrderedSet<P>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl <P: Proposition + FastOrd> FastOrd for TinyCnfClause<P> {
    #[inline]
    fn fast_cmp(&self, other: &Self) -> Ordering { self.0.fast_cmp(&other.0) }
}
