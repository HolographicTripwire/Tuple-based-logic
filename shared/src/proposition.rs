use crate::term::Term;

#[derive(Hash,PartialEq,Eq,Debug,Clone)]
pub struct Proposition(pub Term);
