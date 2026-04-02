use std::cmp::Ordering;

use itertools::Itertools;

use crate::utils::traits::fast_ord::{FastOrd, fastcmp_for_sorted_slices};

#[derive(Clone,Hash,Debug,PartialEq,Eq)]
pub struct TinyImmutableOrderedSet<T: FastOrd + Eq>(Box<[T]>);
impl <T: FastOrd + Eq> TinyImmutableOrderedSet<T> {
    pub fn iter(&self) -> impl Iterator<Item=&T> { self.0.iter() }
    pub fn contains(&self, seek: &T) -> bool {
        self.0.binary_search_by(|probe| probe.fast_cmp(seek)).is_ok()
    }
    pub fn count(&self) -> usize { self.0.len() }
}
impl <T: FastOrd + Eq> FastOrd for TinyImmutableOrderedSet<T> {
    #[inline]
    fn fast_cmp(&self, other: &Self) -> Ordering { fastcmp_for_sorted_slices(&self.0, &other.0) }
}
impl <T: FastOrd + Eq> IntoIterator for TinyImmutableOrderedSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
impl <T: FastOrd + Eq> FromIterator<T> for TinyImmutableOrderedSet<T> {    
    fn from_iter<I: IntoIterator<Item = T>>(propositions: I) -> Self {
        Self(propositions.into_iter().sorted_by(T::fast_cmp).collect())
    }
}

// TODO: Tests!
