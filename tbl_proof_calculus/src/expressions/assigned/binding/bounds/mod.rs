mod atom_value;
mod compound_length;
mod value_duplication;

mod identity;
mod insertion;

pub use atom_value::{TblExpressionBoundAtomExactValue,TblPropositionBoundAtomExactValue};
pub use compound_length::{TblExpressionBoundCompoundExactLength,TblPropositionBoundCompoundExactLength};
pub use value_duplication::{TblExpressionBoundValueDuplicated,TblPropositionBoundValueDuplicated};

pub use identity::{TblExpressionIdentityBound,TblPropositionIdentityBound};
pub use insertion::{TblExpressionSubsumptionBound,TblPropositionInsertionBound};
