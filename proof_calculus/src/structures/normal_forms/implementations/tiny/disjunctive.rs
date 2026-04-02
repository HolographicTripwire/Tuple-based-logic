use std::{cmp::Ordering, ops::Deref};

use crate::{structures::{normal_forms::{Dnf, DnfClause}, propositions::Proposition}, utils::{collections::small_immutable_ordered_set::SmallImmutableOrderedSet, traits::fast_ord::FastOrd}};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct TinyDnf<P: Proposition + FastOrd>(SmallImmutableOrderedSet<TinyDnfClause<P>>);
impl <P: Proposition + FastOrd> Dnf<P> for TinyDnf<P> {
    type C = TinyDnfClause<P>;
}
impl <P: Proposition + FastOrd> TinyDnf<P> {
    pub fn new(clauses: Box<[TinyDnfClause<P>]>) -> Self {
        Self ( SmallImmutableOrderedSet::new(clauses) )
    }

    pub fn get_clauses(&self) -> &impl IntoIterator<Item=TinyDnfClause<P>> { &self.0 }
}
impl <P: Proposition + FastOrd> IntoIterator for TinyDnf<P> {
    type Item = TinyDnfClause<P>;
    type IntoIter = std::vec::IntoIter<TinyDnfClause<P>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
impl <P: Proposition + FastOrd> Deref for TinyDnf<P> {
    type Target = SmallImmutableOrderedSet<TinyDnfClause<P>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl <P: Proposition + FastOrd> FastOrd for TinyDnf<P> {
    #[inline]
    fn fast_cmp(&self, other: &Self) -> Ordering { self.0.fast_cmp(&other.0) }
}

#[derive(Clone,Hash,Debug,PartialEq,Eq)]
pub struct TinyDnfClause<P: Proposition + FastOrd>(SmallImmutableOrderedSet<P>);
impl <P: Proposition + FastOrd> DnfClause<P> for TinyDnfClause<P> {}
impl <P: Proposition + FastOrd> TinyDnfClause<P> {
    pub fn new(propositions: Box<[P]>) -> Self { 
        Self(SmallImmutableOrderedSet::new(propositions))
    }
    pub fn get_propositions(&self) -> &impl IntoIterator<Item=P> { &self.0 }
    pub fn into_propositions(self) -> impl IntoIterator<Item=P> { self.0 }
    
    pub fn contains(&self, proposition: &P) -> bool { self.0.contains(proposition) }
    pub fn len(&self) -> usize { self.0.len() }
}
impl <P: Proposition + FastOrd> IntoIterator for TinyDnfClause<P> {
    type Item = P;
    type IntoIter = std::vec::IntoIter<P>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
impl <P: Proposition + FastOrd> Deref for TinyDnfClause<P> {
    type Target = SmallImmutableOrderedSet<P>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl <P: Proposition + FastOrd> FastOrd for TinyDnfClause<P> {
    #[inline]
    fn fast_cmp(&self, other: &Self) -> Ordering { self.0.fast_cmp(&other.0) }
}
