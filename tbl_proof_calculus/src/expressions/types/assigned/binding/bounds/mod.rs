mod atom_value;
mod atom_location;
mod compound_length;
mod compound_location;
mod expr_location;
mod value_duplication;

mod identity;
mod insertion;

pub use atom_location::{TblExpressionBoundAtomExistsAtLocation,TblPropositionBoundAtomExistsAtLocation};
pub use atom_value::{TblExpressionBoundAtomExactValue,TblPropositionBoundAtomExactValue};
pub use compound_location::{TblExpressionBoundCompoundExistsAtLocation,TblPropositionBoundCompoundExistsAtLocation};
pub use compound_length::{TblExpressionBoundCompoundExactLength,TblPropositionBoundCompoundExactLength};
pub use expr_location::{TblExpressionBoundExpressionExistsAtLocation,TblPropositionBoundExpressionExistsAtLocation,AtomOrCompoundLength};
pub use value_duplication::{TblExpressionBoundValueDuplicated,TblPropositionBoundValueDuplicated};

pub use identity::{TblExpressionIdentityBound,TblPropositionIdentityBound};
pub use insertion::{TblExpressionInsertionBound,TblPropositionInsertionBound};
