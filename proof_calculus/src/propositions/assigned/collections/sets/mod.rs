use crate::propositions::assigned::Proposition;

pub mod implementations;
// pub mod normal_forms;

pub trait PropSet2O<P: Proposition>: Clone + PartialEq + Eq + IntoIterator<Item=Self::I> + FromIterator<Self::I> {
    type I: PropSet1O<P>;
    fn iter<'a>(&'a self) -> impl Iterator<Item=&'a Self::I> where P: 'a;
    fn contains(&self,seek: &Self::I) -> bool;
    fn count(&self) -> usize;
}
pub trait PropSet1O<P: Proposition>: Clone + PartialEq + Eq + IntoIterator<Item=P> + FromIterator<P> {
    fn iter<'a>(&'a self) -> impl Iterator<Item=&'a P> where P: 'a;
    fn contains(&self,seek: &P) -> bool;
    fn count(&self) -> usize;
}
