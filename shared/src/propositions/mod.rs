mod terms;
mod set;

pub use terms::Term;
pub use set::PropositionSet;

#[derive(Hash,PartialEq,Eq,Debug,Clone)]
pub struct Proposition(pub Term);
