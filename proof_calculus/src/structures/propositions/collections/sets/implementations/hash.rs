use std::{collections::HashSet, hash::Hash};

use crate::structures::{propositions::{Proposition, collections::sets::PropSet1O}};

pub type HashPropSet1O<P:Proposition+Hash> = HashSet<P>;
impl <P: Proposition + Hash> PropSet1O<P> for HashPropSet1O<P> {
    fn iter<'a>(&'a self) -> impl Iterator<Item=&'a P> where P: 'a { HashSet::iter(self) }
    fn contains(&self,seek: &P) -> bool { HashSet::contains(self,seek) }
    fn count(&self) -> usize { self.len() }
}

// pub type HashPropSet2O<P:Proposition+Hash> = HashSet<HashPropSet1O<P>>;
// impl <P: Proposition + Hash> PropSet2O<P> for HashPropSet2O<P> {
//     type I = HashPropSet1O<P>;
//     fn iter(&self) -> impl Iterator<Item=&Self::I> { HashSet::iter(self) }
//     fn contains(&self,seek: &Self::I) -> bool { HashSet::contains(self,seek) }
//     fn count(&self) -> usize { self.len() }
// }
