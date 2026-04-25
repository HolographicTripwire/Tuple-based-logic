mod variable_location;
mod variable_value;

mod identity;
mod equivalence;
mod insertion;

pub use variable_location::{TblExpressionBoundVariableExistsAtLocation,TblPropositionBoundVariableExists};
pub use variable_value::{TblExpressionBoundVariableExactValue,TblPropositionBoundVariableExactValue};

pub use identity::{UnassignedTblExpressionIdentityBound,UnassignedTblPropositionIdentityBound};
pub use equivalence::{UnassignedTblExpressionEquivalenceBound,TblPropositionEquivalenceBound};
pub use insertion::{UnassignedTblExpressionInsertionBound,UnassignedTblPropositionSubsumedBound};
