mod expressions;
mod set;
pub mod tuple_or_error;

pub use expressions::Expression;
pub use set::PropositionSet;

/// Every [Proposition] within Tuple-based Logic is simply an [Expression] whose truth value is to be considered
pub type Proposition = Expression;
