use std::cmp::Ordering;

use crate::utils::traits::fast_ord::{FastOrd, fastcmp_for_sorted_slices};

#[derive(Clone,Hash,Debug,PartialEq,Eq)]
pub struct SmallImmutableOrderedSet<T: FastOrd + Eq>(Box<[T]>);
impl <T: FastOrd + Eq> SmallImmutableOrderedSet<T> {
    pub fn new(mut propositions: Box<[T]>) -> Self { 
        propositions.sort_by(T::fast_cmp);
        Self(propositions)
    }

    pub fn contains(&self, seek: &T) -> bool {
        self.0.binary_search_by(|probe| probe.fast_cmp(seek)).is_ok()
    }
    pub fn len(&self) -> usize { self.0.len() }
}
impl <T: FastOrd + Eq> FastOrd for SmallImmutableOrderedSet<T> {
    #[inline]
    fn fast_cmp(&self, other: &Self) -> Ordering { fastcmp_for_sorted_slices(&self.0, &other.0) }
}
impl <T: FastOrd + Eq> IntoIterator for SmallImmutableOrderedSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

// TODO: Tests!
