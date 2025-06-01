mod expressions;
mod set;
pub mod tuple_or_error;

pub use expressions::Expression;
pub use set::PropositionSet;

#[derive(Hash,PartialEq,Eq,Debug,Clone)]
/// A struct representing a single Proposition within Tuple-based logic; that is, an expression we may wish to assign a truth value
pub struct Proposition(pub Expression);
