use std::{cmp::Ordering, ops::Deref};

use crate::{normal_forms::{UnassignedDnf, UnassignedDnfClause}, propositions::types::unassigned::UnassignedProposition, utils::{collections::sets::tiny_immutable_ordered_set::TinyImmutableOrderedSet, traits::fast_ord::FastOrd}};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct TinyUnassignedDnf<UP: UnassignedProposition + FastOrd>(TinyImmutableOrderedSet<TinyUnassignedDnfClause<UP>>);
impl <P: UnassignedProposition + FastOrd> UnassignedDnf for TinyUnassignedDnf<P> {
    type UnassignedClause = TinyUnassignedDnfClause<P>;
}
impl <P: UnassignedProposition + FastOrd> TinyUnassignedDnf<P> {
    pub fn new(clauses: Box<[TinyUnassignedDnfClause<P>]>) -> Self {
        Self ( TinyImmutableOrderedSet::from_iter(clauses) )
    }

    pub fn get_clauses(&self) -> &impl IntoIterator<Item=TinyUnassignedDnfClause<P>> { &self.0 }
}
impl <P: UnassignedProposition + FastOrd> IntoIterator for TinyUnassignedDnf<P> {
    type Item = TinyUnassignedDnfClause<P>;
    type IntoIter = std::vec::IntoIter<TinyUnassignedDnfClause<P>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
impl <P: UnassignedProposition + FastOrd> Deref for TinyUnassignedDnf<P> {
    type Target = TinyImmutableOrderedSet<TinyUnassignedDnfClause<P>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl <P: UnassignedProposition + FastOrd> FastOrd for TinyUnassignedDnf<P> {
    #[inline]
    fn fast_cmp(&self, other: &Self) -> Ordering { self.0.fast_cmp(&other.0) }
}

#[derive(Clone,Hash,Debug,PartialEq,Eq)]
pub struct TinyUnassignedDnfClause<P: UnassignedProposition + FastOrd>(TinyImmutableOrderedSet<P>);
impl <UP: UnassignedProposition + FastOrd> UnassignedDnfClause for TinyUnassignedDnfClause<UP> {
    type UnassignedProposition = UP;
}
impl <P: UnassignedProposition + FastOrd> TinyUnassignedDnfClause<P> {
    pub fn new(propositions: Box<[P]>) -> Self { 
        Self(TinyImmutableOrderedSet::from_iter(propositions))
    }
    pub fn get_propositions(&self) -> &impl IntoIterator<Item=P> { &self.0 }
    pub fn into_propositions(self) -> impl IntoIterator<Item=P> { self.0 }
    
    pub fn contains(&self, proposition: &P) -> bool { self.0.contains(proposition) }
    pub fn len(&self) -> usize { self.0.count() }
}
impl <P: UnassignedProposition + FastOrd> IntoIterator for TinyUnassignedDnfClause<P> {
    type Item = P;
    type IntoIter = std::vec::IntoIter<P>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
impl <P: UnassignedProposition + FastOrd> Deref for TinyUnassignedDnfClause<P> {
    type Target = TinyImmutableOrderedSet<P>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl <P: UnassignedProposition + FastOrd> FastOrd for TinyUnassignedDnfClause<P> {
    #[inline]
    fn fast_cmp(&self, other: &Self) -> Ordering { self.0.fast_cmp(&other.0) }
}
