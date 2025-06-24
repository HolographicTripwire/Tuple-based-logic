mod expressions;
mod set;
mod path;
pub mod tuple_or_error;

pub use expressions::Expression;
pub use set::{PropositionSet,get_contradictions};
pub use path::{SubexpressionPath};

/// Every [Proposition] within Tuple-based Logic is simply an [Expression] whose truth value is to be considered
pub type Proposition = Expression;
