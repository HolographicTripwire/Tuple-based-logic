use std::marker::PhantomData;

use crate::structures::{propositions::Proposition, propsets::{PropSet1O, PropSet2O}};

pub mod implementations;

// pub struct Cnf<P:Proposition,PS:PropSet2O<P>>(PS,PhantomData<P>);
// impl <P: Proposition,PS:PropSet2O<P>> Cnf<P,PS> {
//     pub fn get_clauses(&self) -> impl IntoIterator<Item=&CnfClause<P,PS::I>> {
//         let x = self.0
//             .iter()
//             .map(|x| &CnfClause(x,PhantomData));
//     }
//     pub fn into_clauses(self) -> impl IntoIterator<Item=CnfClause<P,PS::I>> { self.0.into_iter().map(|x| x.into()) }
    
//     pub fn contains(&self, seek: &PS::I) -> bool { self.0.contains(seek) }
//     pub fn len(&self) -> usize { self.0.len() }
// }
// impl <P: Proposition,PS:PropSet2O<P>> From<PS> for Cnf<P,PS> {
//     fn from(value: PS) -> Self { Cnf(value,PhantomData) }
// }

pub struct CnfClause<P:Proposition,PS:PropSet1O<P>>(PS,PhantomData<P>);
impl <P: Proposition,PS:PropSet1O<P>> CnfClause<P,PS> {
    pub fn get_propositions(&self) -> &impl IntoIterator<Item=P> { &self.0 }
    pub fn into_propositions(self) -> impl IntoIterator<Item=P> { self.0 }
    
    pub fn contains(&self, proposition: &P) -> bool { self.0.contains(proposition) }
    pub fn len(&self) -> usize { self.0.len() }
}
impl <P: Proposition,PS:PropSet1O<P>> From<PS> for CnfClause<P,PS> {
    fn from(value: PS) -> Self { CnfClause(value,PhantomData) }
}

pub struct Dnf<P:Proposition,PS:PropSet2O<P>>(PS,PhantomData<P>);
impl <P: Proposition,PS:PropSet2O<P>> Dnf<P,PS> {
    pub fn get_clauses(&self) -> impl IntoIterator<Item=&DnfClause<P,PS::I>> { self.0.iter().map(|c| DnfClause(c,PhantomData)) }
    pub fn into_clauses(self) -> impl IntoIterator<Item=PS::I> { self.0 }
    
    pub fn contains(&self, seek: &PS::I) -> bool { self.0.contains(seek) }
    pub fn len(&self) -> usize { self.0.len() }
}
impl <P: Proposition,PS:PropSet2O<P>> From<PS> for Dnf<P,PS> {
    fn from(value: PS) -> Self { Dnf(value,PhantomData) }
}

pub struct DnfClause<P:Proposition,PS:PropSet1O<P>>(PS,PhantomData<P>);
impl <P: Proposition,PS:PropSet1O<P>> DnfClause<P,PS> {
    pub fn get_propositions(&self) -> &impl IntoIterator<Item=P> { &self.0 }
    pub fn into_propositions(self) -> impl IntoIterator<Item=P> { self.0 }
    
    pub fn contains(&self, proposition: &P) -> bool { self.0.contains(proposition) }
    pub fn len(&self) -> usize { self.0.len() }
}
impl <P: Proposition,PS:PropSet1O<P>> From<PS> for DnfClause<P,PS> {
    fn from(value: PS) -> Self { DnfClause(value,PhantomData) }
}


// struct MapCnfIter<P:Proposition,PS:PropSet1O<P>,I>(I);
// impl<'a, I> Iterator for MapCnfIter<I>
// where
//     I: Iterator<Item = &'a i32>,
// {
//     type Item = NewType;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.next().map(|v| NewType(*v))
//     }
// }

// impl<'a> IntoIterator for &'a Source {
//     type Item = NewType;
//     type IntoIter = MapIter<std::slice::Iter<'a, i32>>;

//     fn into_iter(self) -> Self::IntoIter {
//         MapIter(self.0.iter())
//     }
// }