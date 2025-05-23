mod terms;
mod set;
pub mod tuple_or_error;

pub use terms::Term;
pub use set::PropositionSet;

#[derive(Hash,PartialEq,Eq,Debug,Clone)]
pub struct Proposition(pub Term);
