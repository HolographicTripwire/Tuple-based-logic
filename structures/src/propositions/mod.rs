mod expressions;
mod set;
pub mod tuple_or_error;

pub use expressions::Expression;
pub use set::PropositionSet;

#[derive(Hash,PartialEq,Eq,Debug,Clone)]
pub struct Proposition(pub Expression);
