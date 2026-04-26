use crate::{propositions::assigned::{Proposition,collections::sets::{PropSet1O, PropSet2O}}, utils::{collections::tiny_immutable_ordered_set::TinyImmutableOrderedSet, traits::fast_ord::FastOrd}};

pub type TinyImmutablePropSet1O<P:Proposition+FastOrd> = TinyImmutableOrderedSet<P>;
impl <P: Proposition + FastOrd> PropSet1O<P> for TinyImmutablePropSet1O<P> {
    fn iter<'a>(&'a self) -> impl Iterator<Item=&'a P> where P: 'a { TinyImmutableOrderedSet::iter(self) }
    fn contains(&self,seek: &P) -> bool { self.contains(seek) }
    fn count(&self) -> usize { self.count() }
}
pub type TinyImmutablePropSet2O<P:Proposition+FastOrd> = TinyImmutableOrderedSet<TinyImmutablePropSet1O<P>>;
impl <P: Proposition + FastOrd> PropSet2O<P> for TinyImmutablePropSet2O<P> {
    type I = TinyImmutablePropSet1O<P>;
    fn iter<'a>(&'a self) -> impl Iterator<Item=&Self::I> where P: 'a { TinyImmutableOrderedSet::iter(self) }
    fn contains(&self,seek: &Self::I) -> bool { self.contains(seek) }
    fn count(&self) -> usize { self.count() }
}
